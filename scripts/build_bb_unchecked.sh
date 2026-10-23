#!/usr/bin/env bash

# Builds Barretenberg 6.0.0's `bb` without its post-prove self-check, for the invalid-inner
# recursive fixtures only. Stock bb verifies every proof it makes and refuses to write one that
# fails (bbapi/bbapi_ultra_honk.cpp, "Failed to verify the generated proof!"), so it cannot
# produce a proof whose inner pairing check fails, which is the case those fixtures exist to
# test. Never use this bb for anything else.
#
#   scripts/build_bb_unchecked.sh <aztec-packages checkout at v6.0.0> <out dir>
# Needs cmake, ninja, clang and node (22 or later). The sparse checkout needs barretenberg/cpp/,
# ipc-runtime/cpp/ (bb's entry point links its IPC server, so BB_LITE cannot be used),
# protocol/constants-codegen/ and noir-projects/fnd/noir-protocol-circuits/crates/types/src/
# constants.nr: bb includes aztec_constants.hpp, which the monorepo generates at configure time
# (barretenberg/cpp/CMakeLists.txt); here it is generated the same way, unformatted.

set -euo pipefail
SRC=$(realpath "$1")/barretenberg/cpp
mkdir -p "$2"; OUT=$(cd "$2" && pwd)
F=$SRC/src/barretenberg/bbapi/bbapi_ultra_honk.cpp
if grep -q 'throw_or_abort("Failed to verify the generated proof!");' "$F"; then
  # comment out the throw; the surrounding `if` then does nothing
  sed -i.orig 's|throw_or_abort("Failed to verify the generated proof!");|/* self-check off (invalid-inner fixtures) */;|' "$F"
fi
grep -q "self-check off" "$F" || { echo "the patch did not apply to $F" >&2; exit 1; }
HDR=$SRC/src/barretenberg/aztec/aztec_constants.hpp
ROOT=$(cd "$SRC/../.." && pwd)
if [ ! -f "$HDR" ]; then
  "$ROOT/protocol/constants-codegen/scripts/generate.sh" --cpp "$HDR" --selection "$SRC/scripts/constants-codegen/cpp.json" > "$OUT.constants.log" 2>&1
fi
# With the codegen present, CMake regenerates the header itself (remake-constants.sh), which
# needs the checkout's git root as its working directory and clang-format-20. With the header
# provided and the codegen moved aside, CMake uses the header ("a checkout without it must
# provide the header by other means", CMakeLists.txt).
[ -d "$ROOT/protocol/constants-codegen" ] && mv "$ROOT/protocol/constants-codegen" "$ROOT/protocol/.constants-codegen.used"
cmake -S "$SRC" -B "$OUT" -G Ninja -DCMAKE_BUILD_TYPE=Release -DAVM=OFF -DBB_LITE=OFF \
  -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ > "$OUT.configure.log" 2>&1 \
  || { tail -30 "$OUT.configure.log" >&2; exit 1; }
ninja -C "$OUT" -j "${JOBS:-6}" bb
"$OUT/bin/bb" --version
