#!/usr/bin/env bash

# Generates the crate's test fixtures (tests/data, and the inline ones in src/should.rs) with
# the bb release the verifier targets:
#   tests/data/{zk,plain}/{vk,proof,pubs}             assert_statement (x != y, y = 1 public)
#   tests/data/recursive/valid/{zk,plain}/...         verify_honk_proof: a proof aggregating an
#                                                     inner UltraHonk proof of assert_statement
#   tests/data/recursive/invalid_inner/{zk,plain}/... the same, the inner proof's KZG opening
#                                                     commitment negated: every in-circuit check
#                                                     still holds, only the deferred pairing fails
#   tests/data/rom/{zk,plain}/...                     rom_read: reads at witness indices, so the
#                                                     ROM-LogUp relation (bb v6.0.0) has live rows
# The recursion and ROM programs are barretenberg's own (acir_tests/internal_test_programs and
# noir/bb_proof_verification), taken from an aztec-packages checkout. Every proof except the
# invalid ones is checked with `bb verify`. Stock bb refuses to write a proof that fails its own
# check, so the invalid-inner proofs are made with BB_UNCHECKED, the same release built without
# that check (scripts/build_bb_unchecked.sh); stock bb then confirms it rejects them.
#
#   BB=bb-6.0.0 BB_UNCHECKED=bb-unchecked/bin/bb NARGO=nargo-1.0.0-rc.3 \
#     AZTEC_PACKAGES=../aztec-packages scripts/generate_test_fixtures.sh

set -euo pipefail

NARGO=${NARGO:-nargo}
BB=${BB:-bb}
BB_UNCHECKED=${BB_UNCHECKED:?set BB_UNCHECKED to a bb built by scripts/build_bb_unchecked.sh}
AZTEC_PACKAGES=$(realpath "${AZTEC_PACKAGES:?set AZTEC_PACKAGES to an aztec-packages checkout}")
BBDIR=$AZTEC_PACKAGES/barretenberg
ROOT=$(cd "$(dirname "$0")/.." && pwd)
DATA=$ROOT/tests/data
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

echo "Using $("$NARGO" --version | head -1), bb $("$BB" --version)"

run_bb() { # quiet; on failure, the command and bb's output
  local log=$WORK/bb.log
  if ! "$BB" "$@" > "$log" 2>&1; then echo "bb $* failed:" >&2; cat "$log" >&2; exit 1; fi
}

program() { # <name> <main.nr> <Prover.toml> [dependency line]: a Noir package in $WORK, executed
  local dir=$WORK/$1
  mkdir -p "$dir/src"
  printf '[package]\nname = "%s"\ntype = "bin"\nauthors = [""]\n\n[dependencies]\n%s\n' "$1" "${4:-}" > "$dir/Nargo.toml"
  cp "$2" "$dir/src/main.nr"
  cp "$3" "$dir/Prover.toml"
  (cd "$dir" && "$NARGO" execute --silence-warnings > /dev/null)
}

fixture() { # <package> <dest> <zk|plain> [verify]: write_vk, prove, (verify), copy vk/proof/pubs
  local dir=$WORK/$1 out=$WORK/$1/out-$3 target
  [ "$3" = zk ] && target=evm || target=evm-no-zk
  rm -rf "$out"; mkdir -p "$out" "$2"
  run_bb write_vk -t "$target" -b "$dir/target/$1.json" -o "$out"
  run_bb prove -t "$target" -b "$dir/target/$1.json" -w "$dir/target/$1.gz" -k "$out/vk" -o "$out"
  if [ "${4:-verify}" = verify ]; then
    run_bb verify -t "$target" -p "$out/proof" -i "$out/public_inputs" -k "$out/vk"
  fi
  cp "$out/vk" "$2/vk"; cp "$out/proof" "$2/proof"; cp "$out/public_inputs" "$2/pubs"
  echo "  $2: proof $(wc -c < "$2/proof" | tr -d ' ') B, ${4:-verify}"
}

cat > "$WORK/assert_statement.nr" <<'EOF'
fn main(x: Field, y: pub Field) {
    assert(x != y, "x and y must be different");
}
EOF

echo "== assert_statement"
printf 'x = "2"\ny = "1"\n' > "$WORK/one.toml"
program assert_statement "$WORK/assert_statement.nr" "$WORK/one.toml"
for f in zk plain; do fixture assert_statement "$DATA/$f" $f; done

echo "== rom_read"
program rom_read "$BBDIR/acir_tests/internal_test_programs/rom_read/src/main.nr" \
  "$BBDIR/acir_tests/internal_test_programs/rom_read/Prover.toml"
for f in zk plain; do fixture rom_read "$DATA/rom/$f" $f; done

echo "== the inner proof (assert_statement, y = 2, noir-recursive-no-zk)"
printf 'x = "1"\ny = "2"\n' > "$WORK/two.toml"
program inner "$WORK/assert_statement.nr" "$WORK/two.toml"
run_bb prove -t noir-recursive-no-zk --write_vk --output_format json -o "$WORK/inner/out" \
  -b "$WORK/inner/target/inner.json" -w "$WORK/inner/target/inner.gz"

# Prover.toml for verify_honk_proof (barretenberg/acir_tests/bootstrap.sh's generate_toml)
python3 - "$WORK/inner/out" "$WORK" <<'PY'
import json, sys
out, work = sys.argv[1], sys.argv[2]
vk = json.load(open(f"{out}/vk.json")); proof = json.load(open(f"{out}/proof.json"))["proof"]
pubs = json.load(open(f"{out}/public_inputs.json"))["public_inputs"]
q = 21888242871839275222246405745257275088696311157297823662689037894645226208583
def toml(p):
    arr = lambda xs: "[" + ", ".join(f'"{x}"' for x in xs) + "]"
    return "\n".join([f'key_hash = "{vk["hash"]}"', f"proof = {arr(p)}",
                      f"public_inputs = {arr(pubs)}", f"verification_key = {arr(vk['vk'])}", ""])
open(f"{work}/valid.toml", "w").write(toml(proof))
# The proof ends with the KZG opening commitment, x then y, each as two limbs (lo 136 bits, hi
# the rest). Negating y keeps it on the curve and changes only what the pairing checks.
lo, hi = int(proof[-2], 16), int(proof[-1], 16)
y = (hi << 136) | lo
assert y < q, "the last two fields are not a coordinate"
ny = q - y
bad = proof[:-2] + [hex(ny & ((1 << 136) - 1)), hex(ny >> 136)]
open(f"{work}/invalid.toml", "w").write(toml(bad))
PY

DEP="bb_proof_verification = { path = \"$BBDIR/noir/bb_proof_verification\" }"
echo "== verify_honk_proof, valid inner"
program recursive_valid "$BBDIR/acir_tests/internal_test_programs/verify_honk_proof/src/main.nr" "$WORK/valid.toml" "$DEP"
for f in zk plain; do fixture recursive_valid "$DATA/recursive/valid/$f" $f; done

echo "== verify_honk_proof, inner KZG commitment negated"
program recursive_invalid "$BBDIR/acir_tests/internal_test_programs/verify_honk_proof/src/main.nr" "$WORK/invalid.toml" "$DEP"
for f in zk plain; do
  BB=$BB_UNCHECKED fixture recursive_invalid "$DATA/recursive/invalid_inner/$f" $f no-verify
  if "$BB" verify -t "$([ $f = zk ] && echo evm || echo evm-no-zk)" -p "$DATA/recursive/invalid_inner/$f/proof" \
       -i "$DATA/recursive/invalid_inner/$f/pubs" -k "$DATA/recursive/invalid_inner/$f/vk" > /dev/null 2>&1; then
    echo "  bb verify accepted the invalid-inner $f proof" >&2; exit 1
  fi
  echo "  bb verify rejects it, as it should"
done
