<!-- markdownlint-disable no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0](https://github.com/astriaorg/astria-release-test/compare/bridge-withdrawer-vv1.0.2...bridge-withdrawer-vv2.0.0) (2025-05-09)


### ⚠ BREAKING CHANGES

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

## [Unreleased]

## [1.0.2] - 2025-03-06

### Changed

- Update `idna` dependency to resolve cargo audit warning [#1869](https://github.com/astriaorg/astria/pull/1869).
- Support FROST threshold signing using signer nodes. [#1948](https://github.com/astriaorg/astria/pull/1948).

## [1.0.1] - 2024-11-01

### Changed

- Bump MSRV to 1.83.0 [#1857](https://github.com/astriaorg/astria/pull/1857).

### Fixed

- Set `batch_total_settled_value` metric to 0 when no withdrawals are settled [#1778](https://github.com/astriaorg/astria/pull/1768)
- Fixed ICS20 withdrawal source when using channel with more than one
  port/channel combo.[#1768](https://github.com/astriaorg/astria/pull/1768)

## [1.0.0] - 2024-10-25

### Changed

- Bump penumbra dependencies [#1740](https://github.com/astriaorg/astria/pull/1740).

## [1.0.0-rc.2] - 2024-10-23

### Added

- Add `use_compat_address` configuration value [#1671](https://github.com/astriaorg/astria/pull/1671).
- Metric to track total settled funds [#1693](https://github.com/astriaorg/astria/pull/1693).

### Fixed

- Correctly identify rollup return address in ics20 withdrawal actions [#1714](https://github.com/astriaorg/astria/pull/1714).

## [1.0.0-rc.1] - 2024-10-17

### Added

- Add traceability to rollup deposits [#1410](https://github.com/astriaorg/astria/pull/1410).

### Changed

- Pass GRPC and CometBFT clients to consumers directly [#1510](https://github.com/astriaorg/astria/pull/1510).
- Better grpc client construction [#1528](https://github.com/astriaorg/astria/pull/1528).
- Replace `once_cell` with `LazyLock` [#1576](https://github.com/astriaorg/astria/pull/1576).
- Remove action suffix from all action types [#1630](https://github.com/astriaorg/astria/pull/1630).
- Update `futures-util` dependency based on cargo audit warning [#1644](https://github.com/astriaorg/astria/pull/1644).
- Call transactions `Transaction`, contents `TransactionBody` [#1650](https://github.com/astriaorg/astria/pull/1650).
- Rename sequence action to rollup data submission [#1665](https://github.com/astriaorg/astria/pull/1665).
- Upgrade to proto `v1`s throughout [#1672](https://github.com/astriaorg/astria/pull/1672).

### Fixed

- Migrate from `broadcast_tx_commit` to `broadcast_tx_sync` [#1376](https://github.com/astriaorg/astria/pull/1376).
- Fix memo transaction hash encoding [#1428](https://github.com/astriaorg/astria/pull/1428).

## [0.3.0] - 2024-09-06

### Added

- Add instrumentation [#1324](https://github.com/astriaorg/astria/pull/1324).

### Changed

- Enforce withdrawals consumed [#1391](https://github.com/astriaorg/astria/pull/1391).

### Fixed

- Don't fail entire block due to bad withdraw event [#1409](https://github.com/astriaorg/astria/pull/1409).

## [0.2.1] - 2024-08-22

### Changed

- Improve nonce handling [#1292](https://github.com/astriaorg/astria/pull/1292).
- Update `bytemark` dependency based on cargo audit warning [#1350](https://github.com/astriaorg/astria/pull/1350)

## [0.2.0] - 2024-07-26

### Changed

- Move generated contract bindings to crate [#1237](https://github.com/astriaorg/astria/pull/1237).
- Move bridge-unlock memo to core [#1245](https://github.com/astriaorg/astria/pull/1245).
- Refactor startup to a separate subtask and remove balance check from startup [#1190](https://github.com/astriaorg/astria/pull/1190).
- Make bridge unlock memo string [#1244](https://github.com/astriaorg/astria/pull/1244).
- Share code between cli and service [#1270](https://github.com/astriaorg/astria/pull/1270).
- Define bridge memos in proto [#1285](https://github.com/astriaorg/astria/pull/1285).

### Fixed

- Support withdrawer address that differs from bridge address   [#1262](https://github.com/astriaorg/astria/pull/1262).
- Disambiguate return addresses [#1266](https://github.com/astriaorg/astria/pull/1266).
- Fix nonce handling [#1215](https://github.com/astriaorg/astria/pull/1215).
- Don't panic on init [#1281](https://github.com/astriaorg/astria/pull/1281).

## 0.1.0 - 2024-06-27

### Added

- Initial release of EVM Withdrawer.

[unreleased]: https://github.com/astriaorg/astria/compare/bridge-withdrawer-v1.0.2...HEAD
[1.0.2]: https://github.com/astriaorg/astria/compare/bridge-withdrawer-v1.0.1...bridge-withdrawer-v1.0.2
[1.0.1]: https://github.com/astriaorg/astria/compare/bridge-withdrawer-v1.0.0...bridge-withdrawer-v1.0.1
[1.0.0]: https://github.com/astriaorg/astria/compare/bridge-withdrawer-v1.0.0-rc.2...bridge-withdrawer-v1.0.0
[1.0.0-rc.2]: https://github.com/astriaorg/astria/compare/bridge-withdrawer-v1.0.0-rc.1...bridge-withdrawer-v1.0.0-rc.2
[1.0.0-rc.1]: https://github.com/astriaorg/astria/compare/bridge-withdrawer-v0.3.0...bridge-withdrawer-v1.0.0-rc.1
[0.3.0]: https://github.com/astriaorg/astria/compare/bridge-withdrawer-v0.2.1...bridge-withdrawer-v0.3.0
[0.2.1]: https://github.com/astriaorg/astria/compare/bridge-withdrawer-v0.2.0...bridge-withdrawer-v0.2.1
[0.2.0]: https://github.com/astriaorg/astria/compare/bridge-withdrawer-v0.1.0...bridge-withdrawer-v0.2.0
