:robot: I have created a release *beep* *boop*
---


<details><summary>auctioneer: 0.0.2</summary>

## [0.0.2](https://github.com/astriaorg/astria-release-test/compare/auctioneer-vv0.0.1...auctioneer-vv0.0.2) (2025-05-09)


### Features

* **auctioneer:** add new service ([#1839](https://github.com/astriaorg/astria-release-test/issues/1839)) ([2db84ed](https://github.com/astriaorg/astria-release-test/commit/2db84ed46430013afa24ee955fe2a24f19d30675))


### Miscellaneous

* bump all major dependencies ([#2007](https://github.com/astriaorg/astria-release-test/issues/2007)) ([3b8c453](https://github.com/astriaorg/astria-release-test/commit/3b8c453f10d2d02f4be934aaaecd9d9ab76c0202))
</details>

<details><summary>bridge-withdrawer: 2.0.0</summary>

## [2.0.0](https://github.com/astriaorg/astria-release-test/compare/bridge-withdrawer-vv1.0.2...bridge-withdrawer-vv2.0.0) (2025-05-09)


###   BREAKING CHANGES

* **sequencer, core, conductor:** merge upgrades/price-feed feature branch to main ([#2085](https://github.com/astriaorg/astria-release-test/issues/2085))
* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825))
* **bridge-withdrawer, cli:** use decimals from erc20 contract ([#1762](https://github.com/astriaorg/astria-release-test/issues/1762))
* **bridge-withdrawer:** correctly identify rollup return address in ics20 withdrawal actions ([#1714](https://github.com/astriaorg/astria-release-test/issues/1714))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672))
* **proto:** call transactions `Transaction`, contents `TransactionBody` ([#1650](https://github.com/astriaorg/astria-release-test/issues/1650))
* **proto, core:** remove action suffix from all action types ([#1630](https://github.com/astriaorg/astria-release-test/issues/1630))
* **sequencer:** transaction categories on UnsignedTransaction ([#1512](https://github.com/astriaorg/astria-release-test/issues/1512))
* **proto, core, sequencer:** add traceability to rollup deposits ([#1410](https://github.com/astriaorg/astria-release-test/issues/1410))
* **proto, core, sequencer:** permit bech32 compatible addresses ([#1425](https://github.com/astriaorg/astria-release-test/issues/1425))
* **sequencer, bridge-withdrawer:** enforce withdrawals consumed ([#1391](https://github.com/astriaorg/astria-release-test/issues/1391))
* **core, sequencer:** require that bridge unlock address always be set ([#1339](https://github.com/astriaorg/astria-release-test/issues/1339))
* **core, proto:** define bridge memos in proto ([#1285](https://github.com/astriaorg/astria-release-test/issues/1285))
* **bridge-withdrawer:** fix nonce handling ([#1215](https://github.com/astriaorg/astria-release-test/issues/1215))
* **cli, bridge-withdrawer:** share code between cli and service ([#1270](https://github.com/astriaorg/astria-release-test/issues/1270))
* **core, bridge, sequencer:** dismabiguate return addresses ([#1266](https://github.com/astriaorg/astria-release-test/issues/1266))
* **core, proto:** make bridge unlock memo string ([#1244](https://github.com/astriaorg/astria-release-test/issues/1244))
* **bridge-withdrawer:** refactor startup to a separate subtask and remove balance check from startup ([#1190](https://github.com/astriaorg/astria-release-test/issues/1190))
* **core, bridge-withdrawer:** move bridge-unlock memo to core ([#1245](https://github.com/astriaorg/astria-release-test/issues/1245))
* **proto, sequencer:** use full IBC ICS20 denoms instead of IDs ([#1209](https://github.com/astriaorg/astria-release-test/issues/1209))
* **sequencer:** allow configuring base address prefix ([#1201](https://github.com/astriaorg/astria-release-test/issues/1201))
* **sequencer:** implement bridge sudo and withdrawer addresses ([#1142](https://github.com/astriaorg/astria-release-test/issues/1142))
* **core, proto:** add bech32m addresses ([#1124](https://github.com/astriaorg/astria-release-test/issues/1124))

### Features

* **bridge-withdrawer:** add `use_compat_address` configuration value ([#1671](https://github.com/astriaorg/astria-release-test/issues/1671)) ([49e2d9f](https://github.com/astriaorg/astria-release-test/commit/49e2d9f0f87156ea17b499c78fe696c5eebc766f)), closes [#1424](https://github.com/astriaorg/astria-release-test/issues/1424)
* **bridge-withdrawer:** add justfile ([#1135](https://github.com/astriaorg/astria-release-test/issues/1135)) ([52088e0](https://github.com/astriaorg/astria-release-test/commit/52088e04ec535a925db2af57304e3a1c43364644))
* **bridge-withdrawer:** bridge withdrawer startup ([#1160](https://github.com/astriaorg/astria-release-test/issues/1160)) ([d70fa98](https://github.com/astriaorg/astria-release-test/commit/d70fa98e3836e67229d73a5a07bc7b675a0cc9d7))
* **bridge-withdrawer:** metric to track total settled funds ([#1693](https://github.com/astriaorg/astria-release-test/issues/1693)) ([29683fb](https://github.com/astriaorg/astria-release-test/commit/29683fb21a921170510c8b9dcc75386d435fce20))
* **bridge-withdrawer:** PoC astria-bridge-withdrawer implementation ([#984](https://github.com/astriaorg/astria-release-test/issues/984)) ([afe4901](https://github.com/astriaorg/astria-release-test/commit/afe4901827d636a51a4c774f2ef4c8ee082db19c))
* **bridge-withdrawer:** support FROST threshold signing ([#1948](https://github.com/astriaorg/astria-release-test/issues/1948)) ([92e2064](https://github.com/astriaorg/astria-release-test/commit/92e2064d4bc623ec5b485fbd14f824d3784236da))
* **bridge-withdrawer:** sync logic ([#1165](https://github.com/astriaorg/astria-release-test/issues/1165)) ([3d016b2](https://github.com/astriaorg/astria-release-test/commit/3d016b24ef127ba761129cd4ccd5f5587d6d5500))
* **cli, bridge-withdrawer:** share code between cli and service ([#1270](https://github.com/astriaorg/astria-release-test/issues/1270)) ([75ac37a](https://github.com/astriaorg/astria-release-test/commit/75ac37a5009f7fbe4105a05c7bda2878bb56ea4e))
* **core, proto:** add bech32m addresses ([#1124](https://github.com/astriaorg/astria-release-test/issues/1124)) ([ab8705f](https://github.com/astriaorg/astria-release-test/commit/ab8705f2e0273a158db5ea5248fe0b331a818c8a))
* **core, proto:** make bridge unlock memo string ([#1244](https://github.com/astriaorg/astria-release-test/issues/1244)) ([afd8bcb](https://github.com/astriaorg/astria-release-test/commit/afd8bcb3da295dc7206da05c5a8e37fd7d15a029))
* **proto, core, sequencer:** add traceability to rollup deposits ([#1410](https://github.com/astriaorg/astria-release-test/issues/1410)) ([f543222](https://github.com/astriaorg/astria-release-test/commit/f5432228090e794a917b6f0803f3a26dc1609dcc))
* **proto, core, sequencer:** permit bech32 compatible addresses ([#1425](https://github.com/astriaorg/astria-release-test/issues/1425)) ([0ed3c19](https://github.com/astriaorg/astria-release-test/commit/0ed3c190651e4a2c07ffe304af95ff5756d13e7d))
* **proto, sequencer:** use full IBC ICS20 denoms instead of IDs ([#1209](https://github.com/astriaorg/astria-release-test/issues/1209)) ([f05e829](https://github.com/astriaorg/astria-release-test/commit/f05e8297a4a9ac7d1e1d4f1a3edc266e62b23ddb))
* **sequencer, bridge-withdrawer:** enforce withdrawals consumed ([#1391](https://github.com/astriaorg/astria-release-test/issues/1391)) ([9f61870](https://github.com/astriaorg/astria-release-test/commit/9f618708008e97e63efbe3f1a3a7f31ceb2c39d7))
* **sequencer, core, conductor:** merge upgrades/price-feed feature branch to main ([#2085](https://github.com/astriaorg/astria-release-test/issues/2085)) ([9fd1517](https://github.com/astriaorg/astria-release-test/commit/9fd15173da036a3394f3a774df5c72a985e32aee))
* **sequencer:** allow configuring base address prefix ([#1201](https://github.com/astriaorg/astria-release-test/issues/1201)) ([d35271d](https://github.com/astriaorg/astria-release-test/commit/d35271dfb4e9cfa9c8b5f2da8fe1ddfd0f3cbdd3))
* **sequencer:** implement bridge sudo and withdrawer addresses ([#1142](https://github.com/astriaorg/astria-release-test/issues/1142)) ([29baa40](https://github.com/astriaorg/astria-release-test/commit/29baa40341aa12769817450310cc9d4c52429503))
* **sequencer:** implement refund to rollup logic upon ics20 transfer refund ([#1161](https://github.com/astriaorg/astria-release-test/issues/1161)) ([cd5dbd0](https://github.com/astriaorg/astria-release-test/commit/cd5dbd0cb609fb06d9838fbb9a618c7cde917a97)), closes [#1110](https://github.com/astriaorg/astria-release-test/issues/1110)
* **sequencer:** transaction categories on UnsignedTransaction ([#1512](https://github.com/astriaorg/astria-release-test/issues/1512)) ([17e6711](https://github.com/astriaorg/astria-release-test/commit/17e6711ce4032930519660f70a9e09af1dea90f7))
* use macro to declare metric constants ([#1129](https://github.com/astriaorg/astria-release-test/issues/1129)) ([fb1d7b8](https://github.com/astriaorg/astria-release-test/commit/fb1d7b86a3bbd98793b294894f1c65c81c1c414e))
* **withdrawer:** bridged ERC20 token withdrawals ([#1149](https://github.com/astriaorg/astria-release-test/issues/1149)) ([b52a824](https://github.com/astriaorg/astria-release-test/commit/b52a824871f1c417c7a8ba2f563425b630b3f8a9)), closes [#924](https://github.com/astriaorg/astria-release-test/issues/924)


### Bug Fixes

* **astria-bridge-withdrawer:** use correct file paths in build script ([#1198](https://github.com/astriaorg/astria-release-test/issues/1198)) ([c376cfb](https://github.com/astriaorg/astria-release-test/commit/c376cfbab3b240c36cc467d0e6448def833bd785))
* **bridge-contracts:** fix memo transaction hash encoding ([#1428](https://github.com/astriaorg/astria-release-test/issues/1428)) ([6b5dae9](https://github.com/astriaorg/astria-release-test/commit/6b5dae9b1f432d452abd848b887ff06c583fb160))
* **bridge-withdrawer, cli, sequencer-client:** migrate from `broadcast_tx_commit` to `broadcast_tx_sync` ([#1376](https://github.com/astriaorg/astria-release-test/issues/1376)) ([64b9106](https://github.com/astriaorg/astria-release-test/commit/64b91061a159d7c919bf2f020c29e1cf514d8843))
* **bridge-withdrawer, cli:** use decimals from erc20 contract ([#1762](https://github.com/astriaorg/astria-release-test/issues/1762)) ([ca72c64](https://github.com/astriaorg/astria-release-test/commit/ca72c64a6513dff0c55aaa14c2a5d7d683ab8370))
* **bridge-withdrawer, cli:** use leading channel ([#1768](https://github.com/astriaorg/astria-release-test/issues/1768)) ([1f1d577](https://github.com/astriaorg/astria-release-test/commit/1f1d5770fd31d9fd58a2fb8de5bba249e5b35669))
* **bridge-withdrawer:** also set metric when value 0 ([#1771](https://github.com/astriaorg/astria-release-test/issues/1771)) ([5fc9c56](https://github.com/astriaorg/astria-release-test/commit/5fc9c56b6030038e5d7806a6785adee8a2d18cd3))
* **bridge-withdrawer:** correctly identify rollup return address in ics20 withdrawal actions ([#1714](https://github.com/astriaorg/astria-release-test/issues/1714)) ([815b083](https://github.com/astriaorg/astria-release-test/commit/815b083cbfb47777cf68517139c9646203b9db47))
* **bridge-withdrawer:** don't panic on init ([#1281](https://github.com/astriaorg/astria-release-test/issues/1281)) ([a6d3d96](https://github.com/astriaorg/astria-release-test/commit/a6d3d96bd483afb752f8a8a038f1e287e9fbf1d9))
* **bridge-withdrawer:** fix nonce handling ([#1215](https://github.com/astriaorg/astria-release-test/issues/1215)) ([bb2f96c](https://github.com/astriaorg/astria-release-test/commit/bb2f96c01607a30806cb2195b6a7feb9ca325826))
* **bridge-withdrawer:** skip linting generated contract code ([#1172](https://github.com/astriaorg/astria-release-test/issues/1172)) ([9cf79a7](https://github.com/astriaorg/astria-release-test/commit/9cf79a733f705ce25b466d5db31a92e89935b174)), closes [#1169](https://github.com/astriaorg/astria-release-test/issues/1169)
* **charts, bridge:** fix ci test ([#1310](https://github.com/astriaorg/astria-release-test/issues/1310)) ([4286bba](https://github.com/astriaorg/astria-release-test/commit/4286bba0c10e18c1815203fb32bebef5ab08c411))
* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825)) ([fc10a63](https://github.com/astriaorg/astria-release-test/commit/fc10a63a82d2854420271f3b03268e31e40b1cd7))
* **cli, bridge-withdrawer:** dont fail entire block due to bad withdraw event ([#1409](https://github.com/astriaorg/astria-release-test/issues/1409)) ([3e24c50](https://github.com/astriaorg/astria-release-test/commit/3e24c50fc1daec666a51e93756268512fb098182))
* **core, bridge, sequencer:** dismabiguate return addresses ([#1266](https://github.com/astriaorg/astria-release-test/issues/1266)) ([bb41dc4](https://github.com/astriaorg/astria-release-test/commit/bb41dc4e6bcf64712d1e7dfd8cf31b4f8361741a))
* **core, sequencer:** prefix removal source non-refund ics20 packet ([#1162](https://github.com/astriaorg/astria-release-test/issues/1162)) ([6c29d39](https://github.com/astriaorg/astria-release-test/commit/6c29d39e89ead4fe082962377ae02976588a33b8))
* **proto:** fix import name mismatch ([#1380](https://github.com/astriaorg/astria-release-test/issues/1380)) ([f69ffe2](https://github.com/astriaorg/astria-release-test/commit/f69ffe22a53b063984c83c5993798e249b39c46d))
* **withdrawer:** support withdrawer address that differs from bridge address   ([#1262](https://github.com/astriaorg/astria-release-test/issues/1262)) ([0684117](https://github.com/astriaorg/astria-release-test/commit/0684117e96a21a9eb9f0e14d3a2a40373ce6f966)), closes [#1241](https://github.com/astriaorg/astria-release-test/issues/1241)
* **withdrawer:** update AstriaWithdrawer to check that withdrawal value is sufficient ([#1148](https://github.com/astriaorg/astria-release-test/issues/1148)) ([b62563d](https://github.com/astriaorg/astria-release-test/commit/b62563d5e4fe6deae42f58a08df25b8f34879c33))
* **withdrawer:** use block subscription in batcher; send to destination_chain_address ([#1157](https://github.com/astriaorg/astria-release-test/issues/1157)) ([08cb823](https://github.com/astriaorg/astria-release-test/commit/08cb823b3eb1a8472f706dc56edbecd54a2ee8cb))


### Miscellaneous

* **all:** add changelogs ([#1700](https://github.com/astriaorg/astria-release-test/issues/1700)) ([4f7e53a](https://github.com/astriaorg/astria-release-test/commit/4f7e53a7da874e7b198c102da74da54729999e7a))
* **all:** Migrate all instances of `#[allow]` to `#[expect]` ([#1561](https://github.com/astriaorg/astria-release-test/issues/1561)) ([eced579](https://github.com/astriaorg/astria-release-test/commit/eced5797ead1ee6bd094d3574fe61cdad04e5702)), closes [#1521](https://github.com/astriaorg/astria-release-test/issues/1521)
* **all:** remove `once_cell` ([#1576](https://github.com/astriaorg/astria-release-test/issues/1576)) ([3bf4652](https://github.com/astriaorg/astria-release-test/commit/3bf4652899fd6ab1d5fd6e9caca7369d078bbc40)), closes [#1520](https://github.com/astriaorg/astria-release-test/issues/1520)
* **bridge-withdrawer, composer:** encode submission payload only once ([#2053](https://github.com/astriaorg/astria-release-test/issues/2053)) ([e6db8ce](https://github.com/astriaorg/astria-release-test/commit/e6db8ce9e8836cbfcec6cf994d77c24ed0648f59)), closes [#2050](https://github.com/astriaorg/astria-release-test/issues/2050)
* **bridge-withdrawer:** Add instrumentation ([#1324](https://github.com/astriaorg/astria-release-test/issues/1324)) ([217bc13](https://github.com/astriaorg/astria-release-test/commit/217bc133845b9070556fa3b3aea4ca0baa78161d))
* **bridge-withdrawer:** add missing errors and clean up names ([#1178](https://github.com/astriaorg/astria-release-test/issues/1178)) ([376751c](https://github.com/astriaorg/astria-release-test/commit/376751c34f5e081be165e7cfb329296dcbbfbebe))
* **bridge-withdrawer:** better grpc client construction ([#1528](https://github.com/astriaorg/astria-release-test/issues/1528)) ([0e889ef](https://github.com/astriaorg/astria-release-test/commit/0e889ef88dd235191a15f3646ac7b09efbb04937)), closes [#1527](https://github.com/astriaorg/astria-release-test/issues/1527) [#1542](https://github.com/astriaorg/astria-release-test/issues/1542)
* **bridge-withdrawer:** cleanup nonce handling ([#1292](https://github.com/astriaorg/astria-release-test/issues/1292)) ([aed95e0](https://github.com/astriaorg/astria-release-test/commit/aed95e0b7f7f594fcae08d282e2cbed069e210f5))
* **bridge-withdrawer:** pass GRPC and CometBFT clients to consumers directly ([#1510](https://github.com/astriaorg/astria-release-test/issues/1510)) ([f7ef132](https://github.com/astriaorg/astria-release-test/commit/f7ef132113b7815cefc19e4e82d300842d450727)), closes [#1315](https://github.com/astriaorg/astria-release-test/issues/1315)
* **bridge-withdrawer:** typo fixes ([#2054](https://github.com/astriaorg/astria-release-test/issues/2054)) ([b522c11](https://github.com/astriaorg/astria-release-test/commit/b522c11cf4727578cd6c574d9df7411c59d88a43))
* bump all major dependencies ([#2007](https://github.com/astriaorg/astria-release-test/issues/2007)) ([3b8c453](https://github.com/astriaorg/astria-release-test/commit/3b8c453f10d2d02f4be934aaaecd9d9ab76c0202))
* bump msrv and run clippy ([#1167](https://github.com/astriaorg/astria-release-test/issues/1167)) ([6902ef3](https://github.com/astriaorg/astria-release-test/commit/6902ef35370e5980a76302fc756e1a9a56af21b5))
* bump to rust version 1.83 ([#1857](https://github.com/astriaorg/astria-release-test/issues/1857)) ([2899049](https://github.com/astriaorg/astria-release-test/commit/2899049bf0dd5bd7ba05927a5daf73ee986a46dc)), closes [#1580](https://github.com/astriaorg/astria-release-test/issues/1580)
* **ci:** bump rust toolchain to 1.81 ([#1523](https://github.com/astriaorg/astria-release-test/issues/1523)) ([4478cb6](https://github.com/astriaorg/astria-release-test/commit/4478cb644990e608a11248d53ca3bae4aad009f1))
* cut releases ([#2017](https://github.com/astriaorg/astria-release-test/issues/2017)) ([a12efeb](https://github.com/astriaorg/astria-release-test/commit/a12efeb0e4000d8ac2adc4e70ced4954cfbbb94c))
* **metrics:** restrict `metrics` crate usage to `astria-telemetry` ([#1192](https://github.com/astriaorg/astria-release-test/issues/1192)) ([f251316](https://github.com/astriaorg/astria-release-test/commit/f25131683865a8952a9b2cf24b1e541a882b571a))
* **metrics:** update `metric_name` macro to handle a collection of names ([#1163](https://github.com/astriaorg/astria-release-test/issues/1163)) ([53a1ecb](https://github.com/astriaorg/astria-release-test/commit/53a1ecb5afca0ccdbf412674eaca96227d377379))
* **proto, core:** remove action suffix from all action types ([#1630](https://github.com/astriaorg/astria-release-test/issues/1630)) ([3abd40a](https://github.com/astriaorg/astria-release-test/commit/3abd40ab2ecee5a425ff592859bf8ae8fd2c4a97))
* **proto:** call transactions `Transaction`, contents `TransactionBody` ([#1650](https://github.com/astriaorg/astria-release-test/issues/1650)) ([1c0284e](https://github.com/astriaorg/astria-release-test/commit/1c0284edd1cb2897ad7528ee96d147781cb354f9))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672)) ([812960f](https://github.com/astriaorg/astria-release-test/commit/812960f713d07d7aeed479c5e805d6238fe20312))
* register all metrics during startup ([#1144](https://github.com/astriaorg/astria-release-test/issues/1144)) ([5f117cb](https://github.com/astriaorg/astria-release-test/commit/5f117cb9148016070297f0a4eb1e1f975fc94e4a))
* remove redundant bin entries from all crates' Cargo.toml ([#1725](https://github.com/astriaorg/astria-release-test/issues/1725)) ([8d9aae4](https://github.com/astriaorg/astria-release-test/commit/8d9aae4027ac4c0eb6758f2fb620e5e378f5e76b))
* remove unused dependencies ([#1174](https://github.com/astriaorg/astria-release-test/issues/1174)) ([d2cdea7](https://github.com/astriaorg/astria-release-test/commit/d2cdea7f77099e181acdbfcabf6464eb8ed3e6bb))
* **sequencer-relayer:** add metrics for recording Celestia fees ([#1742](https://github.com/astriaorg/astria-release-test/issues/1742)) ([3c6e456](https://github.com/astriaorg/astria-release-test/commit/3c6e456c23fa9ab0b6aeca5fb5eef7d90931b8ff))
* update `url` dependency ([#1869](https://github.com/astriaorg/astria-release-test/issues/1869)) ([6e91760](https://github.com/astriaorg/astria-release-test/commit/6e91760cd67832db997c1534b5dc0394d7d0d113))
* **withdrawer:** replace contracts with `astria-bridge-contracts` submodule ([#1164](https://github.com/astriaorg/astria-release-test/issues/1164)) ([7a0a17a](https://github.com/astriaorg/astria-release-test/commit/7a0a17ac43761291728145857aed7606dc1ca46e))


### Code Refactoring

* **bridge-withdrawer:** move generated contract bindings to crate ([#1237](https://github.com/astriaorg/astria-release-test/issues/1237)) ([c9e4898](https://github.com/astriaorg/astria-release-test/commit/c9e48980d4ffc1ff8e93395f872e0899496c1073))
* **bridge-withdrawer:** refactor startup to a separate subtask and remove balance check from startup ([#1190](https://github.com/astriaorg/astria-release-test/issues/1190)) ([df76869](https://github.com/astriaorg/astria-release-test/commit/df768695b026999ba7af38ae7a98b4da4f54a2d3)), closes [#1229](https://github.com/astriaorg/astria-release-test/issues/1229)
* **core, bridge-withdrawer:** move bridge-unlock memo to core ([#1245](https://github.com/astriaorg/astria-release-test/issues/1245)) ([06ff55e](https://github.com/astriaorg/astria-release-test/commit/06ff55e074a2d8271a660f14b97023147e19118b))
* **core, proto:** define bridge memos in proto ([#1285](https://github.com/astriaorg/astria-release-test/issues/1285)) ([892e408](https://github.com/astriaorg/astria-release-test/commit/892e408abcc10adcb842675eec749e91fff5972a))
* **core, sequencer:** require that bridge unlock address always be set ([#1339](https://github.com/astriaorg/astria-release-test/issues/1339)) ([ee31f2d](https://github.com/astriaorg/astria-release-test/commit/ee31f2dec88b238694076a26858300a7dc1604e4)), closes [#1338](https://github.com/astriaorg/astria-release-test/issues/1338)
* **core:** move address logic to crate ([#1802](https://github.com/astriaorg/astria-release-test/issues/1802)) ([3e85e8d](https://github.com/astriaorg/astria-release-test/commit/3e85e8d99beae2002a392122c87378d25800c142))
* **core:** parse ics20 denoms as ibc or trace prefixed variants ([#1181](https://github.com/astriaorg/astria-release-test/issues/1181)) ([616dd9a](https://github.com/astriaorg/astria-release-test/commit/616dd9a9a209406db11c545336e9b578035bb208))
* **withdrawer:** read from block subscription stream and get events on each block ([#1207](https://github.com/astriaorg/astria-release-test/issues/1207)) ([dc76efc](https://github.com/astriaorg/astria-release-test/commit/dc76efc66ce4550d5fff7bb786c81a2383b5fb7f))


### Tests

* **bridge withdrawer:** remove unit tests and add blackbox tests ([#1232](https://github.com/astriaorg/astria-release-test/issues/1232)) ([8fa15ce](https://github.com/astriaorg/astria-release-test/commit/8fa15cec0e8b4039470f64ad9ced6442075bc5f6))
* **bridge-withdrawer:** add submitter tests ([#1133](https://github.com/astriaorg/astria-release-test/issues/1133)) ([865f7d7](https://github.com/astriaorg/astria-release-test/commit/865f7d7bb018b80dea32e6ffea25685be8a59416))
</details>

<details><summary>cli: 0.7.0</summary>

## [0.7.0](https://github.com/astriaorg/astria-release-test/compare/cli-vv0.6.0...cli-vv0.7.0) (2025-05-09)


###   BREAKING CHANGES

* **sequencer:** implement `RecoverClient` action  ([#2008](https://github.com/astriaorg/astria-release-test/issues/2008))
* **cli:** remove default chain ID, endpoint URL ([#1792](https://github.com/astriaorg/astria-release-test/issues/1792))
* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825))
* **bridge-withdrawer, cli:** use decimals from erc20 contract ([#1762](https://github.com/astriaorg/astria-release-test/issues/1762))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672))
* **proto:** call transactions `Transaction`, contents `TransactionBody` ([#1650](https://github.com/astriaorg/astria-release-test/issues/1650))
* **proto, core:** remove action suffix from all action types ([#1630](https://github.com/astriaorg/astria-release-test/issues/1630))
* **sequencer:** exclusively use Borsh encoding for stored data (ENG-768) ([#1492](https://github.com/astriaorg/astria-release-test/issues/1492))
* **sequencer:** transaction categories on UnsignedTransaction ([#1512](https://github.com/astriaorg/astria-release-test/issues/1512))
* **proto, core, sequencer:** permit bech32 compatible addresses ([#1425](https://github.com/astriaorg/astria-release-test/issues/1425))
* **cli, bridge-withdrawer:** share code between cli and service ([#1270](https://github.com/astriaorg/astria-release-test/issues/1270))
* **cli:** remove unmaintained rollup subcommand ([#1235](https://github.com/astriaorg/astria-release-test/issues/1235))
* **proto, sequencer:** use full IBC ICS20 denoms instead of IDs ([#1209](https://github.com/astriaorg/astria-release-test/issues/1209))
* **sequencer:** allow configuring base address prefix ([#1201](https://github.com/astriaorg/astria-release-test/issues/1201))
* **sequencer:** implement bridge sudo and withdrawer addresses ([#1142](https://github.com/astriaorg/astria-release-test/issues/1142))
* **core, proto:** add bech32m addresses ([#1124](https://github.com/astriaorg/astria-release-test/issues/1124))

### Features

* **bridge-withdrawer:** add `use_compat_address` configuration value ([#1671](https://github.com/astriaorg/astria-release-test/issues/1671)) ([49e2d9f](https://github.com/astriaorg/astria-release-test/commit/49e2d9f0f87156ea17b499c78fe696c5eebc766f)), closes [#1424](https://github.com/astriaorg/astria-release-test/issues/1424)
* **cli, bridge-withdrawer:** share code between cli and service ([#1270](https://github.com/astriaorg/astria-release-test/issues/1270)) ([75ac37a](https://github.com/astriaorg/astria-release-test/commit/75ac37a5009f7fbe4105a05c7bda2878bb56ea4e))
* **cli:** add cmd to collect withdrawal events and submit as actions ([#1261](https://github.com/astriaorg/astria-release-test/issues/1261)) ([1555c03](https://github.com/astriaorg/astria-release-test/commit/1555c0328b0f964c2476bc29741e7ea322d96f19))
* **cli:** add command to perform ics20 withdrawals ([#1631](https://github.com/astriaorg/astria-release-test/issues/1631)) ([6b25c19](https://github.com/astriaorg/astria-release-test/commit/6b25c191aadfe47d480bb10d1353323cda1f78eb))
* **cli:** add ibc sudo address change command ([#1749](https://github.com/astriaorg/astria-release-test/issues/1749)) ([1ac8458](https://github.com/astriaorg/astria-release-test/commit/1ac8458f35295b728c89b9bdcb2dffa6ba8587f9))
* **cli:** allowed fee-assets query ([#1816](https://github.com/astriaorg/astria-release-test/issues/1816)) ([471ee91](https://github.com/astriaorg/astria-release-test/commit/471ee91197085047243a9cd2d90379254f216738)), closes [#1815](https://github.com/astriaorg/astria-release-test/issues/1815)
* **cli:** bridge sudo change command ([#1612](https://github.com/astriaorg/astria-release-test/issues/1612)) ([1a69dc8](https://github.com/astriaorg/astria-release-test/commit/1a69dc8ac1f763c0fd3cb919d6b92bde0f2af8b4))
* **cli:** implement frost_ed25519 threshold signing cli ([#1654](https://github.com/astriaorg/astria-release-test/issues/1654)) ([af23511](https://github.com/astriaorg/astria-release-test/commit/af2351173b752ca2eef74ca731598f5c19ae9526))
* **cli:** remove default chain ID, endpoint URL ([#1792](https://github.com/astriaorg/astria-release-test/issues/1792)) ([88adcb3](https://github.com/astriaorg/astria-release-test/commit/88adcb33e36a4b411b6f3432f33e7ce0be61e940))
* **cli:** update for new charts, dusk-7 endpoints ([#1103](https://github.com/astriaorg/astria-release-test/issues/1103)) ([74f992e](https://github.com/astriaorg/astria-release-test/commit/74f992ea23e282192fa7a17d5fa56c1dd40fd365))
* **core, proto:** add bech32m addresses ([#1124](https://github.com/astriaorg/astria-release-test/issues/1124)) ([ab8705f](https://github.com/astriaorg/astria-release-test/commit/ab8705f2e0273a158db5ea5248fe0b331a818c8a))
* **proto, core, sequencer:** permit bech32 compatible addresses ([#1425](https://github.com/astriaorg/astria-release-test/issues/1425)) ([0ed3c19](https://github.com/astriaorg/astria-release-test/commit/0ed3c190651e4a2c07ffe304af95ff5756d13e7d))
* **proto, sequencer:** use full IBC ICS20 denoms instead of IDs ([#1209](https://github.com/astriaorg/astria-release-test/issues/1209)) ([f05e829](https://github.com/astriaorg/astria-release-test/commit/f05e8297a4a9ac7d1e1d4f1a3edc266e62b23ddb))
* **sequencer-utils:** add subcommand to parse Celestia blob data ([#1059](https://github.com/astriaorg/astria-release-test/issues/1059)) ([6da485c](https://github.com/astriaorg/astria-release-test/commit/6da485cde09013820c4f627cbee3c4ff4d6214d0))
* **sequencer:** allow configuring base address prefix ([#1201](https://github.com/astriaorg/astria-release-test/issues/1201)) ([d35271d](https://github.com/astriaorg/astria-release-test/commit/d35271dfb4e9cfa9c8b5f2da8fe1ddfd0f3cbdd3))
* **sequencer:** implement `RecoverClient` action  ([#2008](https://github.com/astriaorg/astria-release-test/issues/2008)) ([2ae3b64](https://github.com/astriaorg/astria-release-test/commit/2ae3b64e5f57302eee522518e6bf0336eef08fb1))
* **sequencer:** implement bridge sudo and withdrawer addresses ([#1142](https://github.com/astriaorg/astria-release-test/issues/1142)) ([29baa40](https://github.com/astriaorg/astria-release-test/commit/29baa40341aa12769817450310cc9d4c52429503))
* **sequencer:** transaction categories on UnsignedTransaction ([#1512](https://github.com/astriaorg/astria-release-test/issues/1512)) ([17e6711](https://github.com/astriaorg/astria-release-test/commit/17e6711ce4032930519660f70a9e09af1dea90f7))


### Bug Fixes

* **bridge-withdrawer, cli, sequencer-client:** migrate from `broadcast_tx_commit` to `broadcast_tx_sync` ([#1376](https://github.com/astriaorg/astria-release-test/issues/1376)) ([64b9106](https://github.com/astriaorg/astria-release-test/commit/64b91061a159d7c919bf2f020c29e1cf514d8843))
* **bridge-withdrawer, cli:** use decimals from erc20 contract ([#1762](https://github.com/astriaorg/astria-release-test/issues/1762)) ([ca72c64](https://github.com/astriaorg/astria-release-test/commit/ca72c64a6513dff0c55aaa14c2a5d7d683ab8370))
* **bridge-withdrawer, cli:** use leading channel ([#1768](https://github.com/astriaorg/astria-release-test/issues/1768)) ([1f1d577](https://github.com/astriaorg/astria-release-test/commit/1f1d5770fd31d9fd58a2fb8de5bba249e5b35669))
* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825)) ([fc10a63](https://github.com/astriaorg/astria-release-test/commit/fc10a63a82d2854420271f3b03268e31e40b1cd7))
* **cli, bridge-withdrawer:** dont fail entire block due to bad withdraw event ([#1409](https://github.com/astriaorg/astria-release-test/issues/1409)) ([3e24c50](https://github.com/astriaorg/astria-release-test/commit/3e24c50fc1daec666a51e93756268512fb098182))
* **cli, tests:** add force flag to overwrite withdrawal target path ([#1393](https://github.com/astriaorg/astria-release-test/issues/1393)) ([dbf6991](https://github.com/astriaorg/astria-release-test/commit/dbf6991b7ba981e0c7fd1669f78646bd725e223c))
* **cli:** ensure checkTx passes before waiting for inclusion ([#1636](https://github.com/astriaorg/astria-release-test/issues/1636)) ([237d0c2](https://github.com/astriaorg/astria-release-test/commit/237d0c2e509ea4aa14cc889f37e7691b219881a7)), closes [#1635](https://github.com/astriaorg/astria-release-test/issues/1635)
* **cli:** fix tx submission result check ([#1113](https://github.com/astriaorg/astria-release-test/issues/1113)) ([3473703](https://github.com/astriaorg/astria-release-test/commit/34737036604095e539584bf9d08f890d1b24cff0))
* **core, sequencer:** prefix removal source non-refund ics20 packet ([#1162](https://github.com/astriaorg/astria-release-test/issues/1162)) ([6c29d39](https://github.com/astriaorg/astria-release-test/commit/6c29d39e89ead4fe082962377ae02976588a33b8))


### Miscellaneous

* add `clippy::arithmetic-side-effects` lint and fix resulting warnings ([#1081](https://github.com/astriaorg/astria-release-test/issues/1081)) ([ab00633](https://github.com/astriaorg/astria-release-test/commit/ab00633808dba175e0bc5e1fd8712f81a56c6541))
* **all:** add changelogs ([#1700](https://github.com/astriaorg/astria-release-test/issues/1700)) ([4f7e53a](https://github.com/astriaorg/astria-release-test/commit/4f7e53a7da874e7b198c102da74da54729999e7a))
* **all:** Migrate all instances of `#[allow]` to `#[expect]` ([#1561](https://github.com/astriaorg/astria-release-test/issues/1561)) ([eced579](https://github.com/astriaorg/astria-release-test/commit/eced5797ead1ee6bd094d3574fe61cdad04e5702)), closes [#1521](https://github.com/astriaorg/astria-release-test/issues/1521)
* **all:** remove `once_cell` ([#1576](https://github.com/astriaorg/astria-release-test/issues/1576)) ([3bf4652](https://github.com/astriaorg/astria-release-test/commit/3bf4652899fd6ab1d5fd6e9caca7369d078bbc40)), closes [#1520](https://github.com/astriaorg/astria-release-test/issues/1520)
* bump to rust version 1.83 ([#1857](https://github.com/astriaorg/astria-release-test/issues/1857)) ([2899049](https://github.com/astriaorg/astria-release-test/commit/2899049bf0dd5bd7ba05927a5daf73ee986a46dc)), closes [#1580](https://github.com/astriaorg/astria-release-test/issues/1580)
* **cli:** add withdrawer address to init bridge account command ([#2055](https://github.com/astriaorg/astria-release-test/issues/2055)) ([a329e9e](https://github.com/astriaorg/astria-release-test/commit/a329e9edd1b3ad168de4e68019db799dc72feab7))
* **cli:** remove unmaintained rollup subcommand ([#1235](https://github.com/astriaorg/astria-release-test/issues/1235)) ([9c969a3](https://github.com/astriaorg/astria-release-test/commit/9c969a335b22bc5169849b1d467da5068742ede3))
* **cli:** remove unused code ([#1723](https://github.com/astriaorg/astria-release-test/issues/1723)) ([406b9d2](https://github.com/astriaorg/astria-release-test/commit/406b9d2b5eb551816aa9ae1df54708391c099622))
* **cli:** remove unused rollup cli code ([#1275](https://github.com/astriaorg/astria-release-test/issues/1275)) ([6999a52](https://github.com/astriaorg/astria-release-test/commit/6999a52d64db2e8ee5403d8c3d38891bbae59b79))
* **cli:** return Bech32m Prefixed Address ([#1621](https://github.com/astriaorg/astria-release-test/issues/1621)) ([36e6089](https://github.com/astriaorg/astria-release-test/commit/36e6089d26224565000969c8498f22e633ba8c13)), closes [#1619](https://github.com/astriaorg/astria-release-test/issues/1619)
* **cli:** Update Command Information ([#1789](https://github.com/astriaorg/astria-release-test/issues/1789)) ([2d27d57](https://github.com/astriaorg/astria-release-test/commit/2d27d57ff97289fe60a88168761d897dcd3f1cb3))
* cut releases ([#2017](https://github.com/astriaorg/astria-release-test/issues/2017)) ([a12efeb](https://github.com/astriaorg/astria-release-test/commit/a12efeb0e4000d8ac2adc4e70ced4954cfbbb94c))
* fix typos ([#1041](https://github.com/astriaorg/astria-release-test/issues/1041)) ([3654816](https://github.com/astriaorg/astria-release-test/commit/3654816a921411f8b9214de8af8430709618ad56))
* **proto, core:** remove action suffix from all action types ([#1630](https://github.com/astriaorg/astria-release-test/issues/1630)) ([3abd40a](https://github.com/astriaorg/astria-release-test/commit/3abd40ab2ecee5a425ff592859bf8ae8fd2c4a97))
* **proto:** call transactions `Transaction`, contents `TransactionBody` ([#1650](https://github.com/astriaorg/astria-release-test/issues/1650)) ([1c0284e](https://github.com/astriaorg/astria-release-test/commit/1c0284edd1cb2897ad7528ee96d147781cb354f9))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672)) ([812960f](https://github.com/astriaorg/astria-release-test/commit/812960f713d07d7aeed479c5e805d6238fe20312))
* provide newtypes for verification key and signature ([#1111](https://github.com/astriaorg/astria-release-test/issues/1111)) ([6b91329](https://github.com/astriaorg/astria-release-test/commit/6b91329e0267cbb164bd14d5208f68014e4251fe))
* remove redundant bin entries from all crates' Cargo.toml ([#1725](https://github.com/astriaorg/astria-release-test/issues/1725)) ([8d9aae4](https://github.com/astriaorg/astria-release-test/commit/8d9aae4027ac4c0eb6758f2fb620e5e378f5e76b))
* rename all modules named `test` to `tests` ([#1578](https://github.com/astriaorg/astria-release-test/issues/1578)) ([70046bd](https://github.com/astriaorg/astria-release-test/commit/70046bd984c43fc2c0b505acf2cfefec24f1e2c7))
* **sequencer:** exclusively use Borsh encoding for stored data (ENG-768) ([#1492](https://github.com/astriaorg/astria-release-test/issues/1492)) ([6d9eb28](https://github.com/astriaorg/astria-release-test/commit/6d9eb288efc071402078db258f9146b93e1918c4))
* update `url` dependency ([#1869](https://github.com/astriaorg/astria-release-test/issues/1869)) ([6e91760](https://github.com/astriaorg/astria-release-test/commit/6e91760cd67832db997c1534b5dc0394d7d0d113))


### Code Refactoring

* **cli:** merge argument parsing and command execution ([#1568](https://github.com/astriaorg/astria-release-test/issues/1568)) ([29a5d19](https://github.com/astriaorg/astria-release-test/commit/29a5d197c01745a852031cb57f2cebefb9a26c30))
* **core:** make crypto module into crate ([#1800](https://github.com/astriaorg/astria-release-test/issues/1800)) ([401dfb5](https://github.com/astriaorg/astria-release-test/commit/401dfb5de3360a7bfa012f4908560fc936559637))
* **core:** parse ics20 denoms as ibc or trace prefixed variants ([#1181](https://github.com/astriaorg/astria-release-test/issues/1181)) ([616dd9a](https://github.com/astriaorg/astria-release-test/commit/616dd9a9a209406db11c545336e9b578035bb208))
* **sequencer:** remove mint module ([#1134](https://github.com/astriaorg/astria-release-test/issues/1134)) ([35d69a6](https://github.com/astriaorg/astria-release-test/commit/35d69a64467a555b305a2a20bc28e084b2536082))
</details>

<details><summary>composer: 2.0.0</summary>

## [2.0.0](https://github.com/astriaorg/astria-release-test/compare/composer-vv1.0.1...composer-vv2.0.0) (2025-05-09)


###   BREAKING CHANGES

* **sequencer, core, conductor:** merge upgrades/price-feed feature branch to main ([#2085](https://github.com/astriaorg/astria-release-test/issues/2085))
* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672))
* **proto:** rename sequence action to rollup data submission ([#1665](https://github.com/astriaorg/astria-release-test/issues/1665))
* **proto:** call transactions `Transaction`, contents `TransactionBody` ([#1650](https://github.com/astriaorg/astria-release-test/issues/1650))
* **proto:** prefer astria.primitive.v1.RollupId over bytes ([#1661](https://github.com/astriaorg/astria-release-test/issues/1661))
* **composer:** update to work with appside mempool ([#1643](https://github.com/astriaorg/astria-release-test/issues/1643))
* **sequencer:** enforce block ordering by transaction group  ([#1618](https://github.com/astriaorg/astria-release-test/issues/1618))
* **proto, core:** remove action suffix from all action types ([#1630](https://github.com/astriaorg/astria-release-test/issues/1630))
* **sequencer:** exclusively use Borsh encoding for stored data (ENG-768) ([#1492](https://github.com/astriaorg/astria-release-test/issues/1492))
* **sequencer:** transaction categories on UnsignedTransaction ([#1512](https://github.com/astriaorg/astria-release-test/issues/1512))
* **proto, core, sequencer:** add traceability to rollup deposits ([#1410](https://github.com/astriaorg/astria-release-test/issues/1410))
* **composer:** Add chain_id check on executor build ([#1175](https://github.com/astriaorg/astria-release-test/issues/1175))
* **proto, sequencer:** use full IBC ICS20 denoms instead of IDs ([#1209](https://github.com/astriaorg/astria-release-test/issues/1209))
* **sequencer:** allow configuring base address prefix ([#1201](https://github.com/astriaorg/astria-release-test/issues/1201))
* **core, proto:** add bech32m addresses ([#1124](https://github.com/astriaorg/astria-release-test/issues/1124))
* **sequencer-relayer:** provide filter for rollups ([#1001](https://github.com/astriaorg/astria-release-test/issues/1001))

### Features

* **bridge-withdrawer:** PoC astria-bridge-withdrawer implementation ([#984](https://github.com/astriaorg/astria-release-test/issues/984)) ([afe4901](https://github.com/astriaorg/astria-release-test/commit/afe4901827d636a51a4c774f2ef4c8ee082db19c))
* **composer:** Add chain_id check on executor build ([#1175](https://github.com/astriaorg/astria-release-test/issues/1175)) ([b8f26d2](https://github.com/astriaorg/astria-release-test/commit/b8f26d2f59d837c15670a5ae900e81758feec2db))
* **composer:** add initial set of metrics ([#932](https://github.com/astriaorg/astria-release-test/issues/932)) ([f2edef9](https://github.com/astriaorg/astria-release-test/commit/f2edef9f1ce130ea6f36f3553b0308ebb9d11c51))
* **core, proto:** add bech32m addresses ([#1124](https://github.com/astriaorg/astria-release-test/issues/1124)) ([ab8705f](https://github.com/astriaorg/astria-release-test/commit/ab8705f2e0273a158db5ea5248fe0b331a818c8a))
* **proto, core, sequencer:** add traceability to rollup deposits ([#1410](https://github.com/astriaorg/astria-release-test/issues/1410)) ([f543222](https://github.com/astriaorg/astria-release-test/commit/f5432228090e794a917b6f0803f3a26dc1609dcc))
* **proto, sequencer:** use full IBC ICS20 denoms instead of IDs ([#1209](https://github.com/astriaorg/astria-release-test/issues/1209)) ([f05e829](https://github.com/astriaorg/astria-release-test/commit/f05e8297a4a9ac7d1e1d4f1a3edc266e62b23ddb))
* **sequencer-relayer:** provide filter for rollups ([#1001](https://github.com/astriaorg/astria-release-test/issues/1001)) ([cb2a35e](https://github.com/astriaorg/astria-release-test/commit/cb2a35ecafb46bada1ccbeac9086ff0f48119faf))
* **sequencer, core, conductor:** merge upgrades/price-feed feature branch to main ([#2085](https://github.com/astriaorg/astria-release-test/issues/2085)) ([9fd1517](https://github.com/astriaorg/astria-release-test/commit/9fd15173da036a3394f3a774df5c72a985e32aee))
* **sequencer, core:** Add fee reporting ([#1305](https://github.com/astriaorg/astria-release-test/issues/1305)) ([7953133](https://github.com/astriaorg/astria-release-test/commit/79531330196249e128cb7f46b2b3e14a95aff464))
* **sequencer:** allow configuring base address prefix ([#1201](https://github.com/astriaorg/astria-release-test/issues/1201)) ([d35271d](https://github.com/astriaorg/astria-release-test/commit/d35271dfb4e9cfa9c8b5f2da8fe1ddfd0f3cbdd3))
* **sequencer:** enforce block ordering by transaction group  ([#1618](https://github.com/astriaorg/astria-release-test/issues/1618)) ([412eebe](https://github.com/astriaorg/astria-release-test/commit/412eebeaaff6850bd8a97683d73062ddd82c45ad))
* **sequencer:** transaction categories on UnsignedTransaction ([#1512](https://github.com/astriaorg/astria-release-test/issues/1512)) ([17e6711](https://github.com/astriaorg/astria-release-test/commit/17e6711ce4032930519660f70a9e09af1dea90f7))
* **telemetry:** register metrics via callback ([#1062](https://github.com/astriaorg/astria-release-test/issues/1062)) ([6ceb3f9](https://github.com/astriaorg/astria-release-test/commit/6ceb3f97503566a47f3bbe6ccfaab7e296848fe7))
* use macro to declare metric constants ([#1129](https://github.com/astriaorg/astria-release-test/issues/1129)) ([fb1d7b8](https://github.com/astriaorg/astria-release-test/commit/fb1d7b86a3bbd98793b294894f1c65c81c1c414e))


### Bug Fixes

* abci error code ([#1280](https://github.com/astriaorg/astria-release-test/issues/1280)) ([7b36af7](https://github.com/astriaorg/astria-release-test/commit/7b36af7fc3b0920a13a1210c7806a9407f91850c))
* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825)) ([fc10a63](https://github.com/astriaorg/astria-release-test/commit/fc10a63a82d2854420271f3b03268e31e40b1cd7))
* **composer:** update to work with appside mempool ([#1643](https://github.com/astriaorg/astria-release-test/issues/1643)) ([acfd370](https://github.com/astriaorg/astria-release-test/commit/acfd3703186efd3a345e3a10e9b8bc7af1becaf0))
* **composer:** use tx hash as hex again ([#1014](https://github.com/astriaorg/astria-release-test/issues/1014)) ([d89fe45](https://github.com/astriaorg/astria-release-test/commit/d89fe45c1f57a9674551ffb9e632990064cd1923))
* **core, sequencer:** prefix removal source non-refund ics20 packet ([#1162](https://github.com/astriaorg/astria-release-test/issues/1162)) ([6c29d39](https://github.com/astriaorg/astria-release-test/commit/6c29d39e89ead4fe082962377ae02976588a33b8))
* **proto:** fix import name mismatch ([#1380](https://github.com/astriaorg/astria-release-test/issues/1380)) ([f69ffe2](https://github.com/astriaorg/astria-release-test/commit/f69ffe22a53b063984c83c5993798e249b39c46d))
* **telemetry:** ensure tracer providers are shut down in all services ([#1098](https://github.com/astriaorg/astria-release-test/issues/1098)) ([691888b](https://github.com/astriaorg/astria-release-test/commit/691888bc5c3daf4dcbb243734f11b88d48569a7e))


### Miscellaneous

* add `clippy::arithmetic-side-effects` lint and fix resulting warnings ([#1081](https://github.com/astriaorg/astria-release-test/issues/1081)) ([ab00633](https://github.com/astriaorg/astria-release-test/commit/ab00633808dba175e0bc5e1fd8712f81a56c6541))
* **all:** add changelogs ([#1700](https://github.com/astriaorg/astria-release-test/issues/1700)) ([4f7e53a](https://github.com/astriaorg/astria-release-test/commit/4f7e53a7da874e7b198c102da74da54729999e7a))
* **all:** Migrate all instances of `#[allow]` to `#[expect]` ([#1561](https://github.com/astriaorg/astria-release-test/issues/1561)) ([eced579](https://github.com/astriaorg/astria-release-test/commit/eced5797ead1ee6bd094d3574fe61cdad04e5702)), closes [#1521](https://github.com/astriaorg/astria-release-test/issues/1521)
* **all:** remove `once_cell` ([#1576](https://github.com/astriaorg/astria-release-test/issues/1576)) ([3bf4652](https://github.com/astriaorg/astria-release-test/commit/3bf4652899fd6ab1d5fd6e9caca7369d078bbc40)), closes [#1520](https://github.com/astriaorg/astria-release-test/issues/1520)
* **bridge-withdrawer, composer:** encode submission payload only once ([#2053](https://github.com/astriaorg/astria-release-test/issues/2053)) ([e6db8ce](https://github.com/astriaorg/astria-release-test/commit/e6db8ce9e8836cbfcec6cf994d77c24ed0648f59)), closes [#2050](https://github.com/astriaorg/astria-release-test/issues/2050)
* bump all major dependencies ([#2007](https://github.com/astriaorg/astria-release-test/issues/2007)) ([3b8c453](https://github.com/astriaorg/astria-release-test/commit/3b8c453f10d2d02f4be934aaaecd9d9ab76c0202))
* bump msrv and run clippy ([#1167](https://github.com/astriaorg/astria-release-test/issues/1167)) ([6902ef3](https://github.com/astriaorg/astria-release-test/commit/6902ef35370e5980a76302fc756e1a9a56af21b5))
* bump to rust version 1.83 ([#1857](https://github.com/astriaorg/astria-release-test/issues/1857)) ([2899049](https://github.com/astriaorg/astria-release-test/commit/2899049bf0dd5bd7ba05927a5daf73ee986a46dc)), closes [#1580](https://github.com/astriaorg/astria-release-test/issues/1580)
* **composer:** Add instrumentation ([#1326](https://github.com/astriaorg/astria-release-test/issues/1326)) ([b311636](https://github.com/astriaorg/astria-release-test/commit/b311636d5cc35b2e9b03b802bc5ae0a1b727a13d))
* **composer:** add missing blackbox tests ([#1834](https://github.com/astriaorg/astria-release-test/issues/1834)) ([c6ca388](https://github.com/astriaorg/astria-release-test/commit/c6ca388d4b40b36b8d95b96afcb6b5f2e4917a22))
* **composer:** propagate errors ([#1838](https://github.com/astriaorg/astria-release-test/issues/1838)) ([9553576](https://github.com/astriaorg/astria-release-test/commit/955357613150f996ac786e2eeaa603d6ea94d268)), closes [#1833](https://github.com/astriaorg/astria-release-test/issues/1833)
* **core, proto:** migrate byte slices from Vec to Bytes ([#1319](https://github.com/astriaorg/astria-release-test/issues/1319)) ([f3ea62e](https://github.com/astriaorg/astria-release-test/commit/f3ea62eaf47035c5936039abf170522092ff2b36)), closes [#674](https://github.com/astriaorg/astria-release-test/issues/674)
* cut releases ([#2017](https://github.com/astriaorg/astria-release-test/issues/2017)) ([a12efeb](https://github.com/astriaorg/astria-release-test/commit/a12efeb0e4000d8ac2adc4e70ced4954cfbbb94c))
* fix typos ([#1041](https://github.com/astriaorg/astria-release-test/issues/1041)) ([3654816](https://github.com/astriaorg/astria-release-test/commit/3654816a921411f8b9214de8af8430709618ad56))
* **metrics:** restrict `metrics` crate usage to `astria-telemetry` ([#1192](https://github.com/astriaorg/astria-release-test/issues/1192)) ([f251316](https://github.com/astriaorg/astria-release-test/commit/f25131683865a8952a9b2cf24b1e541a882b571a))
* **metrics:** update `metric_name` macro to handle a collection of names ([#1163](https://github.com/astriaorg/astria-release-test/issues/1163)) ([53a1ecb](https://github.com/astriaorg/astria-release-test/commit/53a1ecb5afca0ccdbf412674eaca96227d377379))
* **proto, core:** remove action suffix from all action types ([#1630](https://github.com/astriaorg/astria-release-test/issues/1630)) ([3abd40a](https://github.com/astriaorg/astria-release-test/commit/3abd40ab2ecee5a425ff592859bf8ae8fd2c4a97))
* **proto:** call transactions `Transaction`, contents `TransactionBody` ([#1650](https://github.com/astriaorg/astria-release-test/issues/1650)) ([1c0284e](https://github.com/astriaorg/astria-release-test/commit/1c0284edd1cb2897ad7528ee96d147781cb354f9))
* **proto:** prefer astria.primitive.v1.RollupId over bytes ([#1661](https://github.com/astriaorg/astria-release-test/issues/1661)) ([e68413a](https://github.com/astriaorg/astria-release-test/commit/e68413a01aacf54b72a9656bbf893b9c9425e5b8))
* **proto:** rename sequence action to rollup data submission ([#1665](https://github.com/astriaorg/astria-release-test/issues/1665)) ([3ff1696](https://github.com/astriaorg/astria-release-test/commit/3ff1696ab6093a5f131186fd5defe68cf13f22a2))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672)) ([812960f](https://github.com/astriaorg/astria-release-test/commit/812960f713d07d7aeed479c5e805d6238fe20312))
* provide newtypes for verification key and signature ([#1111](https://github.com/astriaorg/astria-release-test/issues/1111)) ([6b91329](https://github.com/astriaorg/astria-release-test/commit/6b91329e0267cbb164bd14d5208f68014e4251fe))
* register all metrics during startup ([#1144](https://github.com/astriaorg/astria-release-test/issues/1144)) ([5f117cb](https://github.com/astriaorg/astria-release-test/commit/5f117cb9148016070297f0a4eb1e1f975fc94e4a))
* remove redundant bin entries from all crates' Cargo.toml ([#1725](https://github.com/astriaorg/astria-release-test/issues/1725)) ([8d9aae4](https://github.com/astriaorg/astria-release-test/commit/8d9aae4027ac4c0eb6758f2fb620e5e378f5e76b))
* **sequencer:** exclusively use Borsh encoding for stored data (ENG-768) ([#1492](https://github.com/astriaorg/astria-release-test/issues/1492)) ([6d9eb28](https://github.com/astriaorg/astria-release-test/commit/6d9eb288efc071402078db258f9146b93e1918c4))
* update `url` dependency ([#1869](https://github.com/astriaorg/astria-release-test/issues/1869)) ([6e91760](https://github.com/astriaorg/astria-release-test/commit/6e91760cd67832db997c1534b5dc0394d7d0d113))
</details>

<details><summary>conductor: 2.0.0</summary>

## [2.0.0](https://github.com/astriaorg/astria-release-test/compare/conductor-vv1.1.0...conductor-vv2.0.0) (2025-05-09)


###   BREAKING CHANGES

* **sequencer, core, conductor:** merge upgrades/price-feed feature branch to main ([#2085](https://github.com/astriaorg/astria-release-test/issues/2085))
* **core:** use newtype sequencer block hashes ([#1884](https://github.com/astriaorg/astria-release-test/issues/1884))
* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672))
* **sequencer:** exclusively use Borsh encoding for stored data (ENG-768) ([#1492](https://github.com/astriaorg/astria-release-test/issues/1492))
* **conductor:** implement chain ID checks ([#1482](https://github.com/astriaorg/astria-release-test/issues/1482))
* **conductor:** support disabled celestia auth ([#1372](https://github.com/astriaorg/astria-release-test/issues/1372))
* **proto:** Change execution API to use primitive RollupId ([#1291](https://github.com/astriaorg/astria-release-test/issues/1291))
* **core:** lowerCamelCase for protobuf json mapping ([#1250](https://github.com/astriaorg/astria-release-test/issues/1250))
* **conductor, proto:** celestia base heights in commitment state ([#1121](https://github.com/astriaorg/astria-release-test/issues/1121))
* **conductor:** rate limit sequencer cometbft requests ([#1068](https://github.com/astriaorg/astria-release-test/issues/1068))
* **conductor, relayer:** batch multiple Sequencer blocks to save on Celestia fees ([#1045](https://github.com/astriaorg/astria-release-test/issues/1045))
* **conductor, relayer:** brotli compress data blobs ([#1006](https://github.com/astriaorg/astria-release-test/issues/1006))
* **sequencer-relayer:** submit blobs directly to celestia app ([#963](https://github.com/astriaorg/astria-release-test/issues/963))

### Features

* **bridge-withdrawer:** PoC astria-bridge-withdrawer implementation ([#984](https://github.com/astriaorg/astria-release-test/issues/984)) ([afe4901](https://github.com/astriaorg/astria-release-test/commit/afe4901827d636a51a4c774f2ef4c8ee082db19c))
* **conductor, proto:** celestia base heights in commitment state ([#1121](https://github.com/astriaorg/astria-release-test/issues/1121)) ([106a81b](https://github.com/astriaorg/astria-release-test/commit/106a81bb9644d7313bee8bb3946bf9d4ee5fc9d9))
* **conductor, relayer:** batch multiple Sequencer blocks to save on Celestia fees ([#1045](https://github.com/astriaorg/astria-release-test/issues/1045)) ([00f6b13](https://github.com/astriaorg/astria-release-test/commit/00f6b13d4b1e657d7da092f8f873d73b05db99b2))
* **conductor, relayer:** brotli compress data blobs ([#1006](https://github.com/astriaorg/astria-release-test/issues/1006)) ([1398555](https://github.com/astriaorg/astria-release-test/commit/13985559c54e7860b8d46bd0e6b18bf4583a3c67))
* **conductor:** add metrics ([#1091](https://github.com/astriaorg/astria-release-test/issues/1091)) ([d54b5bf](https://github.com/astriaorg/astria-release-test/commit/d54b5bfd3279b51a6f4bf5f643351b3737829413))
* **conductor:** add retry to execution API gRPCs ([#1115](https://github.com/astriaorg/astria-release-test/issues/1115)) ([5387149](https://github.com/astriaorg/astria-release-test/commit/5387149274df37a382a21ccf0e431c73deee0c42))
* **conductor:** implement chain ID checks ([#1482](https://github.com/astriaorg/astria-release-test/issues/1482)) ([8a4eafc](https://github.com/astriaorg/astria-release-test/commit/8a4eafc16472376ba8ba716534303cd5cbcebdf8))
* **conductor:** implement restart logic ([#1463](https://github.com/astriaorg/astria-release-test/issues/1463)) ([f11f900](https://github.com/astriaorg/astria-release-test/commit/f11f9000a28a4a2959ee488bfad73b751a4194c9)), closes [#906](https://github.com/astriaorg/astria-release-test/issues/906)
* **conductor:** include sequencer block hash ([#1999](https://github.com/astriaorg/astria-release-test/issues/1999)) ([46a5213](https://github.com/astriaorg/astria-release-test/commit/46a52137bc207ddb22e3f25c1b408938980c3b5d))
* **conductor:** rate limit sequencer cometbft requests ([#1068](https://github.com/astriaorg/astria-release-test/issues/1068)) ([d6b91a8](https://github.com/astriaorg/astria-release-test/commit/d6b91a82dddfbdbce44315355a00eb23461ea10f))
* **conductor:** respect shutdown signals during init ([#1080](https://github.com/astriaorg/astria-release-test/issues/1080)) ([234829f](https://github.com/astriaorg/astria-release-test/commit/234829fe05abebe3c350efe8b55ce0f31146a3b7))
* **conductor:** skip outdated block metadata ([#1120](https://github.com/astriaorg/astria-release-test/issues/1120)) ([9925122](https://github.com/astriaorg/astria-release-test/commit/992512225e748806f403991aae88eb9c0c8b8536))
* **conductor:** support disabled celestia auth ([#1372](https://github.com/astriaorg/astria-release-test/issues/1372)) ([59a615a](https://github.com/astriaorg/astria-release-test/commit/59a615a6a6163c5adae22cbba9d000681ae4ec99))
* **core:** lowerCamelCase for protobuf json mapping ([#1250](https://github.com/astriaorg/astria-release-test/issues/1250)) ([f69306f](https://github.com/astriaorg/astria-release-test/commit/f69306f3e92513cd925cc25b21cc0192c4bc7528))
* **core:** use newtype sequencer block hashes ([#1884](https://github.com/astriaorg/astria-release-test/issues/1884)) ([ffbd008](https://github.com/astriaorg/astria-release-test/commit/ffbd008fb4c2d170db75a794e32c04c37ca533a8))
* **sequencer-relayer:** submit blobs directly to celestia app ([#963](https://github.com/astriaorg/astria-release-test/issues/963)) ([65a22ce](https://github.com/astriaorg/astria-release-test/commit/65a22ce5968d048602eb7137772372b903fdf2b9))
* **sequencer, core, conductor:** merge upgrades/price-feed feature branch to main ([#2085](https://github.com/astriaorg/astria-release-test/issues/2085)) ([9fd1517](https://github.com/astriaorg/astria-release-test/commit/9fd15173da036a3394f3a774df5c72a985e32aee))
* **sequencer:** implement `get_pending_nonce` for sequencer API ([#1073](https://github.com/astriaorg/astria-release-test/issues/1073)) ([23c4d9a](https://github.com/astriaorg/astria-release-test/commit/23c4d9ae8c89f3c6982f5e78244bfad45f561e6d))
* **telemetry:** register metrics via callback ([#1062](https://github.com/astriaorg/astria-release-test/issues/1062)) ([6ceb3f9](https://github.com/astriaorg/astria-release-test/commit/6ceb3f97503566a47f3bbe6ccfaab7e296848fe7))
* use macro to declare metric constants ([#1129](https://github.com/astriaorg/astria-release-test/issues/1129)) ([fb1d7b8](https://github.com/astriaorg/astria-release-test/commit/fb1d7b86a3bbd98793b294894f1c65c81c1c414e))


### Bug Fixes

* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825)) ([fc10a63](https://github.com/astriaorg/astria-release-test/commit/fc10a63a82d2854420271f3b03268e31e40b1cd7))
* **conductor:** don't exit on bad Sequencer connection ([#1076](https://github.com/astriaorg/astria-release-test/issues/1076)) ([8953669](https://github.com/astriaorg/astria-release-test/commit/89536699eb409b74310f95be83d00945e9cd294d))
* **conductor:** don't panic during panic ([#1252](https://github.com/astriaorg/astria-release-test/issues/1252)) ([c7f209e](https://github.com/astriaorg/astria-release-test/commit/c7f209e9474773d36bc6ac15183fc06e19c9dd21))
* **conductor:** fix flaky restart test ([#1575](https://github.com/astriaorg/astria-release-test/issues/1575)) ([790af57](https://github.com/astriaorg/astria-release-test/commit/790af57875c56c087ad605ba2a1c48e16bf83157)), closes [#1566](https://github.com/astriaorg/astria-release-test/issues/1566)
* **conductor:** fix flaky soft_and_firm test ([#1472](https://github.com/astriaorg/astria-release-test/issues/1472)) ([33dae42](https://github.com/astriaorg/astria-release-test/commit/33dae4256f7f6d1f936f9e0de166695be1bf416d)), closes [#1143](https://github.com/astriaorg/astria-release-test/issues/1143)
* **conductor:** only execute firm blocks if firm and soft block numbers match ([#1021](https://github.com/astriaorg/astria-release-test/issues/1021)) ([2945a8d](https://github.com/astriaorg/astria-release-test/commit/2945a8da0adf7d483b6e92bd80afdd906456d4b6))
* **conductor:** remove panic source on shutdown ([#1919](https://github.com/astriaorg/astria-release-test/issues/1919)) ([141988f](https://github.com/astriaorg/astria-release-test/commit/141988f0c0c41dc6ed3fe72e3fe92adb353c826d))
* **conductor:** retry blob fetch on request timeout ([#1061](https://github.com/astriaorg/astria-release-test/issues/1061)) ([6a12eb8](https://github.com/astriaorg/astria-release-test/commit/6a12eb8314e53987c384d8b513a34d1a5f782682))
* **conductor:** update for celestia-node v0.15.0 ([#1367](https://github.com/astriaorg/astria-release-test/issues/1367)) ([90d1cb3](https://github.com/astriaorg/astria-release-test/commit/90d1cb3aa79ac95430312a0a6e9d5a2084a93e13))
* **proto:** Change execution API to use primitive RollupId ([#1291](https://github.com/astriaorg/astria-release-test/issues/1291)) ([4b014f7](https://github.com/astriaorg/astria-release-test/commit/4b014f73defbf60f44acbade8712c36a7c87867a))
* **telemetry:** ensure tracer providers are shut down in all services ([#1098](https://github.com/astriaorg/astria-release-test/issues/1098)) ([691888b](https://github.com/astriaorg/astria-release-test/commit/691888bc5c3daf4dcbb243734f11b88d48569a7e))
* update crossbeam-channel dependency ([#2106](https://github.com/astriaorg/astria-release-test/issues/2106)) ([d45ff85](https://github.com/astriaorg/astria-release-test/commit/d45ff858b06ee7c7d461e3c5df9b6acff0972fd2))


### Miscellaneous

* add `clippy::arithmetic-side-effects` lint and fix resulting warnings ([#1081](https://github.com/astriaorg/astria-release-test/issues/1081)) ([ab00633](https://github.com/astriaorg/astria-release-test/commit/ab00633808dba175e0bc5e1fd8712f81a56c6541))
* **all:** add changelogs ([#1700](https://github.com/astriaorg/astria-release-test/issues/1700)) ([4f7e53a](https://github.com/astriaorg/astria-release-test/commit/4f7e53a7da874e7b198c102da74da54729999e7a))
* **all:** Migrate all instances of `#[allow]` to `#[expect]` ([#1561](https://github.com/astriaorg/astria-release-test/issues/1561)) ([eced579](https://github.com/astriaorg/astria-release-test/commit/eced5797ead1ee6bd094d3574fe61cdad04e5702)), closes [#1521](https://github.com/astriaorg/astria-release-test/issues/1521)
* **all:** remove `once_cell` ([#1576](https://github.com/astriaorg/astria-release-test/issues/1576)) ([3bf4652](https://github.com/astriaorg/astria-release-test/commit/3bf4652899fd6ab1d5fd6e9caca7369d078bbc40)), closes [#1520](https://github.com/astriaorg/astria-release-test/issues/1520)
* **astria-merkle:** add benchmarks ([#1179](https://github.com/astriaorg/astria-release-test/issues/1179)) ([273e50e](https://github.com/astriaorg/astria-release-test/commit/273e50ebf090b50a491ef476ae1ee480cb119ebf))
* bump all major dependencies ([#2007](https://github.com/astriaorg/astria-release-test/issues/2007)) ([3b8c453](https://github.com/astriaorg/astria-release-test/commit/3b8c453f10d2d02f4be934aaaecd9d9ab76c0202))
* bump msrv and run clippy ([#1167](https://github.com/astriaorg/astria-release-test/issues/1167)) ([6902ef3](https://github.com/astriaorg/astria-release-test/commit/6902ef35370e5980a76302fc756e1a9a56af21b5))
* bump to rust version 1.83 ([#1857](https://github.com/astriaorg/astria-release-test/issues/1857)) ([2899049](https://github.com/astriaorg/astria-release-test/commit/2899049bf0dd5bd7ba05927a5daf73ee986a46dc)), closes [#1580](https://github.com/astriaorg/astria-release-test/issues/1580)
* **ci:** bump rust toolchain to 1.81 ([#1523](https://github.com/astriaorg/astria-release-test/issues/1523)) ([4478cb6](https://github.com/astriaorg/astria-release-test/commit/4478cb644990e608a11248d53ca3bae4aad009f1))
* **conductor:** Add instrumentation ([#1330](https://github.com/astriaorg/astria-release-test/issues/1330)) ([293bc5c](https://github.com/astriaorg/astria-release-test/commit/293bc5c1c2189120683a59ad00baad26d6c12291))
* **conductor:** don't return empty tuple ([#1885](https://github.com/astriaorg/astria-release-test/issues/1885)) ([7a0e4d0](https://github.com/astriaorg/astria-release-test/commit/7a0e4d02fa1cfcd4e13acaba4976aad399a3b968)), closes [#1879](https://github.com/astriaorg/astria-release-test/issues/1879)
* **conductor:** release conductor 0.17 ([#1139](https://github.com/astriaorg/astria-release-test/issues/1139)) ([3a968fb](https://github.com/astriaorg/astria-release-test/commit/3a968fb57c4f3c56012d31870810240595bd3d4d))
* **conductor:** send boxed objects over channels ([#1865](https://github.com/astriaorg/astria-release-test/issues/1865)) ([8be7af8](https://github.com/astriaorg/astria-release-test/commit/8be7af869ac739f91a45fb1eb1ec68de61b25717)), closes [#1858](https://github.com/astriaorg/astria-release-test/issues/1858)
* **core, proto:** migrate byte slices from Vec to Bytes ([#1319](https://github.com/astriaorg/astria-release-test/issues/1319)) ([f3ea62e](https://github.com/astriaorg/astria-release-test/commit/f3ea62eaf47035c5936039abf170522092ff2b36)), closes [#674](https://github.com/astriaorg/astria-release-test/issues/674)
* cut releases ([#2017](https://github.com/astriaorg/astria-release-test/issues/2017)) ([a12efeb](https://github.com/astriaorg/astria-release-test/commit/a12efeb0e4000d8ac2adc4e70ced4954cfbbb94c))
* fix typos ([#1041](https://github.com/astriaorg/astria-release-test/issues/1041)) ([3654816](https://github.com/astriaorg/astria-release-test/commit/3654816a921411f8b9214de8af8430709618ad56))
* **metrics:** restrict `metrics` crate usage to `astria-telemetry` ([#1192](https://github.com/astriaorg/astria-release-test/issues/1192)) ([f251316](https://github.com/astriaorg/astria-release-test/commit/f25131683865a8952a9b2cf24b1e541a882b571a))
* **metrics:** update `metric_name` macro to handle a collection of names ([#1163](https://github.com/astriaorg/astria-release-test/issues/1163)) ([53a1ecb](https://github.com/astriaorg/astria-release-test/commit/53a1ecb5afca0ccdbf412674eaca96227d377379))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672)) ([812960f](https://github.com/astriaorg/astria-release-test/commit/812960f713d07d7aeed479c5e805d6238fe20312))
* provide newtypes for verification key and signature ([#1111](https://github.com/astriaorg/astria-release-test/issues/1111)) ([6b91329](https://github.com/astriaorg/astria-release-test/commit/6b91329e0267cbb164bd14d5208f68014e4251fe))
* register all metrics during startup ([#1144](https://github.com/astriaorg/astria-release-test/issues/1144)) ([5f117cb](https://github.com/astriaorg/astria-release-test/commit/5f117cb9148016070297f0a4eb1e1f975fc94e4a))
* remove redundant bin entries from all crates' Cargo.toml ([#1725](https://github.com/astriaorg/astria-release-test/issues/1725)) ([8d9aae4](https://github.com/astriaorg/astria-release-test/commit/8d9aae4027ac4c0eb6758f2fb620e5e378f5e76b))
* remove unused dependencies ([#1174](https://github.com/astriaorg/astria-release-test/issues/1174)) ([d2cdea7](https://github.com/astriaorg/astria-release-test/commit/d2cdea7f77099e181acdbfcabf6464eb8ed3e6bb))
* rename all modules named `test` to `tests` ([#1578](https://github.com/astriaorg/astria-release-test/issues/1578)) ([70046bd](https://github.com/astriaorg/astria-release-test/commit/70046bd984c43fc2c0b505acf2cfefec24f1e2c7))
* **sequencer:** exclusively use Borsh encoding for stored data (ENG-768) ([#1492](https://github.com/astriaorg/astria-release-test/issues/1492)) ([6d9eb28](https://github.com/astriaorg/astria-release-test/commit/6d9eb288efc071402078db258f9146b93e1918c4))
* update `url` dependency ([#1869](https://github.com/astriaorg/astria-release-test/issues/1869)) ([6e91760](https://github.com/astriaorg/astria-release-test/commit/6e91760cd67832db997c1534b5dc0394d7d0d113))


### Code Refactoring

* **conductor, relayer:** remove astria-celestia-client ([#1022](https://github.com/astriaorg/astria-release-test/issues/1022)) ([0bd448c](https://github.com/astriaorg/astria-release-test/commit/0bd448c1f594971cb09f3dcf5f8ea0dff61448a1))
* **conductor:** fetch missing blocks as necessary ([#1054](https://github.com/astriaorg/astria-release-test/issues/1054)) ([8523def](https://github.com/astriaorg/astria-release-test/commit/8523deff99dbc7443f89fdd14e930aabe2f7c944))
* **conductor:** make firm, soft readers subtasks ([#1926](https://github.com/astriaorg/astria-release-test/issues/1926)) ([77d0217](https://github.com/astriaorg/astria-release-test/commit/77d0217a86f4718d57b39381acaa70612da6632d))
* **conductor:** perform signal handling in binary and test shutdown logic ([#1094](https://github.com/astriaorg/astria-release-test/issues/1094)) ([ca9c22b](https://github.com/astriaorg/astria-release-test/commit/ca9c22b6d81726bbdc61ef85633b0dd6855d0299))
</details>

<details><summary>sequencer: 3.0.0</summary>

## [3.0.0](https://github.com/astriaorg/astria-release-test/compare/sequencer-vv2.0.0...sequencer-vv3.0.0) (2025-05-09)


###   BREAKING CHANGES

* **sequencer, core, conductor:** merge upgrades/price-feed feature branch to main ([#2085](https://github.com/astriaorg/astria-release-test/issues/2085))
* **sequencer:** implement `RecoverClient` action  ([#2008](https://github.com/astriaorg/astria-release-test/issues/2008))
* **sequencer:** fix fee storage enum ordering ([#1983](https://github.com/astriaorg/astria-release-test/issues/1983))
* **sequencer, charts:** support uds for abci ([#1877](https://github.com/astriaorg/astria-release-test/issues/1877))
* **sequencer:** implement `BridgeTransfer` action ([#1934](https://github.com/astriaorg/astria-release-test/issues/1934))
* **sequencer:** use bridge address to determine asset in bridge unlock cost estimation instead of signer ([#1905](https://github.com/astriaorg/astria-release-test/issues/1905))
* **core:** use newtype sequencer block hashes ([#1884](https://github.com/astriaorg/astria-release-test/issues/1884))
* **sequencer:** fix fungible token packet data import ([#1880](https://github.com/astriaorg/astria-release-test/issues/1880))
* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825))
* **sequencer:** ensure deposits use trace-prefixed assets ([#1807](https://github.com/astriaorg/astria-release-test/issues/1807))
* **core, sequencer:** make native asset optional ([#1703](https://github.com/astriaorg/astria-release-test/issues/1703))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672))
* **proto:** rename sequence action to rollup data submission ([#1665](https://github.com/astriaorg/astria-release-test/issues/1665))
* **sequencer:** allow compat prefixed addresses when receiving ics20 transfers ([#1655](https://github.com/astriaorg/astria-release-test/issues/1655))
* **proto:** call transactions `Transaction`, contents `TransactionBody` ([#1650](https://github.com/astriaorg/astria-release-test/issues/1650))
* **proto:** prefer astria.primitive.v1.RollupId over bytes ([#1661](https://github.com/astriaorg/astria-release-test/issues/1661))
* **sequencer:** rework all fees ([#1647](https://github.com/astriaorg/astria-release-test/issues/1647))
* **sequencer:** enforce block ordering by transaction group  ([#1618](https://github.com/astriaorg/astria-release-test/issues/1618))
* **sequencer:** update storage keys locations and values (ENG-898) ([#1616](https://github.com/astriaorg/astria-release-test/issues/1616))
* **proto, core:** remove action suffix from all action types ([#1630](https://github.com/astriaorg/astria-release-test/issues/1630))
* **sequencer:** add limit to total amount of transactions in parked  ([#1638](https://github.com/astriaorg/astria-release-test/issues/1638))
* **sequencer:** rewrite check_tx to be more efficient and fix regression ([#1515](https://github.com/astriaorg/astria-release-test/issues/1515))
* **sequencer:** exclusively use Borsh encoding for stored data (ENG-768) ([#1492](https://github.com/astriaorg/astria-release-test/issues/1492))
* **sequencer:** transaction categories on UnsignedTransaction ([#1512](https://github.com/astriaorg/astria-release-test/issues/1512))
* **sequencer:** put blocks and deposits to non-verified storage (ENG-812) ([#1525](https://github.com/astriaorg/astria-release-test/issues/1525))
* **sequencer:** change `Deposit` byte length calculation ([#1507](https://github.com/astriaorg/astria-release-test/issues/1507))
* **sequencer:** Add IBC sudo change action ([#1509](https://github.com/astriaorg/astria-release-test/issues/1509))
* **proto, core, sequencer:** add traceability to rollup deposits ([#1410](https://github.com/astriaorg/astria-release-test/issues/1410))
* **proto, core, sequencer:** permit bech32 compatible addresses ([#1425](https://github.com/astriaorg/astria-release-test/issues/1425))
* **sequencer, bridge-withdrawer:** enforce withdrawals consumed ([#1391](https://github.com/astriaorg/astria-release-test/issues/1391))
* **core, proto:** define app genesis state in proto ([#1346](https://github.com/astriaorg/astria-release-test/issues/1346))
* **sequencer:** fix block fees ([#1343](https://github.com/astriaorg/astria-release-test/issues/1343))
* **core, sequencer:** require that bridge unlock address always be set ([#1339](https://github.com/astriaorg/astria-release-test/issues/1339))
* **sequencer:** take funds from bridge in ics20 withdrawals ([#1344](https://github.com/astriaorg/astria-release-test/issues/1344))
* **sequencer:** fix TOCTOU issues by merging check and execution ([#1332](https://github.com/astriaorg/astria-release-test/issues/1332))
* **core, proto:** define bridge memos in proto ([#1285](https://github.com/astriaorg/astria-release-test/issues/1285))
* **core, bridge, sequencer:** dismabiguate return addresses ([#1266](https://github.com/astriaorg/astria-release-test/issues/1266))
* **sequencer:** add metrics ([#1248](https://github.com/astriaorg/astria-release-test/issues/1248))
* **core, proto:** make bridge unlock memo string ([#1244](https://github.com/astriaorg/astria-release-test/issues/1244))
* **sequencer:** store native asset ibc->trace mapping in init_chain ([#1242](https://github.com/astriaorg/astria-release-test/issues/1242))
* **proto, sequencer:** use full IBC ICS20 denoms instead of IDs ([#1209](https://github.com/astriaorg/astria-release-test/issues/1209))
* **sequencer:** allow configuring base address prefix ([#1201](https://github.com/astriaorg/astria-release-test/issues/1201))
* **sequencer:** implement `Ics20TransferDepositMemo` format for incoming ics20 transfers to bridge accounts ([#1202](https://github.com/astriaorg/astria-release-test/issues/1202))
* **sequencer:** implement bridge sudo and withdrawer addresses ([#1142](https://github.com/astriaorg/astria-release-test/issues/1142))
* **core, proto:** add bech32m addresses ([#1124](https://github.com/astriaorg/astria-release-test/issues/1124))
* **sequencer:** fees go to sudo poa ([#1104](https://github.com/astriaorg/astria-release-test/issues/1104))
* **sequencer:** implement `FeeChangeAction` for the authority component ([#1037](https://github.com/astriaorg/astria-release-test/issues/1037))
* **sequencer:** implement bridge unlock action and derestrict transfers   ([#1034](https://github.com/astriaorg/astria-release-test/issues/1034))
* **sequencer:** add fees to genesis state ([#1055](https://github.com/astriaorg/astria-release-test/issues/1055))

### Features

* **auctioneer:** add new service ([#1839](https://github.com/astriaorg/astria-release-test/issues/1839)) ([2db84ed](https://github.com/astriaorg/astria-release-test/commit/2db84ed46430013afa24ee955fe2a24f19d30675))
* **bridge-withdrawer:** sync logic ([#1165](https://github.com/astriaorg/astria-release-test/issues/1165)) ([3d016b2](https://github.com/astriaorg/astria-release-test/commit/3d016b24ef127ba761129cd4ccd5f5587d6d5500))
* **charts:** bridge-withdrawer, smoke test, various chart improvements ([#1141](https://github.com/astriaorg/astria-release-test/issues/1141)) ([b426482](https://github.com/astriaorg/astria-release-test/commit/b42648287224cef7ca58441571b1118895c3c84e))
* **core, proto:** add bech32m addresses ([#1124](https://github.com/astriaorg/astria-release-test/issues/1124)) ([ab8705f](https://github.com/astriaorg/astria-release-test/commit/ab8705f2e0273a158db5ea5248fe0b331a818c8a))
* **core, proto:** make bridge unlock memo string ([#1244](https://github.com/astriaorg/astria-release-test/issues/1244)) ([afd8bcb](https://github.com/astriaorg/astria-release-test/commit/afd8bcb3da295dc7206da05c5a8e37fd7d15a029))
* **core, sequencer:** make native asset optional ([#1703](https://github.com/astriaorg/astria-release-test/issues/1703)) ([3e16986](https://github.com/astriaorg/astria-release-test/commit/3e1698644c3b0f0aa9a1cd69db7ce46d69a0b20d))
* **core:** use newtype sequencer block hashes ([#1884](https://github.com/astriaorg/astria-release-test/issues/1884)) ([ffbd008](https://github.com/astriaorg/astria-release-test/commit/ffbd008fb4c2d170db75a794e32c04c37ca533a8))
* **proto, core, sequencer:** add traceability to rollup deposits ([#1410](https://github.com/astriaorg/astria-release-test/issues/1410)) ([f543222](https://github.com/astriaorg/astria-release-test/commit/f5432228090e794a917b6f0803f3a26dc1609dcc))
* **proto, core, sequencer:** permit bech32 compatible addresses ([#1425](https://github.com/astriaorg/astria-release-test/issues/1425)) ([0ed3c19](https://github.com/astriaorg/astria-release-test/commit/0ed3c190651e4a2c07ffe304af95ff5756d13e7d))
* **proto, sequencer:** use full IBC ICS20 denoms instead of IDs ([#1209](https://github.com/astriaorg/astria-release-test/issues/1209)) ([f05e829](https://github.com/astriaorg/astria-release-test/commit/f05e8297a4a9ac7d1e1d4f1a3edc266e62b23ddb))
* **sequencer-utils:** generate example genesis state ([#1224](https://github.com/astriaorg/astria-release-test/issues/1224)) ([e3e7548](https://github.com/astriaorg/astria-release-test/commit/e3e7548900abcc5cee44376ced8b8942fd14ee3a))
* **sequencer, bridge-withdrawer:** enforce withdrawals consumed ([#1391](https://github.com/astriaorg/astria-release-test/issues/1391)) ([9f61870](https://github.com/astriaorg/astria-release-test/commit/9f618708008e97e63efbe3f1a3a7f31ceb2c39d7))
* **sequencer, charts:** support uds for abci ([#1877](https://github.com/astriaorg/astria-release-test/issues/1877)) ([2151998](https://github.com/astriaorg/astria-release-test/commit/2151998f407d11f5c16ee9759c8538c1938de880))
* **sequencer, core, conductor:** merge upgrades/price-feed feature branch to main ([#2085](https://github.com/astriaorg/astria-release-test/issues/2085)) ([9fd1517](https://github.com/astriaorg/astria-release-test/commit/9fd15173da036a3394f3a774df5c72a985e32aee))
* **sequencer, core:** Add fee reporting ([#1305](https://github.com/astriaorg/astria-release-test/issues/1305)) ([7953133](https://github.com/astriaorg/astria-release-test/commit/79531330196249e128cb7f46b2b3e14a95aff464))
* **sequencer:** add `allowed_fee_asset_ids` abci query and `sequencer_client` support ([#1127](https://github.com/astriaorg/astria-release-test/issues/1127)) ([f0acb1c](https://github.com/astriaorg/astria-release-test/commit/f0acb1c522fd160d7aa303faed779f7bc0c948e4))
* **sequencer:** add fees to genesis state ([#1055](https://github.com/astriaorg/astria-release-test/issues/1055)) ([55f16b9](https://github.com/astriaorg/astria-release-test/commit/55f16b9627c30e69e4c4c47f5b7e3773dbaebc41))
* **sequencer:** Add IBC sudo change action ([#1509](https://github.com/astriaorg/astria-release-test/issues/1509)) ([e945677](https://github.com/astriaorg/astria-release-test/commit/e94567741f18ff3eef6d12660b46c361bd1ffb48)), closes [#1480](https://github.com/astriaorg/astria-release-test/issues/1480)
* **sequencer:** add limit to total amount of transactions in parked  ([#1638](https://github.com/astriaorg/astria-release-test/issues/1638)) ([bcb3b3f](https://github.com/astriaorg/astria-release-test/commit/bcb3b3f08b30c91f5956938ca31a7f40b67683cd))
* **sequencer:** add ttl and invalid cache to app mempool ([#1138](https://github.com/astriaorg/astria-release-test/issues/1138)) ([b6c625c](https://github.com/astriaorg/astria-release-test/commit/b6c625cf9e05e37f910aedd437e52f18d80192b2)), closes [#979](https://github.com/astriaorg/astria-release-test/issues/979)
* **sequencer:** allow configuring base address prefix ([#1201](https://github.com/astriaorg/astria-release-test/issues/1201)) ([d35271d](https://github.com/astriaorg/astria-release-test/commit/d35271dfb4e9cfa9c8b5f2da8fe1ddfd0f3cbdd3))
* **sequencer:** allow querying fee components ([#1748](https://github.com/astriaorg/astria-release-test/issues/1748)) ([e1a4f02](https://github.com/astriaorg/astria-release-test/commit/e1a4f0273fb9e9cb898979a90c9ef31241752d82))
* **sequencer:** enforce block ordering by transaction group  ([#1618](https://github.com/astriaorg/astria-release-test/issues/1618)) ([412eebe](https://github.com/astriaorg/astria-release-test/commit/412eebeaaff6850bd8a97683d73062ddd82c45ad))
* **sequencer:** fees go to sudo poa ([#1104](https://github.com/astriaorg/astria-release-test/issues/1104)) ([d177874](https://github.com/astriaorg/astria-release-test/commit/d1778740cb89851b6f634f2adb56ad18567b5bca))
* **sequencer:** implement `bridge/account_last_tx_hash` abci query ([#1158](https://github.com/astriaorg/astria-release-test/issues/1158)) ([3e22a60](https://github.com/astriaorg/astria-release-test/commit/3e22a60e79c8a3dec53bbcfa1520a969c901f4e5)), closes [#1107](https://github.com/astriaorg/astria-release-test/issues/1107)
* **sequencer:** implement `BridgeTransfer` action ([#1934](https://github.com/astriaorg/astria-release-test/issues/1934)) ([456beb0](https://github.com/astriaorg/astria-release-test/commit/456beb00eb282c674207105090427743971fa658)), closes [#1921](https://github.com/astriaorg/astria-release-test/issues/1921)
* **sequencer:** implement `FeeChangeAction` for the authority component ([#1037](https://github.com/astriaorg/astria-release-test/issues/1037)) ([3daf46d](https://github.com/astriaorg/astria-release-test/commit/3daf46d4caa4c79a105024f03034a59de6dc1e63)), closes [#1003](https://github.com/astriaorg/astria-release-test/issues/1003)
* **sequencer:** implement `get_pending_nonce` for sequencer API ([#1073](https://github.com/astriaorg/astria-release-test/issues/1073)) ([23c4d9a](https://github.com/astriaorg/astria-release-test/commit/23c4d9ae8c89f3c6982f5e78244bfad45f561e6d))
* **sequencer:** implement `Ics20TransferDepositMemo` format for incoming ics20 transfers to bridge accounts ([#1202](https://github.com/astriaorg/astria-release-test/issues/1202)) ([d64d458](https://github.com/astriaorg/astria-release-test/commit/d64d458daa4084811b39e82ab28b43513627f2cf))
* **sequencer:** implement `RecoverClient` action  ([#2008](https://github.com/astriaorg/astria-release-test/issues/2008)) ([2ae3b64](https://github.com/astriaorg/astria-release-test/commit/2ae3b64e5f57302eee522518e6bf0336eef08fb1))
* **sequencer:** implement abci query for bridge account info ([#1189](https://github.com/astriaorg/astria-release-test/issues/1189)) ([a8db883](https://github.com/astriaorg/astria-release-test/commit/a8db88377c0678378188b0e5cc48c74ec4eebf39)), closes [#1118](https://github.com/astriaorg/astria-release-test/issues/1118)
* **sequencer:** implement basic app side mempool with nonce ordering ([#1000](https://github.com/astriaorg/astria-release-test/issues/1000)) ([c3a3021](https://github.com/astriaorg/astria-release-test/commit/c3a3021020714e05f335706c401a671deeab80fd)), closes [#856](https://github.com/astriaorg/astria-release-test/issues/856)
* **sequencer:** implement bridge sudo and withdrawer addresses ([#1142](https://github.com/astriaorg/astria-release-test/issues/1142)) ([29baa40](https://github.com/astriaorg/astria-release-test/commit/29baa40341aa12769817450310cc9d4c52429503))
* **sequencer:** implement bridge unlock action and derestrict transfers   ([#1034](https://github.com/astriaorg/astria-release-test/issues/1034)) ([dbd3aec](https://github.com/astriaorg/astria-release-test/commit/dbd3aec50e233653c7c10123985ead8af68e5cff)), closes [#983](https://github.com/astriaorg/astria-release-test/issues/983)
* **sequencer:** implement refund to rollup logic upon ics20 transfer refund ([#1161](https://github.com/astriaorg/astria-release-test/issues/1161)) ([cd5dbd0](https://github.com/astriaorg/astria-release-test/commit/cd5dbd0cb609fb06d9838fbb9a618c7cde917a97)), closes [#1110](https://github.com/astriaorg/astria-release-test/issues/1110)
* **sequencer:** implement transaction fee query ([#1196](https://github.com/astriaorg/astria-release-test/issues/1196)) ([fc4e76b](https://github.com/astriaorg/astria-release-test/commit/fc4e76b0d959576241a8d783b5517c8ed63eb1ed)), closes [#1071](https://github.com/astriaorg/astria-release-test/issues/1071)
* **sequencer:** make ABCI response for account balances deterministic ([#1574](https://github.com/astriaorg/astria-release-test/issues/1574)) ([b7be904](https://github.com/astriaorg/astria-release-test/commit/b7be904496dd3278190f5cc6b5c3e9440d65e748)), closes [#1451](https://github.com/astriaorg/astria-release-test/issues/1451)
* **sequencer:** make fees optional at genesis ([#1664](https://github.com/astriaorg/astria-release-test/issues/1664)) ([4be57cc](https://github.com/astriaorg/astria-release-test/commit/4be57cc7771d0d022fa788ed6fa48d5a8c921999)), closes [#1662](https://github.com/astriaorg/astria-release-test/issues/1662)
* **sequencer:** make mempool balance aware ([#1408](https://github.com/astriaorg/astria-release-test/issues/1408)) ([b401e4f](https://github.com/astriaorg/astria-release-test/commit/b401e4fb589b25906006338a95eb7899af9606a4))
* **sequencer:** query full denomination from asset ID ([#1067](https://github.com/astriaorg/astria-release-test/issues/1067)) ([1860dec](https://github.com/astriaorg/astria-release-test/commit/1860dec23ed1275abdaeabe6ecb3ec5c73b01ff0)), closes [#1053](https://github.com/astriaorg/astria-release-test/issues/1053)
* **sequencer:** report deposit events ([#1447](https://github.com/astriaorg/astria-release-test/issues/1447)) ([81f2931](https://github.com/astriaorg/astria-release-test/commit/81f293171b0c20ddf435e0d23ca9247fcf963329))
* **sequencer:** rework all fees ([#1647](https://github.com/astriaorg/astria-release-test/issues/1647)) ([b677ce9](https://github.com/astriaorg/astria-release-test/commit/b677ce978130c58562f9ba2167f62c0e4ddb1f7d))
* **sequencer:** rewrite memool to have per-account transaction storage and maintenance  ([#1323](https://github.com/astriaorg/astria-release-test/issues/1323)) ([2ce5fd9](https://github.com/astriaorg/astria-release-test/commit/2ce5fd9812f91a0e55245c945dc197a757558f68))
* **sequencer:** transaction categories on UnsignedTransaction ([#1512](https://github.com/astriaorg/astria-release-test/issues/1512)) ([17e6711](https://github.com/astriaorg/astria-release-test/commit/17e6711ce4032930519660f70a9e09af1dea90f7))
* **telemetry:** register metrics via callback ([#1062](https://github.com/astriaorg/astria-release-test/issues/1062)) ([6ceb3f9](https://github.com/astriaorg/astria-release-test/commit/6ceb3f97503566a47f3bbe6ccfaab7e296848fe7))
* use macro to declare metric constants ([#1129](https://github.com/astriaorg/astria-release-test/issues/1129)) ([fb1d7b8](https://github.com/astriaorg/astria-release-test/commit/fb1d7b86a3bbd98793b294894f1c65c81c1c414e))


### Bug Fixes

* abci error code ([#1280](https://github.com/astriaorg/astria-release-test/issues/1280)) ([7b36af7](https://github.com/astriaorg/astria-release-test/commit/7b36af7fc3b0920a13a1210c7806a9407f91850c))
* **bridge-contracts:** fix memo transaction hash encoding ([#1428](https://github.com/astriaorg/astria-release-test/issues/1428)) ([6b5dae9](https://github.com/astriaorg/astria-release-test/commit/6b5dae9b1f432d452abd848b887ff06c583fb160))
* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825)) ([fc10a63](https://github.com/astriaorg/astria-release-test/commit/fc10a63a82d2854420271f3b03268e31e40b1cd7))
* **core, bridge, sequencer:** dismabiguate return addresses ([#1266](https://github.com/astriaorg/astria-release-test/issues/1266)) ([bb41dc4](https://github.com/astriaorg/astria-release-test/commit/bb41dc4e6bcf64712d1e7dfd8cf31b4f8361741a))
* **core, sequencer:** prefix removal source non-refund ics20 packet ([#1162](https://github.com/astriaorg/astria-release-test/issues/1162)) ([6c29d39](https://github.com/astriaorg/astria-release-test/commit/6c29d39e89ead4fe082962377ae02976588a33b8))
* **proto:** fix import name mismatch ([#1380](https://github.com/astriaorg/astria-release-test/issues/1380)) ([f69ffe2](https://github.com/astriaorg/astria-release-test/commit/f69ffe22a53b063984c83c5993798e249b39c46d))
* remove enable mint entry from example env config ([#1674](https://github.com/astriaorg/astria-release-test/issues/1674)) ([093ba71](https://github.com/astriaorg/astria-release-test/commit/093ba71870c6cf4c9f97f8ce51ecd0d93b55f518))
* **sequencer:** add `end_block` to `app_execute_transaction_with_every_action_snapshot` ([#1455](https://github.com/astriaorg/astria-release-test/issues/1455)) ([665479b](https://github.com/astriaorg/astria-release-test/commit/665479b43549afde96dbc9ca4270d6ae27836659)), closes [#1454](https://github.com/astriaorg/astria-release-test/issues/1454)
* **sequencer:** allow compat prefixed addresses when receiving ics20 transfers ([#1655](https://github.com/astriaorg/astria-release-test/issues/1655)) ([3900b70](https://github.com/astriaorg/astria-release-test/commit/3900b700008075d52630a9c5d195eaf1399609ef)), closes [#1653](https://github.com/astriaorg/astria-release-test/issues/1653)
* **sequencer:** bump penumbra dep to commit with ibc height fix ([#1856](https://github.com/astriaorg/astria-release-test/issues/1856)) ([bbdf4a5](https://github.com/astriaorg/astria-release-test/commit/bbdf4a54b9e51671b0528e804f193d5b57302664))
* **sequencer:** bump penumbra dep to fix ibc state access bug ([#1389](https://github.com/astriaorg/astria-release-test/issues/1389)) ([10b196a](https://github.com/astriaorg/astria-release-test/commit/10b196a57a49d3e96b27a567c2d9b285b9c64828))
* **sequencer:** change how verified consensus state is fetched in `RecoverIbcClient` ([#2037](https://github.com/astriaorg/astria-release-test/issues/2037)) ([a493305](https://github.com/astriaorg/astria-release-test/commit/a4933058520ea42bdc8fbf0cbea075babe0d10a0))
* **sequencer:** ensure deposits use trace-prefixed assets ([#1807](https://github.com/astriaorg/astria-release-test/issues/1807)) ([e67fdb9](https://github.com/astriaorg/astria-release-test/commit/e67fdb949de83365b9a6e6ef0bbbf10482da7f4d)), closes [#1806](https://github.com/astriaorg/astria-release-test/issues/1806)
* **sequencer:** ensure proposal execution uses latest info ([#2024](https://github.com/astriaorg/astria-release-test/issues/2024)) ([1d3d2f4](https://github.com/astriaorg/astria-release-test/commit/1d3d2f4f33c86161af473ba06ca911c8d9f2df2f))
* **sequencer:** fix `just run-cometbft` command ([#1119](https://github.com/astriaorg/astria-release-test/issues/1119)) ([51acfc6](https://github.com/astriaorg/astria-release-test/commit/51acfc6bb765fc93b048a80cb89412fb6efc4378))
* **sequencer:** fix app hash in horcrux sentries ([#1646](https://github.com/astriaorg/astria-release-test/issues/1646)) ([10441af](https://github.com/astriaorg/astria-release-test/commit/10441afacac97c80391c6846bca270040cad977a))
* **sequencer:** fix block fees ([#1343](https://github.com/astriaorg/astria-release-test/issues/1343)) ([781c4c5](https://github.com/astriaorg/astria-release-test/commit/781c4c5c54b654cf4d3906d03ea615f59199eb37))
* **sequencer:** fix fee estimation ([#1701](https://github.com/astriaorg/astria-release-test/issues/1701)) ([cc70df4](https://github.com/astriaorg/astria-release-test/commit/cc70df48608f0aca97fc9a05f916770e364b1fcd))
* **sequencer:** fix fee storage enum ordering ([#1983](https://github.com/astriaorg/astria-release-test/issues/1983)) ([a8f14bb](https://github.com/astriaorg/astria-release-test/commit/a8f14bbcf0adfbdd98e1007aa1004b8892b000be))
* **sequencer:** fix fungible token packet data import ([#1880](https://github.com/astriaorg/astria-release-test/issues/1880)) ([0332384](https://github.com/astriaorg/astria-release-test/commit/0332384c763a05ab65a08e0b469ebe8e9cd7c2b6))
* **sequencer:** fix ibc prefix conversion ([#1065](https://github.com/astriaorg/astria-release-test/issues/1065)) ([a393f3a](https://github.com/astriaorg/astria-release-test/commit/a393f3a77dcfabe3c2b18bb150a0a3166b2eff68))
* **sequencer:** Fix incorrect error message from BridgeUnlock actions ([#1505](https://github.com/astriaorg/astria-release-test/issues/1505)) ([1be156e](https://github.com/astriaorg/astria-release-test/commit/1be156e4818f3234fb0a3f03257f33ca2627d95f)), closes [#1465](https://github.com/astriaorg/astria-release-test/issues/1465)
* **sequencer:** fix TOCTOU issues by merging check and execution ([#1332](https://github.com/astriaorg/astria-release-test/issues/1332)) ([9f959f4](https://github.com/astriaorg/astria-release-test/commit/9f959f4fc492599ffc4a0bfa0d6e29d26b097b4e))
* **sequencer:** improve and fix instrumentation ([#1255](https://github.com/astriaorg/astria-release-test/issues/1255)) ([45a6415](https://github.com/astriaorg/astria-release-test/commit/45a6415bceeb9a52137d6ec909bbe32bfa9d344e))
* **sequencer:** increase mempool removal cache size ([#1969](https://github.com/astriaorg/astria-release-test/issues/1969)) ([2c4da33](https://github.com/astriaorg/astria-release-test/commit/2c4da33d0bc614ab3909870876474acefffe1d12))
* **sequencer:** install astria-eyre hook ([#1552](https://github.com/astriaorg/astria-release-test/issues/1552)) ([c1cc5e5](https://github.com/astriaorg/astria-release-test/commit/c1cc5e5c16767b2149fb4c0b181a7777dc36fbfb)), closes [#1551](https://github.com/astriaorg/astria-release-test/issues/1551)
* **sequencer:** mempool pending nonce calcuations ([#2012](https://github.com/astriaorg/astria-release-test/issues/2012)) ([60bc6f2](https://github.com/astriaorg/astria-release-test/commit/60bc6f200a9d79d167bc6c1e83a1d3947c471971)), closes [#2011](https://github.com/astriaorg/astria-release-test/issues/2011)
* **sequencer:** move fee event recording to tx level ([#1718](https://github.com/astriaorg/astria-release-test/issues/1718)) ([1df9031](https://github.com/astriaorg/astria-release-test/commit/1df903156fb802ac6998afb50d7bde8b768356d5))
* **sequencer:** panic sequencer instead of cometbft on erroring abci consensus requests ([#1016](https://github.com/astriaorg/astria-release-test/issues/1016)) ([f78f6d8](https://github.com/astriaorg/astria-release-test/commit/f78f6d858e314fcc99f24ac87434f3768c49647d)), closes [#1004](https://github.com/astriaorg/astria-release-test/issues/1004)
* **sequencer:** provide context in checktx response log ([#1506](https://github.com/astriaorg/astria-release-test/issues/1506)) ([b12510d](https://github.com/astriaorg/astria-release-test/commit/b12510d53a1b445429277fd5e23b0cc0ff360bb7)), closes [#1464](https://github.com/astriaorg/astria-release-test/issues/1464)
* **sequencer:** remove unwrap from app utilized mempool logic  ([#1772](https://github.com/astriaorg/astria-release-test/issues/1772)) ([8879aaf](https://github.com/astriaorg/astria-release-test/commit/8879aafebec56fcdefb563a543778b1a25fd35c8))
* **sequencer:** rewrite check_tx to be more efficient and fix regression ([#1515](https://github.com/astriaorg/astria-release-test/issues/1515)) ([0238e67](https://github.com/astriaorg/astria-release-test/commit/0238e676bbb0512b1d09c275671b4b006a1ec2be))
* **sequencer:** set current app hash properly when creating app ([#1025](https://github.com/astriaorg/astria-release-test/issues/1025)) ([6fac8bb](https://github.com/astriaorg/astria-release-test/commit/6fac8bb9b6e1b5a3002200a67b332ec82790368d))
* **sequencer:** stateful check now ensures balance for total tx ([#1009](https://github.com/astriaorg/astria-release-test/issues/1009)) ([20d547b](https://github.com/astriaorg/astria-release-test/commit/20d547b00821a4beb807bd60f7087ed8fa3ea6e2)), closes [#786](https://github.com/astriaorg/astria-release-test/issues/786)
* **sequencer:** store native asset ibc-&gt;trace mapping in init_chain ([#1242](https://github.com/astriaorg/astria-release-test/issues/1242)) ([38a034b](https://github.com/astriaorg/astria-release-test/commit/38a034b00fd51a3bebef968a76da77f931994a16))
* **sequencer:** take funds from bridge in ics20 withdrawals ([#1344](https://github.com/astriaorg/astria-release-test/issues/1344)) ([d47a374](https://github.com/astriaorg/astria-release-test/commit/d47a3745a7f3adf8f94d53c86bbcf8378b07be15))
* **sequencer:** update mempool benchmarks (ENG-733) ([#1385](https://github.com/astriaorg/astria-release-test/issues/1385)) ([3a3ace7](https://github.com/astriaorg/astria-release-test/commit/3a3ace70f2023615fa22849bbc5b5d1a22796d27))
* **sequencer:** use bridge address to determine asset in bridge unlock cost estimation instead of signer ([#1905](https://github.com/astriaorg/astria-release-test/issues/1905)) ([5c4feaf](https://github.com/astriaorg/astria-release-test/commit/5c4feafd155c4550d2355c8fd35702907589adfd)), closes [#1904](https://github.com/astriaorg/astria-release-test/issues/1904)
* **telemetry:** ensure tracer providers are shut down in all services ([#1098](https://github.com/astriaorg/astria-release-test/issues/1098)) ([691888b](https://github.com/astriaorg/astria-release-test/commit/691888bc5c3daf4dcbb243734f11b88d48569a7e))
* tendermint-rs 0.40.3 ([#2099](https://github.com/astriaorg/astria-release-test/issues/2099)) ([d6a5dfe](https://github.com/astriaorg/astria-release-test/commit/d6a5dfe8eac2f26295415918226071931b7cead8))


### Miscellaneous

* add `clippy::arithmetic-side-effects` lint and fix resulting warnings ([#1081](https://github.com/astriaorg/astria-release-test/issues/1081)) ([ab00633](https://github.com/astriaorg/astria-release-test/commit/ab00633808dba175e0bc5e1fd8712f81a56c6541))
* **all:** add changelogs ([#1700](https://github.com/astriaorg/astria-release-test/issues/1700)) ([4f7e53a](https://github.com/astriaorg/astria-release-test/commit/4f7e53a7da874e7b198c102da74da54729999e7a))
* **all:** Migrate all instances of `#[allow]` to `#[expect]` ([#1561](https://github.com/astriaorg/astria-release-test/issues/1561)) ([eced579](https://github.com/astriaorg/astria-release-test/commit/eced5797ead1ee6bd094d3574fe61cdad04e5702)), closes [#1521](https://github.com/astriaorg/astria-release-test/issues/1521)
* **all:** remove `once_cell` ([#1576](https://github.com/astriaorg/astria-release-test/issues/1576)) ([3bf4652](https://github.com/astriaorg/astria-release-test/commit/3bf4652899fd6ab1d5fd6e9caca7369d078bbc40)), closes [#1520](https://github.com/astriaorg/astria-release-test/issues/1520)
* bump all major dependencies ([#2007](https://github.com/astriaorg/astria-release-test/issues/2007)) ([3b8c453](https://github.com/astriaorg/astria-release-test/commit/3b8c453f10d2d02f4be934aaaecd9d9ab76c0202))
* bump msrv and run clippy ([#1167](https://github.com/astriaorg/astria-release-test/issues/1167)) ([6902ef3](https://github.com/astriaorg/astria-release-test/commit/6902ef35370e5980a76302fc756e1a9a56af21b5))
* bump penumbra deps ([#1159](https://github.com/astriaorg/astria-release-test/issues/1159)) ([cadfc58](https://github.com/astriaorg/astria-release-test/commit/cadfc588e624d6d452825910d65d2e28ebf08254))
* bump penumbra deps ([#1216](https://github.com/astriaorg/astria-release-test/issues/1216)) ([5a3af08](https://github.com/astriaorg/astria-release-test/commit/5a3af081c47cf8b8b2d91d3eadd030f86ecf5ec1))
* bump to rust version 1.83 ([#1857](https://github.com/astriaorg/astria-release-test/issues/1857)) ([2899049](https://github.com/astriaorg/astria-release-test/commit/2899049bf0dd5bd7ba05927a5daf73ee986a46dc)), closes [#1580](https://github.com/astriaorg/astria-release-test/issues/1580)
* **ci:** bump rust toolchain to 1.81 ([#1523](https://github.com/astriaorg/astria-release-test/issues/1523)) ([4478cb6](https://github.com/astriaorg/astria-release-test/commit/4478cb644990e608a11248d53ca3bae4aad009f1))
* **core, proto:** migrate byte slices from Vec to Bytes ([#1319](https://github.com/astriaorg/astria-release-test/issues/1319)) ([f3ea62e](https://github.com/astriaorg/astria-release-test/commit/f3ea62eaf47035c5936039abf170522092ff2b36)), closes [#674](https://github.com/astriaorg/astria-release-test/issues/674)
* **core, sequencer:** add names to all snapshot tests ([#1690](https://github.com/astriaorg/astria-release-test/issues/1690)) ([7e00378](https://github.com/astriaorg/astria-release-test/commit/7e00378748f6e1149e91a8caed93a0e965b0b9f9)), closes [#1656](https://github.com/astriaorg/astria-release-test/issues/1656)
* **core:** Implement Protobuf trait for tx actions ([#1320](https://github.com/astriaorg/astria-release-test/issues/1320)) ([be139d5](https://github.com/astriaorg/astria-release-test/commit/be139d558f1f57df2f99a2606fc688e889bf4ed9))
* cut releases ([#2017](https://github.com/astriaorg/astria-release-test/issues/2017)) ([a12efeb](https://github.com/astriaorg/astria-release-test/commit/a12efeb0e4000d8ac2adc4e70ced4954cfbbb94c))
* fix typos ([#1041](https://github.com/astriaorg/astria-release-test/issues/1041)) ([3654816](https://github.com/astriaorg/astria-release-test/commit/3654816a921411f8b9214de8af8430709618ad56))
* memoize `address_bytes` of verification key ([#1444](https://github.com/astriaorg/astria-release-test/issues/1444)) ([e9d5f0f](https://github.com/astriaorg/astria-release-test/commit/e9d5f0f0fc2f2449292aeeb07a1f2f7e7227a637))
* **metrics:** restrict `metrics` crate usage to `astria-telemetry` ([#1192](https://github.com/astriaorg/astria-release-test/issues/1192)) ([f251316](https://github.com/astriaorg/astria-release-test/commit/f25131683865a8952a9b2cf24b1e541a882b571a))
* **metrics:** update `metric_name` macro to handle a collection of names ([#1163](https://github.com/astriaorg/astria-release-test/issues/1163)) ([53a1ecb](https://github.com/astriaorg/astria-release-test/commit/53a1ecb5afca0ccdbf412674eaca96227d377379))
* **proto, core:** remove action suffix from all action types ([#1630](https://github.com/astriaorg/astria-release-test/issues/1630)) ([3abd40a](https://github.com/astriaorg/astria-release-test/commit/3abd40ab2ecee5a425ff592859bf8ae8fd2c4a97))
* **proto:** call transactions `Transaction`, contents `TransactionBody` ([#1650](https://github.com/astriaorg/astria-release-test/issues/1650)) ([1c0284e](https://github.com/astriaorg/astria-release-test/commit/1c0284edd1cb2897ad7528ee96d147781cb354f9))
* **proto:** prefer astria.primitive.v1.RollupId over bytes ([#1661](https://github.com/astriaorg/astria-release-test/issues/1661)) ([e68413a](https://github.com/astriaorg/astria-release-test/commit/e68413a01aacf54b72a9656bbf893b9c9425e5b8))
* **proto:** rename sequence action to rollup data submission ([#1665](https://github.com/astriaorg/astria-release-test/issues/1665)) ([3ff1696](https://github.com/astriaorg/astria-release-test/commit/3ff1696ab6093a5f131186fd5defe68cf13f22a2))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672)) ([812960f](https://github.com/astriaorg/astria-release-test/commit/812960f713d07d7aeed479c5e805d6238fe20312))
* provide newtypes for verification key and signature ([#1111](https://github.com/astriaorg/astria-release-test/issues/1111)) ([6b91329](https://github.com/astriaorg/astria-release-test/commit/6b91329e0267cbb164bd14d5208f68014e4251fe))
* register all metrics during startup ([#1144](https://github.com/astriaorg/astria-release-test/issues/1144)) ([5f117cb](https://github.com/astriaorg/astria-release-test/commit/5f117cb9148016070297f0a4eb1e1f975fc94e4a))
* remove redundant bin entries from all crates' Cargo.toml ([#1725](https://github.com/astriaorg/astria-release-test/issues/1725)) ([8d9aae4](https://github.com/astriaorg/astria-release-test/commit/8d9aae4027ac4c0eb6758f2fb620e5e378f5e76b))
* rename all modules named `test` to `tests` ([#1578](https://github.com/astriaorg/astria-release-test/issues/1578)) ([70046bd](https://github.com/astriaorg/astria-release-test/commit/70046bd984c43fc2c0b505acf2cfefec24f1e2c7))
* **sequencer:** add instrumentation ([#1761](https://github.com/astriaorg/astria-release-test/issues/1761)) ([96ee2ed](https://github.com/astriaorg/astria-release-test/commit/96ee2ed467ab8dd582e4cc8f4ffcd0de4880cfab)), closes [#1321](https://github.com/astriaorg/astria-release-test/issues/1321)
* **sequencer:** add mempool benchmarks ([#1238](https://github.com/astriaorg/astria-release-test/issues/1238)) ([269edce](https://github.com/astriaorg/astria-release-test/commit/269edce7eb73a53a1ee2b09ac723c4a20d43253f))
* **sequencer:** add metrics ([#1248](https://github.com/astriaorg/astria-release-test/issues/1248)) ([0fe870c](https://github.com/astriaorg/astria-release-test/commit/0fe870c71ed5694212ed669b4e19d029f0f5b375))
* **sequencer:** add missing storage key tests ([#1989](https://github.com/astriaorg/astria-release-test/issues/1989)) ([71d63b8](https://github.com/astriaorg/astria-release-test/commit/71d63b8252475e2f6cf7193ce3b75a3fdc934fc5)), closes [#1988](https://github.com/astriaorg/astria-release-test/issues/1988)
* **sequencer:** add snapshots for storage values to enforce non-state-breaking changes ([#1985](https://github.com/astriaorg/astria-release-test/issues/1985)) ([dd99bf9](https://github.com/astriaorg/astria-release-test/commit/dd99bf9fcc42d3e4e1e2801a5fe32f2f00b20dfd))
* **sequencer:** avoid hashing proposal data in execution state ([#2044](https://github.com/astriaorg/astria-release-test/issues/2044)) ([373b5c6](https://github.com/astriaorg/astria-release-test/commit/373b5c64b177c5f6b0d818c64060befcacd9db4f))
* **sequencer:** bump penumbra tag ([#1740](https://github.com/astriaorg/astria-release-test/issues/1740)) ([751ce8d](https://github.com/astriaorg/astria-release-test/commit/751ce8d21d4f6a01a2eec6b8bc71918867b011eb))
* **sequencer:** change `Deposit` byte length calculation ([#1507](https://github.com/astriaorg/astria-release-test/issues/1507)) ([c44b45c](https://github.com/astriaorg/astria-release-test/commit/c44b45c3ae8d8be4d1a2959df81cd8df8c5eb866)), closes [#1503](https://github.com/astriaorg/astria-release-test/issues/1503)
* **sequencer:** change name of `source_action_index` to `position_in_transaction` ([#1746](https://github.com/astriaorg/astria-release-test/issues/1746)) ([ab87d35](https://github.com/astriaorg/astria-release-test/commit/ab87d35e350bdf63b7ed85b4d125b869377079c8)), closes [#1735](https://github.com/astriaorg/astria-release-test/issues/1735)
* **sequencer:** change test addresses to versions with known private keys ([#1487](https://github.com/astriaorg/astria-release-test/issues/1487)) ([ca2766f](https://github.com/astriaorg/astria-release-test/commit/ca2766fcedfa32a8a57a3a292cc7dfa5cd8ff15a))
* **sequencer:** consolidate all action handling to one module ([#1759](https://github.com/astriaorg/astria-release-test/issues/1759)) ([d5b9a3d](https://github.com/astriaorg/astria-release-test/commit/d5b9a3d4a12d17a89ae61a4aefec23dacdea72d3)), closes [#1657](https://github.com/astriaorg/astria-release-test/issues/1657)
* **sequencer:** exclusively use Borsh encoding for stored data (ENG-768) ([#1492](https://github.com/astriaorg/astria-release-test/issues/1492)) ([6d9eb28](https://github.com/astriaorg/astria-release-test/commit/6d9eb288efc071402078db258f9146b93e1918c4))
* **sequencer:** fix names to conform to Rust naming conventions ([#1931](https://github.com/astriaorg/astria-release-test/issues/1931)) ([acb3dc1](https://github.com/astriaorg/astria-release-test/commit/acb3dc1f190c67af2942accce624a95dc2d52aa5)), closes [#1930](https://github.com/astriaorg/astria-release-test/issues/1930)
* **sequencer:** fix signing key comments ([#2056](https://github.com/astriaorg/astria-release-test/issues/2056)) ([a8f92b3](https://github.com/astriaorg/astria-release-test/commit/a8f92b3c47fcfc63f61f42d23603b963af181486))
* **sequencer:** index all ABCI events ([#1786](https://github.com/astriaorg/astria-release-test/issues/1786)) ([740d9e7](https://github.com/astriaorg/astria-release-test/commit/740d9e7d3d6d37341ce827bd1dc4ba6997277981))
* **sequencer:** init allowed assets in fees component ([#1730](https://github.com/astriaorg/astria-release-test/issues/1730)) ([37b58da](https://github.com/astriaorg/astria-release-test/commit/37b58da3699b016f174f7b54d895598a5d2f13b6))
* **sequencer:** migrate from `anyhow::Result` to `eyre::Result` ([#1387](https://github.com/astriaorg/astria-release-test/issues/1387)) ([ac7222e](https://github.com/astriaorg/astria-release-test/commit/ac7222e3a8c230ab187ce7387efccd7f39120795)), closes [#1386](https://github.com/astriaorg/astria-release-test/issues/1386)
* **sequencer:** move and improve transaction fee estimation ([#1722](https://github.com/astriaorg/astria-release-test/issues/1722)) ([2777c93](https://github.com/astriaorg/astria-release-test/commit/2777c93599ee8d10f246a3b8190d7017c95418f3)), closes [#1716](https://github.com/astriaorg/astria-release-test/issues/1716)
* **sequencer:** provide more thorough unit testing for actions ([#1916](https://github.com/astriaorg/astria-release-test/issues/1916)) ([d966cb8](https://github.com/astriaorg/astria-release-test/commit/d966cb86a72b74bd3fef7a0e8576fee41c894251)), closes [#1909](https://github.com/astriaorg/astria-release-test/issues/1909)
* **sequencer:** put blocks and deposits to non-verified storage (ENG-812) ([#1525](https://github.com/astriaorg/astria-release-test/issues/1525)) ([bda4ffc](https://github.com/astriaorg/astria-release-test/commit/bda4ffc4a75ebd373aaa2945ac07207ef2bb3e10))
* **sequencer:** refactor app ([#1819](https://github.com/astriaorg/astria-release-test/issues/1819)) ([4ec949c](https://github.com/astriaorg/astria-release-test/commit/4ec949c9d45c917c365a00e3b050d9896eea76d8)), closes [#1785](https://github.com/astriaorg/astria-release-test/issues/1785)
* **sequencer:** refactor bridge checks ([#2010](https://github.com/astriaorg/astria-release-test/issues/2010)) ([a5ef6bc](https://github.com/astriaorg/astria-release-test/commit/a5ef6bcdf44a320db1da7987ebdf64a1612b6ccc))
* **sequencer:** refactor fees ([#1811](https://github.com/astriaorg/astria-release-test/issues/1811)) ([903e1f3](https://github.com/astriaorg/astria-release-test/commit/903e1f3c659479142fe17e2782033e2ec5e31769))
* **sequencer:** refactor mempool ([#1092](https://github.com/astriaorg/astria-release-test/issues/1092)) ([8067367](https://github.com/astriaorg/astria-release-test/commit/806736789d16f81dbedfa74ab78ad4ca09bdf6e7))
* **sequencer:** remove misplaced logs ([#1892](https://github.com/astriaorg/astria-release-test/issues/1892)) ([a19b822](https://github.com/astriaorg/astria-release-test/commit/a19b82243962a89fbdee149cac00e0c3a3d9fefc))
* **sequencer:** remove support for starting networks in post-aspen mode ([#2093](https://github.com/astriaorg/astria-release-test/issues/2093)) ([4758776](https://github.com/astriaorg/astria-release-test/commit/4758776b889edef14749ad58f2750ac190ac08fc))
* **sequencer:** remove unnecessary result return ([#1849](https://github.com/astriaorg/astria-release-test/issues/1849)) ([eac2759](https://github.com/astriaorg/astria-release-test/commit/eac2759e3d5a0bf8d7f67da98ef103df8f286af0)), closes [#1845](https://github.com/astriaorg/astria-release-test/issues/1845)
* **sequencer:** remove unused enable mint env ([#1673](https://github.com/astriaorg/astria-release-test/issues/1673)) ([46c39e8](https://github.com/astriaorg/astria-release-test/commit/46c39e8706cbeb117c46e47408d87bd8ad77d911))
* **sequencer:** separate test utils from shared benchmark and test utils ([#1613](https://github.com/astriaorg/astria-release-test/issues/1613)) ([d9913bb](https://github.com/astriaorg/astria-release-test/commit/d9913bb27990dc0e79a79d43f57d7b6e7e83f1eb)), closes [#1585](https://github.com/astriaorg/astria-release-test/issues/1585)
* **sequencer:** simplify boolean expressions in `transaction container` ([#1595](https://github.com/astriaorg/astria-release-test/issues/1595)) ([6fdf4ea](https://github.com/astriaorg/astria-release-test/commit/6fdf4ea16f22fd45eee7676e4fd92808cdab0ce3)), closes [#1583](https://github.com/astriaorg/astria-release-test/issues/1583)
* **sequencer:** update storage keys locations and values (ENG-898) ([#1616](https://github.com/astriaorg/astria-release-test/issues/1616)) ([bb6c435](https://github.com/astriaorg/astria-release-test/commit/bb6c435d2be3118685ae855df63cc5b5eb9bd5d4))
* **spec:** update sequencer-app docs ([#1682](https://github.com/astriaorg/astria-release-test/issues/1682)) ([ad455f1](https://github.com/astriaorg/astria-release-test/commit/ad455f168ef821b620bed5489bb9e8d9275999bf))
* update `url` dependency ([#1869](https://github.com/astriaorg/astria-release-test/issues/1869)) ([6e91760](https://github.com/astriaorg/astria-release-test/commit/6e91760cd67832db997c1534b5dc0394d7d0d113))


### Code Refactoring

* **core, proto:** define app genesis state in proto ([#1346](https://github.com/astriaorg/astria-release-test/issues/1346)) ([acff940](https://github.com/astriaorg/astria-release-test/commit/acff940abd2dd9857e038323ed6eb8aa88016a87))
* **core, proto:** define bridge memos in proto ([#1285](https://github.com/astriaorg/astria-release-test/issues/1285)) ([892e408](https://github.com/astriaorg/astria-release-test/commit/892e408abcc10adcb842675eec749e91fff5972a))
* **core, sequencer:** require that bridge unlock address always be set ([#1339](https://github.com/astriaorg/astria-release-test/issues/1339)) ([ee31f2d](https://github.com/astriaorg/astria-release-test/commit/ee31f2dec88b238694076a26858300a7dc1604e4)), closes [#1338](https://github.com/astriaorg/astria-release-test/issues/1338)
* **core:** make crypto module into crate ([#1800](https://github.com/astriaorg/astria-release-test/issues/1800)) ([401dfb5](https://github.com/astriaorg/astria-release-test/commit/401dfb5de3360a7bfa012f4908560fc936559637))
* **core:** move address logic to crate ([#1802](https://github.com/astriaorg/astria-release-test/issues/1802)) ([3e85e8d](https://github.com/astriaorg/astria-release-test/commit/3e85e8d99beae2002a392122c87378d25800c142))
* **core:** parse ics20 denoms as ibc or trace prefixed variants ([#1181](https://github.com/astriaorg/astria-release-test/issues/1181)) ([616dd9a](https://github.com/astriaorg/astria-release-test/commit/616dd9a9a209406db11c545336e9b578035bb208))
* **sequencer:** clarify transaction cost estimation ([#1908](https://github.com/astriaorg/astria-release-test/issues/1908)) ([d34fb06](https://github.com/astriaorg/astria-release-test/commit/d34fb06eed31a7781062a1c77052833481fc479c)), closes [#1907](https://github.com/astriaorg/astria-release-test/issues/1907)
* **sequencer:** fix prepare proposal metrics ([#1211](https://github.com/astriaorg/astria-release-test/issues/1211)) ([30c562a](https://github.com/astriaorg/astria-release-test/commit/30c562a46462ffaefd534a8fabd75beb90c74e4c))
* **sequencer:** generate `SequencerBlock` after transaction execution in proposal phase ([#1562](https://github.com/astriaorg/astria-release-test/issues/1562)) ([f4dba95](https://github.com/astriaorg/astria-release-test/commit/f4dba95a0dfa5232f9e795f74447a0641a98d2ae))
* **sequencer:** move asset state methods to asset module ([#1313](https://github.com/astriaorg/astria-release-test/issues/1313)) ([8171eed](https://github.com/astriaorg/astria-release-test/commit/8171eed2934bec693ed462b3be0f52f7df2f45ef)), closes [#1312](https://github.com/astriaorg/astria-release-test/issues/1312)
* **sequencer:** remove global state ([#1317](https://github.com/astriaorg/astria-release-test/issues/1317)) ([c765408](https://github.com/astriaorg/astria-release-test/commit/c765408c97442549db04f7a8c46dd486e4f2be21)), closes [#1208](https://github.com/astriaorg/astria-release-test/issues/1208)
* **sequencer:** remove mint module ([#1134](https://github.com/astriaorg/astria-release-test/issues/1134)) ([35d69a6](https://github.com/astriaorg/astria-release-test/commit/35d69a64467a555b305a2a20bc28e084b2536082))
* **sequencer:** remove unused asset storage variant ([#1704](https://github.com/astriaorg/astria-release-test/issues/1704)) ([5ff422f](https://github.com/astriaorg/astria-release-test/commit/5ff422f7cecd690684f47090bd6cd6395830441c))
* **sequencer:** split run method to eliminate long-lived spans ([#1898](https://github.com/astriaorg/astria-release-test/issues/1898)) ([12b3bd1](https://github.com/astriaorg/astria-release-test/commit/12b3bd1cfa556eed955423ee559242fd77609123)), closes [#1895](https://github.com/astriaorg/astria-release-test/issues/1895) [#1893](https://github.com/astriaorg/astria-release-test/issues/1893)
* **sequencer:** store fees for actions in app state ([#1017](https://github.com/astriaorg/astria-release-test/issues/1017)) ([049ec61](https://github.com/astriaorg/astria-release-test/commit/049ec61f1bc2dc1a3336aa92a7040b1bf765c646))
* **sequencer:** use builder pattern for transaction container tests ([#1592](https://github.com/astriaorg/astria-release-test/issues/1592)) ([8e3a474](https://github.com/astriaorg/astria-release-test/commit/8e3a474cc41cc655892b9d2ee9d8438ad6da27b2))
* **sequencer:** use stream for allowed assets, fix query ([#1710](https://github.com/astriaorg/astria-release-test/issues/1710)) ([f1318b7](https://github.com/astriaorg/astria-release-test/commit/f1318b7434fa763e88081f939f79802fe14a55dd))


### Performance Improvements

* **sequencer:** add benchmark for prepare_proposal (ENG-660) ([#1337](https://github.com/astriaorg/astria-release-test/issues/1337)) ([1f4a359](https://github.com/astriaorg/astria-release-test/commit/1f4a35931233136892a336af16ab30fd4d0f9688))
</details>

<details><summary>sequencer-relayer: 2.0.0</summary>

## [2.0.0](https://github.com/astriaorg/astria-release-test/compare/sequencer-relayer-vv1.0.1...sequencer-relayer-vv2.0.0) (2025-05-09)


###   BREAKING CHANGES

* **sequencer, core, conductor:** merge upgrades/price-feed feature branch to main ([#2085](https://github.com/astriaorg/astria-release-test/issues/2085))
* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672))
* **sequencer-relayer:** minimize resubmissions to Celestia ([#1234](https://github.com/astriaorg/astria-release-test/issues/1234))
* **sequencer-relayer:** remove functionality to restrict relaying blocks to only those proposed by a given validator ([#1168](https://github.com/astriaorg/astria-release-test/issues/1168))
* **sequencer-relayer:** add chain IDs for sequencer and Celestia to config env vars ([#1063](https://github.com/astriaorg/astria-release-test/issues/1063))
* **relayer:** make per submission gauges into histograms ([#1060](https://github.com/astriaorg/astria-release-test/issues/1060))
* **conductor, relayer:** batch multiple Sequencer blocks to save on Celestia fees ([#1045](https://github.com/astriaorg/astria-release-test/issues/1045))
* **sequencer-relayer:** provide filter for rollups ([#1001](https://github.com/astriaorg/astria-release-test/issues/1001))
* **conductor, relayer:** brotli compress data blobs ([#1006](https://github.com/astriaorg/astria-release-test/issues/1006))
* **sequencer-relayer:** submit blobs directly to celestia app ([#963](https://github.com/astriaorg/astria-release-test/issues/963))

### metrics

* **relayer:** make per submission gauges into histograms ([#1060](https://github.com/astriaorg/astria-release-test/issues/1060)) ([19a70cf](https://github.com/astriaorg/astria-release-test/commit/19a70cfa3bbff41c8887162adc74c8f12c1be0a1))


### Features

* **bridge-withdrawer:** PoC astria-bridge-withdrawer implementation ([#984](https://github.com/astriaorg/astria-release-test/issues/984)) ([afe4901](https://github.com/astriaorg/astria-release-test/commit/afe4901827d636a51a4c774f2ef4c8ee082db19c))
* **conductor, relayer:** batch multiple Sequencer blocks to save on Celestia fees ([#1045](https://github.com/astriaorg/astria-release-test/issues/1045)) ([00f6b13](https://github.com/astriaorg/astria-release-test/commit/00f6b13d4b1e657d7da092f8f873d73b05db99b2))
* **conductor, relayer:** brotli compress data blobs ([#1006](https://github.com/astriaorg/astria-release-test/issues/1006)) ([1398555](https://github.com/astriaorg/astria-release-test/commit/13985559c54e7860b8d46bd0e6b18bf4583a3c67))
* **sequencer-relayer:** add chain IDs for sequencer and Celestia to config env vars ([#1063](https://github.com/astriaorg/astria-release-test/issues/1063)) ([e3595cb](https://github.com/astriaorg/astria-release-test/commit/e3595cb3071895255eef803e5d0e9c4bb21e7630))
* **sequencer-relayer:** add metric recording highest sequencer block submitted ([#1040](https://github.com/astriaorg/astria-release-test/issues/1040)) ([aa63189](https://github.com/astriaorg/astria-release-test/commit/aa63189cbf7768bc06cd0f5ff6ea2fa59902d5cf))
* **sequencer-relayer:** provide filter for rollups ([#1001](https://github.com/astriaorg/astria-release-test/issues/1001)) ([cb2a35e](https://github.com/astriaorg/astria-release-test/commit/cb2a35ecafb46bada1ccbeac9086ff0f48119faf))
* **sequencer-relayer:** submit blobs directly to celestia app ([#963](https://github.com/astriaorg/astria-release-test/issues/963)) ([65a22ce](https://github.com/astriaorg/astria-release-test/commit/65a22ce5968d048602eb7137772372b903fdf2b9))
* **sequencer, core, conductor:** merge upgrades/price-feed feature branch to main ([#2085](https://github.com/astriaorg/astria-release-test/issues/2085)) ([9fd1517](https://github.com/astriaorg/astria-release-test/commit/9fd15173da036a3394f3a774df5c72a985e32aee))
* **sequencer:** implement `get_pending_nonce` for sequencer API ([#1073](https://github.com/astriaorg/astria-release-test/issues/1073)) ([23c4d9a](https://github.com/astriaorg/astria-release-test/commit/23c4d9ae8c89f3c6982f5e78244bfad45f561e6d))
* **telemetry:** register metrics via callback ([#1062](https://github.com/astriaorg/astria-release-test/issues/1062)) ([6ceb3f9](https://github.com/astriaorg/astria-release-test/commit/6ceb3f97503566a47f3bbe6ccfaab7e296848fe7))
* use macro to declare metric constants ([#1129](https://github.com/astriaorg/astria-release-test/issues/1129)) ([fb1d7b8](https://github.com/astriaorg/astria-release-test/commit/fb1d7b86a3bbd98793b294894f1c65c81c1c414e))


### Bug Fixes

* **ci, core:** ensure committed and code generated from protobuf spec match ([#1825](https://github.com/astriaorg/astria-release-test/issues/1825)) ([fc10a63](https://github.com/astriaorg/astria-release-test/commit/fc10a63a82d2854420271f3b03268e31e40b1cd7))
* **relayer:** change `reqwest` for `isahc` in relayer blackbox tests (ENG-699) ([#1366](https://github.com/astriaorg/astria-release-test/issues/1366)) ([49452f4](https://github.com/astriaorg/astria-release-test/commit/49452f41dd81ff10b97aeb3149e943d07e355d6b))
* **sequencer-relayer:** avoid hanging while waiting for submitter task to return ([#1206](https://github.com/astriaorg/astria-release-test/issues/1206)) ([2daebe5](https://github.com/astriaorg/astria-release-test/commit/2daebe5f93cdbe5c3da89bdd7442d2e316fe0307))
* **telemetry:** ensure tracer providers are shut down in all services ([#1098](https://github.com/astriaorg/astria-release-test/issues/1098)) ([691888b](https://github.com/astriaorg/astria-release-test/commit/691888bc5c3daf4dcbb243734f11b88d48569a7e))


### Miscellaneous

* add `clippy::arithmetic-side-effects` lint and fix resulting warnings ([#1081](https://github.com/astriaorg/astria-release-test/issues/1081)) ([ab00633](https://github.com/astriaorg/astria-release-test/commit/ab00633808dba175e0bc5e1fd8712f81a56c6541))
* **all:** add changelogs ([#1700](https://github.com/astriaorg/astria-release-test/issues/1700)) ([4f7e53a](https://github.com/astriaorg/astria-release-test/commit/4f7e53a7da874e7b198c102da74da54729999e7a))
* **all:** Migrate all instances of `#[allow]` to `#[expect]` ([#1561](https://github.com/astriaorg/astria-release-test/issues/1561)) ([eced579](https://github.com/astriaorg/astria-release-test/commit/eced5797ead1ee6bd094d3574fe61cdad04e5702)), closes [#1521](https://github.com/astriaorg/astria-release-test/issues/1521)
* **all:** remove `once_cell` ([#1576](https://github.com/astriaorg/astria-release-test/issues/1576)) ([3bf4652](https://github.com/astriaorg/astria-release-test/commit/3bf4652899fd6ab1d5fd6e9caca7369d078bbc40)), closes [#1520](https://github.com/astriaorg/astria-release-test/issues/1520)
* bump all major dependencies ([#2007](https://github.com/astriaorg/astria-release-test/issues/2007)) ([3b8c453](https://github.com/astriaorg/astria-release-test/commit/3b8c453f10d2d02f4be934aaaecd9d9ab76c0202))
* bump msrv and run clippy ([#1167](https://github.com/astriaorg/astria-release-test/issues/1167)) ([6902ef3](https://github.com/astriaorg/astria-release-test/commit/6902ef35370e5980a76302fc756e1a9a56af21b5))
* bump to rust version 1.83 ([#1857](https://github.com/astriaorg/astria-release-test/issues/1857)) ([2899049](https://github.com/astriaorg/astria-release-test/commit/2899049bf0dd5bd7ba05927a5daf73ee986a46dc)), closes [#1580](https://github.com/astriaorg/astria-release-test/issues/1580)
* **core, proto:** migrate byte slices from Vec to Bytes ([#1319](https://github.com/astriaorg/astria-release-test/issues/1319)) ([f3ea62e](https://github.com/astriaorg/astria-release-test/commit/f3ea62eaf47035c5936039abf170522092ff2b36)), closes [#674](https://github.com/astriaorg/astria-release-test/issues/674)
* cut releases ([#2017](https://github.com/astriaorg/astria-release-test/issues/2017)) ([a12efeb](https://github.com/astriaorg/astria-release-test/commit/a12efeb0e4000d8ac2adc4e70ced4954cfbbb94c))
* fix typos ([#1041](https://github.com/astriaorg/astria-release-test/issues/1041)) ([3654816](https://github.com/astriaorg/astria-release-test/commit/3654816a921411f8b9214de8af8430709618ad56))
* **just:** organize just recipes into modules and add comments to all ([#2002](https://github.com/astriaorg/astria-release-test/issues/2002)) ([9bac042](https://github.com/astriaorg/astria-release-test/commit/9bac0422a4c8be8a850f006f14cda7f7441b8fd7)), closes [#1992](https://github.com/astriaorg/astria-release-test/issues/1992)
* **metrics:** restrict `metrics` crate usage to `astria-telemetry` ([#1192](https://github.com/astriaorg/astria-release-test/issues/1192)) ([f251316](https://github.com/astriaorg/astria-release-test/commit/f25131683865a8952a9b2cf24b1e541a882b571a))
* **metrics:** update `metric_name` macro to handle a collection of names ([#1163](https://github.com/astriaorg/astria-release-test/issues/1163)) ([53a1ecb](https://github.com/astriaorg/astria-release-test/commit/53a1ecb5afca0ccdbf412674eaca96227d377379))
* **proto:** upgrade to proto v1s throughout ([#1672](https://github.com/astriaorg/astria-release-test/issues/1672)) ([812960f](https://github.com/astriaorg/astria-release-test/commit/812960f713d07d7aeed479c5e805d6238fe20312))
* register all metrics during startup ([#1144](https://github.com/astriaorg/astria-release-test/issues/1144)) ([5f117cb](https://github.com/astriaorg/astria-release-test/commit/5f117cb9148016070297f0a4eb1e1f975fc94e4a))
* remove redundant bin entries from all crates' Cargo.toml ([#1725](https://github.com/astriaorg/astria-release-test/issues/1725)) ([8d9aae4](https://github.com/astriaorg/astria-release-test/commit/8d9aae4027ac4c0eb6758f2fb620e5e378f5e76b))
* remove unused dependencies ([#1174](https://github.com/astriaorg/astria-release-test/issues/1174)) ([d2cdea7](https://github.com/astriaorg/astria-release-test/commit/d2cdea7f77099e181acdbfcabf6464eb8ed3e6bb))
* **sequencer-relayer:** Add instrumentation ([#1375](https://github.com/astriaorg/astria-release-test/issues/1375)) ([82dec3a](https://github.com/astriaorg/astria-release-test/commit/82dec3a2bec880d33a34e301aabfc8a73beb3428))
* **sequencer-relayer:** add metrics for recording Celestia fees ([#1742](https://github.com/astriaorg/astria-release-test/issues/1742)) ([3c6e456](https://github.com/astriaorg/astria-release-test/commit/3c6e456c23fa9ab0b6aeca5fb5eef7d90931b8ff))
* **sequencer-relayer:** add timeout to gRPCs to Celestia app ([#1191](https://github.com/astriaorg/astria-release-test/issues/1191)) ([f6171b1](https://github.com/astriaorg/astria-release-test/commit/f6171b1d37f839f4b55eb362db339326f2446c3c))
* **sequencer-relayer:** change blob submitter to use boxed blocks ([#1863](https://github.com/astriaorg/astria-release-test/issues/1863)) ([85d356c](https://github.com/astriaorg/astria-release-test/commit/85d356c0575223e66000ba422a7a54661bf1a4db)), closes [#1860](https://github.com/astriaorg/astria-release-test/issues/1860)
* **sequencer-relayer:** change compression ratio calculation ([#1075](https://github.com/astriaorg/astria-release-test/issues/1075)) ([59bdae7](https://github.com/astriaorg/astria-release-test/commit/59bdae76338cbdab0f889f68f5ca0871f51dff95))
* **sequencer-relayer:** minimize resubmissions to Celestia ([#1234](https://github.com/astriaorg/astria-release-test/issues/1234)) ([961294c](https://github.com/astriaorg/astria-release-test/commit/961294c82c2484423653fea1d690f57ec08cf2e8))
* **sequencer-relayer:** remove functionality to restrict relaying blocks to only those proposed by a given validator ([#1168](https://github.com/astriaorg/astria-release-test/issues/1168)) ([381d798](https://github.com/astriaorg/astria-release-test/commit/381d798ef86fb68df0d9b19237a241754f1c0cba))
* update `url` dependency ([#1869](https://github.com/astriaorg/astria-release-test/issues/1869)) ([6e91760](https://github.com/astriaorg/astria-release-test/commit/6e91760cd67832db997c1534b5dc0394d7d0d113))


### Code Refactoring

* **conductor, relayer:** remove astria-celestia-client ([#1022](https://github.com/astriaorg/astria-release-test/issues/1022)) ([0bd448c](https://github.com/astriaorg/astria-release-test/commit/0bd448c1f594971cb09f3dcf5f8ea0dff61448a1))


### Tests

* **sequencer-relayer:** reinstate black box tests ([#1033](https://github.com/astriaorg/astria-release-test/issues/1033)) ([83d23ab](https://github.com/astriaorg/astria-release-test/commit/83d23ab147b473936cb7f0638fee7651a3c33b3c))
</details>

---
This PR was generated with [Release Please](https://github.com/googleapis/release-please). See [documentation](https://github.com/googleapis/release-please#release-please).