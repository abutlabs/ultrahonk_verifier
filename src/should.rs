// Copyright 2022 Aztec
// Copyright 2025 Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0 or MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    constants::{EVM_WORD_SIZE, PAIRING_POINTS_SIZE},
    *,
};
use alloc::{boxed::Box, vec::Vec};
use rstest::{fixture, rstest};

#[fixture]
fn valid_zk_proof() -> Box<[u8]> {
    Box::new(hex_literal::hex!(
        "
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        2c49c713bfde5cc7a3e0b9499745bce14749ad245b118afcaf3628fa73a692ca
        13fb01a0b48eeda8d82c1370a64045659afb1d6b6da6e4df66d86f1005ded4fe
        043a650fe2d6cf547c34b773c59a8517c6fcdf22101580a52a9ebc6a7dc6573c
        0dd61d792bee026bdb81060c4085e3d2a41d2f5f60ca07b8117c0ec40683cd80
        1e22694c67110c628e1a6c58b8dcde1f4696b53cd3fc269e1251408b00767669
        0a16ef82d7982d80ac937536d09e56e0936a0dfb791f75c61546b071b27f5c09
        0864982d59d522409ee7c38d8f665f477709cbd1337e8a0c2398716641b49697
        14f1f5875de0492defbd2f85f731e03199448c2e3df0d8128dd114f38ed442c8
        28e733171d2b4f294b8f9d57af7ee617b4d8d58d58fdba44cbb3ec4d9d98380a
        0d50e33d3f4146e5c05a8ab3e43604483a5fb8c078924c787f3113286bd2de98
        0e292a7729c2dc0989fa57971e66ab30c4cdf264082f2f7c170157d7952a5f70
        22c90dd9fe4ca973da4d1090d0957ab2e7f95477eef204a5c00ecff2b248b2b0
        1a66594dd8e9580686f54200114f301ccf03ffa31b3a43d8ebadd5cd3563dd53
        1f654c49c5269250956db1ba95bce5bb0f499349bcc8e7da6aebe961b4afdd00
        02459de7383a800dfcc7b52d042803d596715c447f3b689d96b860b20e3c594d
        21e6084a4c9585d6d86869e85179ed244b2d871e290e6d0bde5572f870eea1fa
        07081dde7d75292904639050bb5f75985e43c163bbc17f33221089dfdca43d2a
        23c0194c14a31658cd760888be5f19f0dd5edaee6b09174543cfdebf4cbacdb4
        253a8be073383e6edf354bbaec9c3edede51d73ae5c17e55a6a13fd260128a8d
        17e62215596dd8b67dcbd556bfefd1ff3a59d1ccc130dd401ff9f0e7304cced6
        112809aa70cbd90618c3b99fc181191ea6dfa3f898ce7b82a5e376aaafce8991
        17e84b4e7fb35c05b0b0849d5df71e8080282b47b999b4bb4d3b881815fd669f
        141f0492909fed8526ec1d583fb82bd6c6ad8b1710e88746189307184b106e02
        040a42ffff7e1da0183e9c2a7edf71301e0eaae4e57e930d40e531fe33d8357c
        01cdb45902f04e668395a08b9c95d2cd5c13271cb6d05d12b6dc4939a742c6c7
        121b9aaac80e9a7720d10cc4cb4e2333abc3828eb15ebe18fca8b4110b78c72c
        2595b5034202e31996b7f3e9fc911c0aaacb7d170f7dae1382c42aa9bd1aba6e
        163e6b29cc23d7b9512f685d3906cea2df17210d3bc511ba24ed8b73730fe615
        1035c302bb9aa99d362c4b894d358e3308f6d814f6a0b3a03f5e8ce38840cb7a
        2b5a294d371001fc4c05b56280441a25e15c53713d9d815264d96dee7ca8d2fb
        173b285320a361136a1f39cff034789616151a514718dbb8aacc76f00a5fc22e
        1e6f3382fc92958efaa0c9cbdaab665692a69b6a29962ba4e951791e02525a5e
        10645996118e658080d14ee2f103c0686af7ef9014e44acc2ef8bfaf99df14ea
        17057b2a649969a1519c64f812ef1404b583a47d12212e7b092fe0a1285407ff
        005c07e5d7327889c0d74a2b05fd0e3bdc46abfdf9607bafb5c1090ae698cdc2
        0deb0f35ffa11366f1adf427850017201d6a18eed64c7e6691dd32bc56bc427d
        073ae3fac8da9eafe00949d33a5c63a6afa57662f8bcbd111b71da89a3affed7
        24ca1355afff7779d73a448344d0114e17dd7e0fe63217bdf736204f8b115be2
        24679b149814d67067cc1c0732c95d15c77f5e244c88854834cb6d5487ab52fe
        26c08927b6a7957a7cddb955c81fb355b8c2ee88f6052f12f8db68fde89cffc4
        294bde2bcaa78edc4bdc5325c189c1d523b2cdb7f829c27a5de2a956c1491aa9
        0ad8d2852d98379155aa9a09b1333c063bf219f3b71619d723668d2beee7701c
        2b569baed62a87f66ae821f4e5374951bdd53852d0a07354984b21bfb316301d
        04f29d6e0b5ed05726415094375a8901a414be958c5e648edf84ac38a6ba4367
        3045a5cbfa555308f1af6f1a610830f1b2c6ed2d24e3179a5b665c0b4d80ddde
        014c437ea00e372c4c679086de5ffdbd065bac3954701b08ddd0dcdcf12bb14f
        1237a63d3866c6cc7d805ee983177593811c32c9467842794d29277110ec07e7
        166650c8bfc621beb6c33e588060d0a668340154381b0d69b4a470bed338c9cf
        0a4286cc4874abd3272027d5999fa6ff66330a4bd5e002c0ad8a379fdd798467
        21e50185282a6c6c4ae793d5204c9ad49bfbf768bbefcfe2e9f903b6e7f7c815
        27368416397259d8ef934c18884f2da309272112a1424096502dc4b2bc4b6bee
        27a5506d5d3fba46e6d62228b5c453f20d390c1ce8f3cbca2c7e16963de33ef0
        093b54135e7e251bf046b877616b34ae59b28fe51c3c5e8d28537c6746502fc5
        05235f24f76b184b43bfdd51f0e3ea15743f505f21296350250e79a7b437faa7
        1d8cc4fa66f200e0b509200783740d6590c989f32273171c0877b058d6871c3b
        107b3e26e33f8b8df87dad2e01b18eab3b62e3e816ba652e300cbc0866b9547d
        1866b86b17f6a6e1272a64c640707988944e3efc475a57ee69af8831d64ab4b8
        2e911f1fe72efc6cc80fc9ed9726b7f2ad3e7acb7c4d496629cca1f79cec39c3
        2abd996842438f31f38f4f2a18b5e56cb003cb80d55d47545ffcbba1c57e7075
        09a19a4186040c7039fc5e332d9407f023d907da3838302556f7a475fb3d54b0
        189de940ba602a7ca7883ab813a185626b6279d7dd64033b136c57f3bf876192
        231e608fbbcc066f776df5d31e41e393f58d7c941541bdf5ddce9063fed1d562
        07ee03737601d0195ed0934e5f4e4722ab40f9d7a1b1c7a3f8ec1b4fe6e2ae92
        16cddc5cbc36ae46848c538770313884f3cdd040d269835ea1e996331a84e1a7
        0649677a35595f78b5ee415f9c70a219b2d0364158a12efd5b387d6f85adc403
        0645d924950438ed2b7b6c8f0185cf6b0708129f59cd9bc78d63e29abcfe3bea
        112533cb071a6c1bb72bfec2e58e9b95ec04ee5e0deacb356c368d1fde23752f
        28bc8f253f3091bd9f0bfaf41e083dc578912cdf7088568a5c1c390a95c7402b
        1d457ae616e2ecbf225800f304a13553449fbaf289293b95754868066a094fe5
        1fdd2eb2c2ba67f0d1f8a9260925ab30e2b2f2e0dd54a9f373d4132923b1b6dc
        24cfb2e0dbf918edc2e71cca0441cff61d7d2d5645f4e3e68106fb62808fa217
        2a607a2e4ab1f7fa17f9423d460135148b2c74aff2344be947e51612dcba41b9
        292d04062ed6ba84e8247133f83c88026845812b941e626dadf1625b7e5a5528
        2b8b2d2ac109dd49ad25b1007b7cc3f0358d52797aec800a408245b6c516786b
        10de950f9890bf3d7ec099b0c730cfe017d244f3fcceb50d8b59218bb91028f9
        1608264b20d646d696baa8e8ab3ab7165060d0e404dbe22d734c026f6313f55e
        295222fa2af2c7b58396451d2d849b0352a11a05087def9fe86f92601abe5f57
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        1a851c7c9fd2b5dc08de75bbf5761d798ca53b50658facace112eeb1e710d80a
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        170dcbd9a54fd4bc6c9dcb409cb5ec6045b2d86181d40fdc130bdb504a517828
        1bd2f3276a34ffc5fc0e8c949eac6f10ebdaa075dec16405fde85c8e0ca82fcc
        183c65bd0940bf2b29c9c686ad609ddcefd703caef7e5bd3c348afbe8bb3c859
        05df4429f786374bb1597543396150d098d1d4306554590b5869ebc2dc517724
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        034ce2ba706b03200a8f82dfd8fe7d6d6ec4ef04b7b46d7850a10a697b976bd0
        1db921b136dfafa28cff2709570e5310c36303ad4d3d274cc7b0ed09d7a5c548
        25f1a3ceb9730027524a6ba417e32b1db2ec2b8930d9632ef55fe20b9800b99a
        29e6447c1cf322464b114fd8b958bc37766d339fc37b54f69bbcb56f40631df1
        15c35202d93a3a3f88c25160e8ab2d73d4a7181b0b70b78bec90ad2345cd3040
        13150ac1610c47b9efe8cbbb78b9cdaa665aaf0710a886c24a2b3d39f6bfc536
        160d4b4005b5fe5403eea2c5fc5704c5af0fb306fb9ac41c72e71b2251e40dba
        29412a6c8a5797c810078c63ed67d756c6091dbaea7832e477633b3914e213fe
        2f72beef14e8de308107647498246289f3c1d5010f3acce35ab9fed897db7c57
        268d65549e6525dbe11bf5aae1133fecaeafdfe6ee43baba8524d3282cdcb4cd
        1d0e4395296008ec261b9fbcccc7b19a1aadd6cd2e6452dbcec9881d9e125148
        1b4cb2f59da5fd70428f83f9ffe91570b38af364f0ee31935d2c1517605643e5
        21b54afb032cea361539cbf7302fc60d9b99668e0f6c602d84dc16778c39526c
        2fd785bcc3e3699a072b1ceaa1f2fe6bc6557a4d3bee0af900c6e4838a49c273
        0ef70c71429982db8425f81265ffee2454257ff56566a5673d382eb5ff20f1f6
        19c7c55d11b161064ae39f014037201c337e6a6a7d0998f25fee6f0a2eb6e04c
        1b75b5d89204363efcd54706b584ec90eef1f790970457b6040d75458c7d4a59
        25afe4b9f0e030937ef12bf794c9fbc1639fb9ceb9828315ab8b19b02ba79355
        20151b255d8e40a7bc9b1fc57f1bb8df8b1daa775a21fff25c470313df75140e
        17d0b177a9b86c0f656c6f70a9967716b7a82b780b1273652857d6eaf0c68434
        121285fd514f05fed1300dd28b62968f754c208b0801e7d10106c5b826991899
        054a0139ce7a55be0f6d998ec2c7be507681d6148ffdb67ba1e2bc4f2350c186
        18dc310b647585162f60ed394178b635354c359a442bc42b7ce4e1c7ce3d6a5b
        148bae5845575ac32434cb13f846a90183598169481ae4e25dbff1938a62d696
        07818068e0c142830519517785f6b63f575b74ea96d1aea1158e5f79111fdcdb
        224737ede7dded84445744212f1f8d34a41c2ededf57e5ffb439455449b0fbec
        1d29ef20f52b02e0eff141700d955d480b274a62af9bf50e22845dd58b6dea6d
        0de155c4ae339c54af3b1b30db48c69f8e80a81cc7eafea199e2adcc117244ac
        262286a20101491422c18d9a8f558cd847e42ec8ee527667aab3766095e05251
        139fb2fe0d3bff9c225df5d01934b47c313a0b073f973878f299a7aea0dadea8
        0e5ef7c953953b1e29af89a51b6e9bd82b82b9e4db1d24e1e2dcaeccf71a00ff
        1e68cb61a781fc5a27b967ee7ca5f7734a9a38904332e507a558c65af15c0cf5
        2e9bad253c3e6162c7ca81c1fb2844477e3b7037f1185fafaa1f164e6bac8772
        20ce448d98003834aec386bedb9644d2949a502f2bb2f4d48bd9e8326e26d9d5
        1f0b5a5ca421bee198bb96b41d2d108dde4b569b347f1e7cb56a1b2ed82f408e
        277d9cc10caedc687efd40f2945963e6b3e4cd3d8c88e5e74337778cb4458dc9
        222107e1906c12863ec83afd08c84dc08a5765839eeba8416558890f8a775894
        010d3a5fb2630540083798a5c64d0a5d2b8e561806146666ab23ba11cf61a58d
        09fcef10b0682a6e8122437731e3d6ee538d51c4268b90424b7ec4b6d32dbce5
        "
    ))
}

#[fixture]
fn valid_plain_proof() -> Box<[u8]> {
    Box::new(hex_literal::hex!(
        "
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        25490292cdd1cf424a385c1a455f3ec80b1094041e41b23ae25a78cd79b1e314
        009efa04e57f5c50af8c61b004bf205c3ac12e55ac0774903321f723058d5f4b
        0c279ec05f041e6ad6fd5cec474a5b17435defbd9e0c3bb1e605d9665bf4404b
        2b82ec7ad9cbd6fbb3c39a8266be18df1f311aca9257cb212a1d1fe1a4cb140b
        00afd6bfebf2114039428d5b8c67ffd4ee50f3824ead01cfa6cf25af49863040
        0a846d319f8c8f7504036c40779fb6da198d6169fcfbd733f3e2682d77408fb3
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        2d2f818a008edb4d79cecad1dd495e0878451fcbec4333fbaed0bd195bf43856
        12867b5399bf3f0486fd8fac9c7fa640af1e6b2ec7a54456a85ac08f5d6b82c5
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0450fc4d1e075b7ab3abd883ac19b326f5804fd98bbf6ce40d893cd9031c34f6
        1891db6da9bb87a0fd9b5d542161423196af9b1c0e4485a924c8286507f8b232
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        1ccb7cf4f8625a2906c98eb4c139ec382ddbe0068362f346d71aac59669939d0
        027eada8387a32dabc8967854d17d09f081679abb10ec0a9dff9642c2667a1b1
        0ad8b97ca1cb362917bfa919e6498af502ac96bb41ef78abc6075d29029cac97
        21d551f0b88bc2a523195ad0b38b14934578f57b4fee6d973212d349ee8f41e7
        091444ef916fe7bf63d919fb41c31bcd4ed1688a9b76396133521b2f9daa2137
        08197fb236c516f3473d925455f33313b15988ebe6e47e7d1162cb4093b8bc0b
        2f6e38edfb8968689418ffe6c0093c9d2c1b62ba5f133520cab36fd38ded1641
        14b4dc33d5b945c59f18387e9e43b420ac198dcc2c8a22cb5f3518b7749e04f9
        2d9ef613eae66dd1ef6146247d94eadb7e253f4db312851deaedba0bda680194
        2bab5a7e683eac1c34652376f9eec8ff7c19dfb25a571a8b0cd5e45a12914338
        297e60dea721d8408c3761bfa267caed16f749d5407ae10fb04373cfc24cee99
        1ed7014369e3e40f1815fc22c52533762e757e1bb3c87b815ff399b93a857507
        1dae16834b5a65935cb9c5c3078f3e7d5543e9cf9e609be3eddcfd352f616da9
        05c1a03938da3bd8643d8adb4045e9656888e8b68fd55ecd949c3c163d82e8de
        11797379aba68cfb11c69a46059dcf6745ebb5f495376e2cf14985b1d734da4a
        09c03beda03fd7f2095e79336b10bebab5e95a7ace18dcd5fe70cfa060e45c99
        200c5da61af03cb7d86ed5f7f24b67f8d301fc1650336180d5491a7a85e21b0d
        18f7b47de4efa23f343f38c234b1d079054863226e4344e9eb89cbec91059dcc
        16ad0f905917d6429c07c4902ffed1f36e09b7ff96ae836967ffaad00ac1350e
        24808301397e06d054e0fc0ba360cfb353c83750b2ba6822d79c3d204bc8f35c
        17c546e32b89f00d7ce0afc511ddcfce0ff53d62559574274d821bef72420809
        26a9c0229cedfe83a604a13a6d01a918ced2794caea872ac75d22883997a79e7
        21188a11a4617fe20a3b71aac8753a4fa109e67aa4dc97dfbc72e9630960d103
        095b52b394e2170e4996ecf4dc83f594c95f54330b200b58a6ec2ea0852408c4
        2b59b99b3de4b7cf28d20ae78189363320fe7b88353461524ca2826b1de77518
        2f715718a7c16ba84fcf13ee9d1b402fab6b08716bf746986ff815896c2a9ab2
        09ad0a3811ca2f77cb836940973c8a83da257bdced24d0a73e2f6906da7a30e1
        125e36ffc8050e5199707a08ccb07b9e83a6ea9b3ff7e52f91f1eaf443a5e622
        11362ece5d7c0b6cfe050b12758cb194bfbe4a834e6be40d40a63bdfaf3710d7
        10c05b947d0b3d95729ae6a5969b03780459ea0e64fd2c326cd380ae3c284c32
        1ca7a256c4a3e0da981fa8e683cf79714ddd953c6078c1c29dc9d37217b1718f
        25b6d16dd44e41dbb8fb191ed5a059b49e6cbebeab0d3f11a2965a4ec8d53063
        15b0eed37b978659a09c45f4706111eaeae34ad18997b4ca1190e99ab674a53b
        0d7ac79f9bfe83d05b4d3172996f1a47c4aa377cafa96ce215189f0e02f0cec1
        145bbe6b4405afa8b2e5b1d93dd0fb8f522133268caee22e89612a1d58adfe35
        21f87ca525b154b2394965087319cdebf5e92a8d4694141bb0ecc4589b243ae4
        17c6dca3e4f50ec1edf1919fb6cc860fa67d86276be71b907ca789168496553b
        0ac3a95c0073ccc8ef23813def41c15b4683e77d50ebe21eef681795106fa978
        18a757af5a5948721a97d7eb744525a656c3d290f0af3b4a1da681de8693bec5
        11be234e61c59b4f1947fe228b3e92b8ed722e16759708b5f035ca1e6e59758e
        26addc329dbc4105a472572771f343f899f281df0f9718cbb6f0b95da2a55b08
        2c546d30082264c3bb1febc7cb390d02c8544103f9b69ec0ec7119a94f8a0f49
        2e820b831c30a2926211127a8f971309a693f6732702773d3eda2e5c1a7708d1
        063b74d317d36915f32e88625aa8907c8586cf7be23d4e02e12426a4e3bb6218
        22ec2b07e8e828441c44c9b19d8dd0751c8904ca2434c76494983938a217ab54
        1054314b5ad806bd3d07c8e2f4ffba60d8eff3815b7fb491a539f1223588ba07
        0aa9efc6de441f8666246473dc5e1129381b33f112a2e2ab88c4482c1c539e67
        041b9e8e8838e77bab5910fafe6a53a13764b1129295f17f79393ffbb1edc2a0
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        1958552c3869d902ac345c9bf1d45f67850095067cd34928d71808f144d30c40
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        16757117ee475b956e2426d98fd77231bcef72d9377584428d325b19ba04e60b
        125904266f9bef4b3b1556e6c1b94a1ebd98a4c198286f35bd098888c5a7925d
        15dee8e933c6f003d02c64989001eb6dd6ab92707204e11cad9ac990c8dcd855
        3057d5f321a9886fff413008150b2b07bd75fd2faaab66b67ccdf3ec68c0edf4
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        2003bbb7aa3df577abe5b80f394b14e21c17c1bb8db5b8fc22cb4540bf88b0f3
        19e545865688fff3d8ee36547f1e3fee90de53b2309a19d72d29a7542539d213
        13d067299f354f1077349d22377d1137e00b0765e5284d29c10dfc8d9bac3b16
        29af5012d2fc3f14476101bac77efca5b6b699d6572d2e466ea8c61e6489136b
        1991c945f0174f63aa94a008ce820a2268f3e46e3a24498afb39eb8fc92d5006
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0804e5d88f3213016f34dabc8403b02f5b36c87dc055ab92bc9fe99ffdfaf372
        2d237bc5be8fa9ca187d9beb7a709f42be824ffa86f586f4a059b8f4c54aed60
        2f06c981fef83e50e4e30fc0122b77af53510b2fb93401f6c440cd9b95261779
        1517ec270c0df9bc9ab6f0e320819183e80c95c076f015a80d9133215f2afd83
        00860231686733534cd3568b7d9c0a138497197582f945c12e0c30ce52bb4ff2
        0c8fd0a66fc359b662700bd4c4bbd108dd20f36e2ea9692386c8bd10c9754dad
        2d5584e7290af59620c5fd136b55f8fce9f5a13c4df2b38a5f5443b3f9d79713
        0802ff117a230239f8acf03688a557d78ba44c02088169913b2c8eb62d1a2bab
        1c2b98d78792987ce92f178b18a1df04f4d1626559043c64903512fabe91b0ca
        2358cfa2edf85b4783043306379dc6c08425999c9d38c8d48d67149ac3fa9470
        2f32755adb05f52e208e0e8376f72fcb2aeb9032bfe8aa74fdd73a7f28df605c
        182c60cd36a2688aafecdef018864c8be86f32a329d760a1d6ba8566668baeec
        10a0514daaf03b04d83ee5036bf4975b26d7790dae8b93d9019188352d8b312a
        1f2828a83d1bea0a6af4fbd83471acfa207e6765c0981063046b1577a590baca
        2098bfb225239964ad28a1ac6654b1e4d4699e53198b9b9fca46133f5b8d2c55
        1f0e11b00ce21b010ac093376468d0e726c8a0e8b4d60cf27eb673610d54aba6
        2ace22bfe83ac39ade1ec532325380d416f90cc8e097b6113a4eb3296a22c2a0
        22ef532621092acbaf56a8bbf8a0fee8a144f8a0a436c46a72778f3d9fb51b7d
        2d71e982c925a771c08cfc57b081fd92f91ceb102038e339df32bfd76957c77f
        13f134e96f5efc20381064a21d2d5327dc17c2827dedde6049304917498f6177
        291b87e83343ee3dbd75f76fe002b88002f6fe25c4f58ec52a127d06f72632f8
        1323c3f6283e79f0813366435d0022113472764c39faa0965036fade2c5a181d
        "
    ))
}

#[fixture]
fn valid_vk() -> [u8; VK_SIZE] {
    hex_literal::hex!(
        "
        0000000000000000000000000000000000000000000000000000000000000005
        0000000000000000000000000000000000000000000000000000000000000009
        0000000000000000000000000000000000000000000000000000000000000005
        149a721095f6859166b5139c5a146a6b763a3327ef7429d15157263d2350c4d4
        276026acdb0086065e12d942bcc1ef1e1a079ba56c1ec3963c1453af68ddb07b
        12e72de338668965108935ef0bcc0a0d78d50091def0918bb7c918263f21540f
        0ad38baeafb4099173024919b4d7be6177f3a0fb68b6b1174f9c40dd64af3447
        1ec6379d0ad2ff6b24ea07c4dcc7f3794c3f2cf8f4641232ad83760c31041466
        1eb19496437596a4f935ced2e845685e07dbe1bc0d9a6d97a3ccbbb102f357f5
        09fd8025d9cc1721cd60ffafa09c23c4ac1e1dc3c3c1080984aef2c99974b97e
        18f7ed540433323127fcba0711c4dcb0a7ef11e34524c19649546d901c3e1b77
        18daf46452240bed15259381cb2904292a4db7f8b5daca436198a8923bb85f59
        2bec4c30df10d76bba1913c919f69c8a8082a6cdcfd95a601ffbb3871712a0f9
        1ea253bbc21c331cbe814355ff4a371bc7ddf0e8052e9fe3654b4a7dcb57a4da
        1017eb54958a73a96fa256404feb42ca1c690f81c01c57f5bfb32f5ef923533b
        18a7986de6a0f9d99da5d08edfdd6ea2e8121531e7194102fb75f8508cd2f760
        0b687acbec8e4ac5efaa03520ab622cfeea44c363edbcec754952d08051258b9
        1c4a9d2294d4ff462d7848a6782027166105586ff3971f4d6e2a7c6652f8aadd
        3028839e3a4d2bf827fbd3faf736c31b29f5892a1b98031aef0728213302e4c2
        2a56ce41f6b0be13b9c26747621b821eee81b23a887f299049b14c11e98460d6
        1aa98f2de3ddda547d8f6de4e725ded5827d6338c78656c0d12ca1aea6ef2c7c
        0af3884ecad3331429af995779c2602e93ca1ea976e9e1bc64bbcdbb9fe79212
        1f18803add8ad686e13dc2a989dcfb010cb69b0b38200df51787b7104bc74fb6
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        253c21f9f3795b10195999b67322b28166950c3b2f6f77c10094f69a4779eee1
        1466aeb3b499706d3db0371b0975619ade0665a82706773fd96a2af8e4830f1e
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        024236bda126650fb5228cf424a0878775499e69e8bd2c39af33bd5fa0b4079a
        0d2773e04e739d5a1575a8a6b9d06dacac66e41661ec8a26e384f5a283576215
        0a294c9be8c155b0d32de33e1167a8dd41e48d536e49ddab69649795ca9bc269
        19c7b285818668cd7cf62768c6103ce1059b7cdb66b146327b65dcc10940be60
        0524c8e7146a41551c673b3139893fa365285bca50c9a1b5a476834f5f518c05
        222ec4ffc52d32d1e4a1e4e25327fae9cb22dc0e24ef68cf8adbc2792ddd1bbc
        2cd1c8f85b2800d53095c484906304cad08644c229781b7fb145ab0f15adf0f5
        0dcf0176af2c23487d71d957f5fe6b27022160e084310aa05c0394f2196a5104
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        0000000000000000000000000000000000000000000000000000000000000000
        "
    )
}

#[fixture]
fn valid_pubs() -> [PublicInput; 1] {
    [hex_literal::hex!(
        "0000000000000000000000000000000000000000000000000000000000000001"
    )]
}

#[rstest]
fn verify_valid_zk_proof(
    valid_vk: [u8; VK_SIZE],
    valid_zk_proof: Box<[u8]>,
    valid_pubs: [PublicInput; 1],
) {
    assert!(verify::<()>(&valid_vk, &ProofType::ZK(valid_zk_proof), &valid_pubs).is_ok());
}

#[rstest]
fn verify_valid_plain_proof(
    valid_vk: [u8; VK_SIZE],
    valid_plain_proof: Box<[u8]>,
    valid_pubs: [PublicInput; 1],
) {
    assert!(verify::<()>(&valid_vk, &ProofType::Plain(valid_plain_proof), &valid_pubs).is_ok());
}

// bb's rom_read: table reads at witness indices, so the ROM-LogUp relation (bb v6.0.0) has live
// rows. A verifier without that relation, or with it wrong, rejects these proofs.
mod rom {
    use super::*;

    macro_rules! rom_fixture {
        ($flavor:literal, $file:literal) => {
            include_bytes!(concat!("../tests/data/rom/", $flavor, "/", $file))
        };
    }

    fn pubs(bytes: &[u8]) -> Vec<PublicInput> {
        bytes.chunks_exact(32).map(|c| c.try_into().unwrap()).collect()
    }

    #[rstest]
    #[case::zk(ProofType::ZK(rom_fixture!("zk", "proof").as_slice().into()), rom_fixture!("zk", "vk"), rom_fixture!("zk", "pubs"))]
    #[case::plain(ProofType::Plain(rom_fixture!("plain", "proof").as_slice().into()), rom_fixture!("plain", "vk"), rom_fixture!("plain", "pubs"))]
    fn verify_a_proof_with_rom_reads(#[case] proof: ProofType, #[case] vk: &[u8], #[case] pubs_bytes: &[u8]) {
        let pi = pubs(pubs_bytes);
        assert_eq!(pi.len(), 3);
        assert!(verify::<()>(vk, &proof, &pi).is_ok());
    }

    #[rstest]
    fn reject_a_rom_read_proof_with_a_wrong_read_value() {
        let mut pi = pubs(rom_fixture!("plain", "pubs"));
        pi[1][31] ^= 1; // e1: table[30] claimed as 31
        let proof = ProofType::Plain(rom_fixture!("plain", "proof").as_slice().into());
        assert!(verify::<()>(rom_fixture!("plain", "vk"), &proof, &pi).is_err());
    }
}

// A proof of a circuit that recursively verifies another UltraHonk proof: its pairing point
// object is not the default one, so the verifier has to aggregate it.
mod recursive {
    use super::*;

    macro_rules! recursive_fixture {
        ($kind:literal, $flavor:literal, $file:literal) => {
            include_bytes!(concat!(
                "../tests/data/recursive/",
                $kind,
                "/",
                $flavor,
                "/",
                $file
            ))
        };
    }

    fn proof(flavor: &str, bytes: &[u8]) -> ProofType {
        match flavor {
            "zk" => ProofType::ZK(bytes.into()),
            _ => ProofType::Plain(bytes.into()),
        }
    }

    fn pubs(bytes: &[u8]) -> [PublicInput; 1] {
        [bytes.try_into().expect("a single public input")]
    }

    #[rstest]
    #[case::zk(
        "zk",
        recursive_fixture!("valid", "zk", "vk"),
        recursive_fixture!("valid", "zk", "proof"),
        recursive_fixture!("valid", "zk", "pubs")
    )]
    #[case::plain(
        "plain",
        recursive_fixture!("valid", "plain", "vk"),
        recursive_fixture!("valid", "plain", "proof"),
        recursive_fixture!("valid", "plain", "pubs")
    )]
    fn verify_a_valid_proof(
        #[case] flavor: &str,
        #[case] vk: &[u8],
        #[case] raw_proof: &[u8],
        #[case] raw_pubs: &[u8],
    ) {
        assert!(!proof::are_pairing_points_default(
            &raw_proof[..PAIRING_POINTS_SIZE * EVM_WORD_SIZE]
                .chunks(EVM_WORD_SIZE)
                .map(|w| w.try_into().unwrap())
                .collect::<Vec<EVMWord>>()
                .try_into()
                .unwrap()
        ));
        assert!(verify::<()>(vk, &proof(flavor, raw_proof), &pubs(raw_pubs)).is_ok());
    }

    // The inner proof's KZG quotient was replaced by another curve point. The in-circuit
    // verifier defers the pairing check to the outer proof's pairing point object, so only
    // aggregating that object exposes it: skipping the aggregation would accept this proof.
    #[rstest]
    #[case::zk(
        "zk",
        recursive_fixture!("invalid_inner", "zk", "vk"),
        recursive_fixture!("invalid_inner", "zk", "proof"),
        recursive_fixture!("invalid_inner", "zk", "pubs")
    )]
    #[case::plain(
        "plain",
        recursive_fixture!("invalid_inner", "plain", "vk"),
        recursive_fixture!("invalid_inner", "plain", "proof"),
        recursive_fixture!("invalid_inner", "plain", "pubs")
    )]
    fn reject_a_proof_wrapping_an_invalid_inner_proof(
        #[case] flavor: &str,
        #[case] vk: &[u8],
        #[case] raw_proof: &[u8],
        #[case] raw_pubs: &[u8],
    ) {
        assert_eq!(
            verify::<()>(vk, &proof(flavor, raw_proof), &pubs(raw_pubs)).unwrap_err(),
            VerifyError::VerificationError {
                message: format!("Shplemini Failed. Cause: Shplemini pairing check failed")
            }
        );
    }
}

mod reject {
    use super::*;
    use crate::constants::{
        BATCHED_RELATION_PARTIAL_LENGTH, EVM_WORD_SIZE, FIELD_ELEMENT_SIZE, GROUP_ELEMENT_SIZE,
        NUMBER_OF_WITNESS_ENTITIES, NUMBER_OF_WITNESS_ENTITIES_ZK, PAIRING_POINTS_SIZE,
        ZK_BATCHED_RELATION_PARTIAL_LENGTH,
    };

    // the fixtures' circuit size, from the vk they share
    fn logn() -> u64 {
        VerificationKey::<()>::try_from(&valid_vk()[..]).unwrap().log_circuit_size
    }
    use crate::utils::IntoBEBytes32;
    use ark_ff::PrimeField;

    #[rstest]
    fn a_zk_proof_with_non_matching_number_of_pis(
        valid_vk: [u8; VK_SIZE],
        valid_zk_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let invalid_pubs: [PublicInput; 0] = [];

        assert_eq!(
                verify::<()>(
                    &valid_vk,
                    &ProofType::ZK(valid_zk_proof),
                    &invalid_pubs
                )
                .unwrap_err(),
                VerifyError::PublicInputError {
                    message: format!(
                        "Provided public inputs length does not match value in vk. Expected: {}; Got: {}",
                        valid_pubs.len(),
                        invalid_pubs.len()
                    )
                }
            );
    }

    #[rstest]
    fn a_plain_proof_with_non_matching_number_of_pis(
        valid_vk: [u8; VK_SIZE],
        valid_plain_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let invalid_pubs: [PublicInput; 0] = [];

        assert_eq!(
                verify::<()>(
                    &valid_vk,
                    &ProofType::Plain(valid_plain_proof),
                    &invalid_pubs
                )
                .unwrap_err(),
                VerifyError::PublicInputError {
                    message: format!(
                        "Provided public inputs length does not match value in vk. Expected: {}; Got: {}",
                        valid_pubs.len(),
                        invalid_pubs.len()
                    )
                }
            );
    }

    #[rstest]
    fn a_zk_proof_failing_sumcheck(
        valid_vk: [u8; VK_SIZE],
        valid_zk_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let mut invalid_zk_proof = valid_zk_proof.to_vec();
        invalid_zk_proof.copy_from_slice(&valid_zk_proof);
        invalid_zk_proof[PAIRING_POINTS_SIZE * EVM_WORD_SIZE + 1184
            ..PAIRING_POINTS_SIZE * EVM_WORD_SIZE + 1184 + EVM_WORD_SIZE]
            .fill(0); // Alter sumcheck_univariates[0]

        assert_eq!(
            verify::<()>(
                &valid_vk,
                &ProofType::ZK(invalid_zk_proof.into_boxed_slice()),
                &valid_pubs
            )
            .unwrap_err(),
            VerifyError::VerificationError {
                message: format!(
                    "Sumcheck Failed. Cause: Total Sum differs from Round Target Sum."
                )
            }
        );
    }

    #[rstest]
    fn a_plain_proof_failing_sumcheck(
        valid_vk: [u8; VK_SIZE],
        valid_plain_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let mut invalid_plain_proof = valid_plain_proof.to_vec();
        invalid_plain_proof.copy_from_slice(&valid_plain_proof);
        invalid_plain_proof[PAIRING_POINTS_SIZE * EVM_WORD_SIZE + 0x400
            ..PAIRING_POINTS_SIZE * EVM_WORD_SIZE + 0x400 + EVM_WORD_SIZE]
            .fill(0); // Alter sumcheck_univariates[0]

        assert_eq!(
            verify::<()>(
                &valid_vk,
                &ProofType::Plain(invalid_plain_proof.into_boxed_slice()),
                &valid_pubs
            )
            .unwrap_err(),
            VerifyError::VerificationError {
                message: format!(
                    "Sumcheck Failed. Cause: Total Sum differs from Round Target Sum."
                )
            }
        );
    }

    #[rstest]
    fn a_zk_proof_failing_sumcheck_v2(
        valid_vk: [u8; VK_SIZE],
        valid_zk_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let mut invalid_zk_proof = valid_zk_proof.to_vec();
        // pairing points, the witness commitments (with the masking poly), Libra's first
        // commitment and sum, the sumcheck univariates, then the evaluations: the first is the
        // masking poly's, which only Shplemini uses, so alter the second (a relation entity)
        let sumcheck_evaluations_offset = PAIRING_POINTS_SIZE * EVM_WORD_SIZE
            + (NUMBER_OF_WITNESS_ENTITIES_ZK + 1) * GROUP_ELEMENT_SIZE
            + EVM_WORD_SIZE
            + logn() as usize * ZK_BATCHED_RELATION_PARTIAL_LENGTH * EVM_WORD_SIZE
            + EVM_WORD_SIZE;
        invalid_zk_proof[sumcheck_evaluations_offset..sumcheck_evaluations_offset + EVM_WORD_SIZE]
            .fill(0); // Alter sumcheck_evaluations

        assert_eq!(
                verify::<()>(&valid_vk, &ProofType::ZK(invalid_zk_proof.into_boxed_slice()), &valid_pubs).unwrap_err(),
                VerifyError::VerificationError {
                    message: format!(
                        "Sumcheck Failed. Cause: Grand Honk Relation Sum does not match Round Target Sum."
                    )
                }
            );
    }

    #[rstest]
    fn a_plain_proof_failing_sumcheck_v2(
        valid_vk: [u8; VK_SIZE],
        valid_plain_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let mut invalid_plain_proof = valid_plain_proof.into_vec();
        // pairing points, the witness commitments, then the sumcheck univariates
        let sumcheck_evaluations_offset = PAIRING_POINTS_SIZE * EVM_WORD_SIZE
            + NUMBER_OF_WITNESS_ENTITIES * GROUP_ELEMENT_SIZE
            + logn() as usize * BATCHED_RELATION_PARTIAL_LENGTH * EVM_WORD_SIZE;
        invalid_plain_proof
            [sumcheck_evaluations_offset..sumcheck_evaluations_offset + EVM_WORD_SIZE]
            .fill(0); // Alter sumcheck_evaluations

        assert_eq!(
                verify::<()>(&valid_vk, &ProofType::Plain(invalid_plain_proof.into_boxed_slice()), &valid_pubs).unwrap_err(),
                VerifyError::VerificationError {
                    message: format!(
                        "Sumcheck Failed. Cause: Grand Honk Relation Sum does not match Round Target Sum."
                    )
                }
            );
    }

    #[rstest]
    fn a_zk_proof_failing_shplemini_pairing_check(
        valid_vk: [u8; VK_SIZE],
        valid_zk_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let log_circuit_size = VerificationKey::<()>::try_from(&valid_vk[..])
            .expect("vk is valid")
            .log_circuit_size;
        let zk_proof_size = ZKProof::<()>::calculate_proof_byte_len(log_circuit_size);
        let offset = zk_proof_size
            - 2 * GROUP_ELEMENT_SIZE
            - FIELD_ELEMENT_SIZE * NUM_LIBRA_EVALUATIONS
            - FIELD_ELEMENT_SIZE * log_circuit_size as usize;
        let mut invalid_zk_proof = valid_zk_proof.into_vec();
        // Alter field; notice that (1, 3) ∉ G1
        invalid_zk_proof[offset..offset + GROUP_ELEMENT_SIZE].fill(0);
        invalid_zk_proof[offset + EVM_WORD_SIZE - 1] = 1;
        invalid_zk_proof[offset + 2 * EVM_WORD_SIZE - 1] = 3;

        assert_eq!(
            verify::<()>(
                &valid_vk,
                &ProofType::ZK(invalid_zk_proof.into_boxed_slice()),
                &valid_pubs
            )
            .unwrap_err(),
            VerifyError::VerificationError {
                message: format!("Shplemini Failed. Cause: Shplemini pairing check failed")
            }
        );
    }

    #[rstest]
    fn a_plain_proof_failing_shplemini_pairing_check(
        valid_vk: [u8; VK_SIZE],
        valid_plain_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let log_circuit_size = VerificationKey::<()>::try_from(&valid_vk[..])
            .expect("vk is valid")
            .log_circuit_size;
        let plain_proof_size = PlainProof::<()>::calculate_proof_byte_len(log_circuit_size);
        let offset = plain_proof_size
            - 2 * GROUP_ELEMENT_SIZE
            - FIELD_ELEMENT_SIZE * log_circuit_size as usize;

        let mut invalid_plain_proof = valid_plain_proof.into_vec();
        // Alter field; notice that (1, 3) ∉ G1
        invalid_plain_proof[offset..offset + GROUP_ELEMENT_SIZE].fill(0);
        invalid_plain_proof[offset + EVM_WORD_SIZE - 1] = 1;
        invalid_plain_proof[offset + 2 * EVM_WORD_SIZE - 1] = 3;

        assert_eq!(
            verify::<()>(
                &valid_vk,
                &ProofType::Plain(invalid_plain_proof.into_boxed_slice()),
                &valid_pubs
            )
            .unwrap_err(),
            VerifyError::VerificationError {
                message: format!("Shplemini Failed. Cause: Shplemini pairing check failed")
            }
        );
    }

    // Big-endian `word + r`: the same field element, non-canonically encoded.
    fn add_modulus(word: &[u8; 32]) -> [u8; 32] {
        let modulus = Fr::MODULUS.into_be_bytes32();
        let mut out = [0u8; 32];
        let mut carry = 0u16;
        for i in (0..32).rev() {
            let sum = word[i] as u16 + modulus[i] as u16 + carry;
            out[i] = sum as u8;
            carry = sum >> 8;
        }
        assert_eq!(carry, 0, "word + r must fit in 256 bits");
        out
    }

    #[rstest]
    fn a_proof_with_a_non_canonical_public_input(
        valid_vk: [u8; VK_SIZE],
        valid_zk_proof: Box<[u8]>,
        valid_plain_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let invalid_pubs = [add_modulus(&valid_pubs[0])];
        for proof in [
            ProofType::ZK(valid_zk_proof),
            ProofType::Plain(valid_plain_proof),
        ] {
            assert_eq!(
                verify::<()>(&valid_vk, &proof, &invalid_pubs).unwrap_err(),
                VerifyError::PublicInputError {
                    message: format!("Public input 0 is not a canonical field element")
                }
            );
        }
    }

    #[rstest]
    fn a_proof_with_a_non_canonical_scalar(
        valid_vk: [u8; VK_SIZE],
        valid_zk_proof: Box<[u8]>,
        valid_plain_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        // The last scalar before the trailing Shplonk Q and KZG quotient commitments.
        let offset = |len: usize| len - 2 * GROUP_ELEMENT_SIZE - FIELD_ELEMENT_SIZE;
        let tamper = |proof: Box<[u8]>| {
            let mut proof = proof.into_vec();
            let at = offset(proof.len());
            let word: [u8; 32] = proof[at..at + 32].try_into().unwrap();
            proof[at..at + 32].copy_from_slice(&add_modulus(&word));
            proof.into_boxed_slice()
        };

        assert_eq!(
            verify::<()>(
                &valid_vk,
                &ProofType::ZK(tamper(valid_zk_proof)),
                &valid_pubs
            )
            .unwrap_err(),
            VerifyError::InvalidProofError {
                message: format!("Failed parsing ZK proof")
            }
        );
        assert_eq!(
            verify::<()>(
                &valid_vk,
                &ProofType::Plain(tamper(valid_plain_proof)),
                &valid_pubs
            )
            .unwrap_err(),
            VerifyError::InvalidProofError {
                message: format!("Failed parsing Plain proof")
            }
        );
    }

    #[rstest]
    #[case::lo_limb(0, 136)]
    #[case::hi_limb(1, 120)]
    fn a_proof_with_an_oversized_pairing_point_limb(
        valid_vk: [u8; VK_SIZE],
        valid_zk_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
        #[case] limb: usize,
        #[case] max_bits: usize,
    ) {
        let mut invalid_zk_proof = valid_zk_proof.into_vec();
        // Set bit `max_bits` of the limb, so that it is exactly 2^max_bits.
        let word = &mut invalid_zk_proof[limb * EVM_WORD_SIZE..(limb + 1) * EVM_WORD_SIZE];
        word.fill(0);
        word[EVM_WORD_SIZE - 1 - max_bits / 8] = 1;

        assert_eq!(
            verify::<()>(
                &valid_vk,
                &ProofType::ZK(invalid_zk_proof.into_boxed_slice()),
                &valid_pubs
            )
            .unwrap_err(),
            VerifyError::InvalidProofError {
                message: format!("Failed parsing ZK proof")
            }
        );
    }

    // Commitments equal to (0, 0) decode to the point at infinity, which has no affine
    // coordinates: hashing one into the transcript must not panic.
    #[rstest]
    fn a_proof_with_a_commitment_at_infinity_without_panicking(
        valid_vk: [u8; VK_SIZE],
        valid_zk_proof: Box<[u8]>,
        valid_plain_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let w1_offset = |proof: &ProofType| match proof {
            // Pairing point object, then the Gemini masking commitment (ZK only).
            ProofType::ZK(_) => PAIRING_POINTS_SIZE * EVM_WORD_SIZE + GROUP_ELEMENT_SIZE,
            ProofType::Plain(_) => PAIRING_POINTS_SIZE * EVM_WORD_SIZE,
        };
        for proof in [
            ProofType::ZK(valid_zk_proof),
            ProofType::Plain(valid_plain_proof),
        ] {
            let at = w1_offset(&proof);
            let invalid_proof = match proof {
                ProofType::ZK(p) | ProofType::Plain(p) => {
                    let mut p = p.into_vec();
                    p[at..at + GROUP_ELEMENT_SIZE].fill(0);
                    p.into_boxed_slice()
                }
            };
            let invalid_proof = match at {
                at if at == PAIRING_POINTS_SIZE * EVM_WORD_SIZE => ProofType::Plain(invalid_proof),
                _ => ProofType::ZK(invalid_proof),
            };
            assert!(matches!(
                verify::<()>(&valid_vk, &invalid_proof, &valid_pubs).unwrap_err(),
                VerifyError::VerificationError { .. }
            ));
        }
    }

    #[rstest]
    fn a_vk_with_a_combined_input_size_smaller_than_the_pairing_point_object(
        valid_vk: [u8; VK_SIZE],
        valid_zk_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        let mut invalid_vk = valid_vk;
        // Second header word: combined_input_size = PAIRING_POINTS_SIZE - 1
        invalid_vk[EVM_WORD_SIZE..2 * EVM_WORD_SIZE].fill(0);
        invalid_vk[2 * EVM_WORD_SIZE - 1] = PAIRING_POINTS_SIZE as u8 - 1;

        assert_eq!(
            verify::<()>(&invalid_vk, &ProofType::ZK(valid_zk_proof), &valid_pubs).unwrap_err(),
            VerifyError::KeyError
        );
    }

    #[rstest]
    fn a_proof_with_the_pre_v5_length(
        valid_vk: [u8; VK_SIZE],
        valid_zk_proof: Box<[u8]>,
        valid_pubs: [PublicInput; 1],
    ) {
        // bb < 5.0.0 proofs carry 16 pairing point limbs instead of 8.
        let mut v3_sized_proof = valid_zk_proof.into_vec();
        v3_sized_proof.resize(v3_sized_proof.len() + 8 * EVM_WORD_SIZE, 0);

        assert!(matches!(
            verify::<()>(
                &valid_vk,
                &ProofType::ZK(v3_sized_proof.into_boxed_slice()),
                &valid_pubs
            )
            .unwrap_err(),
            VerifyError::InvalidProofError { .. }
        ));
    }
}
