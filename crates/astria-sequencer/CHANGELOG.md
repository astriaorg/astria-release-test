<!-- markdownlint-disable no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0](https://github.com/astriaorg/astria-release-test/compare/sequencer-vv2.0.0...sequencer-vv3.0.0) (2025-05-16)


### ⚠ BREAKING CHANGES

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

### Features

* **auctioneer:** add new service ([#1839](https://github.com/astriaorg/astria-release-test/issues/1839)) ([2db84ed](https://github.com/astriaorg/astria-release-test/commit/2db84ed46430013afa24ee955fe2a24f19d30675))
* **bridge-withdrawer:** sync logic ([#1165](https://github.com/astriaorg/astria-release-test/issues/1165)) ([3d016b2](https://github.com/astriaorg/astria-release-test/commit/3d016b24ef127ba761129cd4ccd5f5587d6d5500))
* **charts:** bridge-withdrawer, smoke test, various chart improvements ([#1141](https://github.com/astriaorg/astria-release-test/issues/1141)) ([b426482](https://github.com/astriaorg/astria-release-test/commit/b42648287224cef7ca58441571b1118895c3c84e))
* **core, proto:** add bech32m addresses ([#1124](https://github.com/astriaorg/astria-release-test/issues/1124)) ([ab8705f](https://github.com/astriaorg/astria-release-test/commit/ab8705f2e0273a158db5ea5248fe0b331a818c8a))
* **core, proto:** make bridge unlock memo string ([#1244](https://github.com/astriaorg/astria-release-test/issues/1244)) ([afd8bcb](https://github.com/astriaorg/astria-release-test/commit/afd8bcb3da295dc7206da05c5a8e37fd7d15a029))
* **core, sequencer:** make native asset optional ([#1703](https://github.com/astriaorg/astria-release-test/issues/1703)) ([3e16986](https://github.com/astriaorg/astria-release-test/commit/3e1698644c3b0f0aa9a1cd69db7ce46d69a0b20d))
* **core, sequencer:** provide checked actions and transaction ([#2142](https://github.com/astriaorg/astria-release-test/issues/2142)) ([5f428a9](https://github.com/astriaorg/astria-release-test/commit/5f428a93f94f3521cfd49f4368d1b80caabe9bb2))
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
* **sequencer:** Add IBC sudo change action ([#1509](https://github.com/astriaorg/astria-release-test/issues/1509)) ([e945677](https://github.com/astriaorg/astria-release-test/commit/e94567741f18ff3eef6d12660b46c361bd1ffb48)), closes [#1480](https://github.com/astriaorg/astria-release-test/issues/1480)
* **sequencer:** add limit to total amount of transactions in parked  ([#1638](https://github.com/astriaorg/astria-release-test/issues/1638)) ([bcb3b3f](https://github.com/astriaorg/astria-release-test/commit/bcb3b3f08b30c91f5956938ca31a7f40b67683cd))
* **sequencer:** add mempool gRPC capabilities ([#2133](https://github.com/astriaorg/astria-release-test/issues/2133)) ([20c9ad9](https://github.com/astriaorg/astria-release-test/commit/20c9ad9450ad4a6a677f8272ea8c261428cae049)), closes [#2119](https://github.com/astriaorg/astria-release-test/issues/2119)
* **sequencer:** add name to validator update action ([#2108](https://github.com/astriaorg/astria-release-test/issues/2108)) ([b1a0178](https://github.com/astriaorg/astria-release-test/commit/b1a0178832669c39c181432d46864f3db6e1e8ed)), closes [#1590](https://github.com/astriaorg/astria-release-test/issues/1590)
* **sequencer:** add ttl and invalid cache to app mempool ([#1138](https://github.com/astriaorg/astria-release-test/issues/1138)) ([b6c625c](https://github.com/astriaorg/astria-release-test/commit/b6c625cf9e05e37f910aedd437e52f18d80192b2)), closes [#979](https://github.com/astriaorg/astria-release-test/issues/979)
* **sequencer:** allow configuring base address prefix ([#1201](https://github.com/astriaorg/astria-release-test/issues/1201)) ([d35271d](https://github.com/astriaorg/astria-release-test/commit/d35271dfb4e9cfa9c8b5f2da8fe1ddfd0f3cbdd3))
* **sequencer:** allow querying fee components ([#1748](https://github.com/astriaorg/astria-release-test/issues/1748)) ([e1a4f02](https://github.com/astriaorg/astria-release-test/commit/e1a4f0273fb9e9cb898979a90c9ef31241752d82))
* **sequencer:** enforce block ordering by transaction group  ([#1618](https://github.com/astriaorg/astria-release-test/issues/1618)) ([412eebe](https://github.com/astriaorg/astria-release-test/commit/412eebeaaff6850bd8a97683d73062ddd82c45ad))
* **sequencer:** fees go to sudo poa ([#1104](https://github.com/astriaorg/astria-release-test/issues/1104)) ([d177874](https://github.com/astriaorg/astria-release-test/commit/d1778740cb89851b6f634f2adb56ad18567b5bca))
* **sequencer:** implement `bridge/account_last_tx_hash` abci query ([#1158](https://github.com/astriaorg/astria-release-test/issues/1158)) ([3e22a60](https://github.com/astriaorg/astria-release-test/commit/3e22a60e79c8a3dec53bbcfa1520a969c901f4e5)), closes [#1107](https://github.com/astriaorg/astria-release-test/issues/1107)
* **sequencer:** implement `BridgeTransfer` action ([#1934](https://github.com/astriaorg/astria-release-test/issues/1934)) ([456beb0](https://github.com/astriaorg/astria-release-test/commit/456beb00eb282c674207105090427743971fa658)), closes [#1921](https://github.com/astriaorg/astria-release-test/issues/1921)
* **sequencer:** implement `get_pending_nonce` for sequencer API ([#1073](https://github.com/astriaorg/astria-release-test/issues/1073)) ([23c4d9a](https://github.com/astriaorg/astria-release-test/commit/23c4d9ae8c89f3c6982f5e78244bfad45f561e6d))
* **sequencer:** implement `Ics20TransferDepositMemo` format for incoming ics20 transfers to bridge accounts ([#1202](https://github.com/astriaorg/astria-release-test/issues/1202)) ([d64d458](https://github.com/astriaorg/astria-release-test/commit/d64d458daa4084811b39e82ab28b43513627f2cf))
* **sequencer:** implement `RecoverClient` action  ([#2008](https://github.com/astriaorg/astria-release-test/issues/2008)) ([2ae3b64](https://github.com/astriaorg/astria-release-test/commit/2ae3b64e5f57302eee522518e6bf0336eef08fb1))
* **sequencer:** implement abci query for bridge account info ([#1189](https://github.com/astriaorg/astria-release-test/issues/1189)) ([a8db883](https://github.com/astriaorg/astria-release-test/commit/a8db88377c0678378188b0e5cc48c74ec4eebf39)), closes [#1118](https://github.com/astriaorg/astria-release-test/issues/1118)
* **sequencer:** implement bridge sudo and withdrawer addresses ([#1142](https://github.com/astriaorg/astria-release-test/issues/1142)) ([29baa40](https://github.com/astriaorg/astria-release-test/commit/29baa40341aa12769817450310cc9d4c52429503))
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
* **sequencer:** Fix incorrect error message from BridgeUnlock actions ([#1505](https://github.com/astriaorg/astria-release-test/issues/1505)) ([1be156e](https://github.com/astriaorg/astria-release-test/commit/1be156e4818f3234fb0a3f03257f33ca2627d95f)), closes [#1465](https://github.com/astriaorg/astria-release-test/issues/1465)
* **sequencer:** fix TOCTOU issues by merging check and execution ([#1332](https://github.com/astriaorg/astria-release-test/issues/1332)) ([9f959f4](https://github.com/astriaorg/astria-release-test/commit/9f959f4fc492599ffc4a0bfa0d6e29d26b097b4e))
* **sequencer:** improve and fix instrumentation ([#1255](https://github.com/astriaorg/astria-release-test/issues/1255)) ([45a6415](https://github.com/astriaorg/astria-release-test/commit/45a6415bceeb9a52137d6ec909bbe32bfa9d344e))
* **sequencer:** increase mempool removal cache size ([#1969](https://github.com/astriaorg/astria-release-test/issues/1969)) ([2c4da33](https://github.com/astriaorg/astria-release-test/commit/2c4da33d0bc614ab3909870876474acefffe1d12))
* **sequencer:** install astria-eyre hook ([#1552](https://github.com/astriaorg/astria-release-test/issues/1552)) ([c1cc5e5](https://github.com/astriaorg/astria-release-test/commit/c1cc5e5c16767b2149fb4c0b181a7777dc36fbfb)), closes [#1551](https://github.com/astriaorg/astria-release-test/issues/1551)
* **sequencer:** mempool pending nonce calcuations ([#2012](https://github.com/astriaorg/astria-release-test/issues/2012)) ([60bc6f2](https://github.com/astriaorg/astria-release-test/commit/60bc6f200a9d79d167bc6c1e83a1d3947c471971)), closes [#2011](https://github.com/astriaorg/astria-release-test/issues/2011)
* **sequencer:** move fee event recording to tx level ([#1718](https://github.com/astriaorg/astria-release-test/issues/1718)) ([1df9031](https://github.com/astriaorg/astria-release-test/commit/1df903156fb802ac6998afb50d7bde8b768356d5))
* **sequencer:** provide context in checktx response log ([#1506](https://github.com/astriaorg/astria-release-test/issues/1506)) ([b12510d](https://github.com/astriaorg/astria-release-test/commit/b12510d53a1b445429277fd5e23b0cc0ff360bb7)), closes [#1464](https://github.com/astriaorg/astria-release-test/issues/1464)
* **sequencer:** remove failed promotable instead of inserted tx ([#2135](https://github.com/astriaorg/astria-release-test/issues/2135)) ([8759d24](https://github.com/astriaorg/astria-release-test/commit/8759d24e79ef2dbafb13b6eec63bbb74db590360))
* **sequencer:** remove unwrap from app utilized mempool logic  ([#1772](https://github.com/astriaorg/astria-release-test/issues/1772)) ([8879aaf](https://github.com/astriaorg/astria-release-test/commit/8879aafebec56fcdefb563a543778b1a25fd35c8))
* **sequencer:** rewrite check_tx to be more efficient and fix regression ([#1515](https://github.com/astriaorg/astria-release-test/issues/1515)) ([0238e67](https://github.com/astriaorg/astria-release-test/commit/0238e676bbb0512b1d09c275671b4b006a1ec2be))
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
* **core, sequencer:** simplify price feed action ([#2113](https://github.com/astriaorg/astria-release-test/issues/2113)) ([3c31eac](https://github.com/astriaorg/astria-release-test/commit/3c31eace2b1a0264730831a83215234fc785355e))
* **core:** Implement Protobuf trait for tx actions ([#1320](https://github.com/astriaorg/astria-release-test/issues/1320)) ([be139d5](https://github.com/astriaorg/astria-release-test/commit/be139d558f1f57df2f99a2606fc688e889bf4ed9))
* cut releases ([#2017](https://github.com/astriaorg/astria-release-test/issues/2017)) ([a12efeb](https://github.com/astriaorg/astria-release-test/commit/a12efeb0e4000d8ac2adc4e70ced4954cfbbb94c))
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
* **sequencer:** use builder pattern for transaction container tests ([#1592](https://github.com/astriaorg/astria-release-test/issues/1592)) ([8e3a474](https://github.com/astriaorg/astria-release-test/commit/8e3a474cc41cc655892b9d2ee9d8438ad6da27b2))
* **sequencer:** use stream for allowed assets, fix query ([#1710](https://github.com/astriaorg/astria-release-test/issues/1710)) ([f1318b7](https://github.com/astriaorg/astria-release-test/commit/f1318b7434fa763e88081f939f79802fe14a55dd))


### Performance Improvements

* **sequencer:** add benchmark for prepare_proposal (ENG-660) ([#1337](https://github.com/astriaorg/astria-release-test/issues/1337)) ([1f4a359](https://github.com/astriaorg/astria-release-test/commit/1f4a35931233136892a336af16ab30fd4d0f9688))

## [Unreleased]

### Added

- Add metrics:
  - `ASTRIA_SEQUENCER_CHECK_TX_FAILED_ACTION_CHECKS`
  - `ASTRIA_SEQUENCER_CHECK_TX_DURATION_SECONDS_CHECK_ACTIONS`
  - `ASTRIA_SEQUENCER_CHECK_TX_DURATION_SECONDS_RECHECK`
  [#2142](https://github.com/astriaorg/astria/pull/2142)

### Changed

- Changed to use `CheckedTransaction`, `CheckedAction` and `Checked...` wrappers
  for all action types [#2142](https://github.com/astriaorg/astria/pull/2142).
- Rename metric `ASTRIA_SEQUENCER_CHECK_TX_REMOVED_TOO_LARGE` to
  `ASTRIA_SEQUENCER_CHECK_TX_FAILED_TX_TOO_LARGE` [#2142](https://github.com/astriaorg/astria/pull/2142)

### Removed

- Delete metrics:
  - `ASTRIA_SEQUENCER_CHECK_TX_REMOVED_FAILED_STATELESS`
  - `ASTRIA_SEQUENCER_CHECK_TX_DURATION_SECONDS_PARSE_TX`
  - `ASTRIA_SEQUENCER_CHECK_TX_DURATION_SECONDS_CHECK_STATELESS`
  - `ASTRIA_SEQUENCER_CHECK_TX_DURATION_SECONDS_CHECK_TRACKED`
  - `ASTRIA_SEQUENCER_CHECK_TX_DURATION_SECONDS_CHECK_CHAIN_ID`
  - `ASTRIA_SEQUENCER_CHECK_TX_DURATION_SECONDS_CHECK_REMOVED`
  - `ASTRIA_SEQUENCER_CHECK_TX_DURATION_SECONDS_CONVERT_ADDRESS`
  [#2142](https://github.com/astriaorg/astria/pull/2142)

### Fixed

- Remove failed promotable instead of inserted transaction during mempool insertion
  [#2135](https://github.com/astriaorg/astria/pull/2135).

## [3.0.0-rc.1]

### Added

- Provide support for upgrading the sequencer network [#2085](https://github.com/astriaorg/astria/pull/2085).
- Implement first sequencer upgrade, named `Aspen` [#2085](https://github.com/astriaorg/astria/pull/2085).
- Add endpoint to sequencer gRPC service at `/v1/sequencer/upgrades` which
  responds with a summary of applied and scheduled upgrades [#2085](https://github.com/astriaorg/astria/pull/2085).
- Include price feed data in blocks provided by Connect oracle sidecar once
  `Aspen` upgrade has activated [#2085](https://github.com/astriaorg/astria/pull/2085).
- Include names in `ValidatorUpdate` actions once `Aspen` upgrade has been activated
  [#2089](https://github.com/astriaorg/astria/pull/2089).
- Add `ASTRIA_SEQUENCER_UPGRADES_FILEPATH` config variable to specify the path
  to the now required `upgrades.json` file [#2085](https://github.com/astriaorg/astria/pull/2085).
- Add `ASTRIA_SEQUENCER_COMETBFT_RPC_ADDR` config variable to specify the
  address of the CometBFT RPC endpoint for this sequencer [#2085](https://github.com/astriaorg/astria/pull/2085).
- Add `ASTRIA_SEQUENCER_NO_PRICE_FEED` config variable to disable providing
  price feed data in the consensus vote extensions, avoiding the need to run
  the price feed oracle sidecar [#2085](https://github.com/astriaorg/astria/pull/2085).
- Add `ASTRIA_SEQUENCER_PRICE_FEED_GRPC_ADDR` config variable to specify the
  gRPC endpoint for the price feed oracle sidecar [#2085](https://github.com/astriaorg/astria/pull/2085).
- Add `ASTRIA_SEQUENCER_PRICE_FEED_CLIENT_TIMEOUT_MILLISECONDS` config variable
  to specify the timeout for responses from the price feed oracle sidecar [#2085](https://github.com/astriaorg/astria/pull/2085).
- Add the following metrics relating to price feed data:
  `astria_sequencer_extended_commit_info_bytes`,
  `astria_sequencer_extend_vote_duration_seconds`,
  `astria_sequencer_extend_vote_failure_count` and
  `astria_sequencer_verify_vote_extension_failure_count` [#2085](https://github.com/astriaorg/astria/pull/2085).
- Add mempool gRPC service [#2133](https://github.com/astriaorg/astria/pull/2133).

## [2.0.1]

### Security

- Update to tendermint 0.40.3 for security patch to ISA-2025-003 [#2099](https://github.com/astriaorg/astria/pull/2099)

## [2.0.0]

### Added

- Implement `astria.sequencerblock.optimistic.v1alpha1.OptimisticBlockService` [#1839](https://github.com/astriaorg/astria/pull/1839).
- Add `ASTRIA_SEQUENCER_ABCI_LISTEN_URL` config variable [#1877](https://github.com/astriaorg/astria/pull/1877)

### Changed

- Bump MSRV to 1.83.0 [#1857](https://github.com/astriaorg/astria/pull/1857).
- Index all event attributes [#1786](https://github.com/astriaorg/astria/pull/1786).
- Consolidate action handling to single module [#1759](https://github.com/astriaorg/astria/pull/1759).
- Ensure all deposit assets are trace prefixed [#1807](https://github.com/astriaorg/astria/pull/1807).
- Update `idna` dependency to resolve cargo audit warning [#1869](https://github.com/astriaorg/astria/pull/1869).
- Remove events reporting on state storage creation [#1892](https://github.com/astriaorg/astria/pull/1892).
- Use bridge address to determine asset in bridge unlock cost estimation instead
  of signer [#1905](https://github.com/astriaorg/astria/pull/1905).
- Add more thorough unit tests for all actions [#1916](https://github.com/astriaorg/astria/pull/1916).
- Implement `BridgeTransfer` action [#1934](https://github.com/astriaorg/astria/pull/1934).
- Implement `RecoverIbcClient` action [#2008](https://github.com/astriaorg/astria/pull/2008).

### Removed

- Remove ASTRIA_SEQUENCER_LISTEN_ADDR config variable [#1877](https://github.com/astriaorg/astria/pull/1877)

### Fixed

- Increase mempool removal cache size to be greater than default CometBFT
  mempool size [#1969](https://github.com/astriaorg/astria/pull/1969).
- Support distributed signers as validators [#2024](https://github.com/astriaorg/astria/pull/2024)
- Direct fetching of consensus state in `RecoverIbcClient` action [#2037](https://github.com/astriaorg/astria/pull/2037)
- Ensure getPendingNonce gRPC returns the correct nonce [#2012](https://github.com/astriaorg/astria/pull/2012).

## [2.0.0-rc.2]

### Fixed

- Support distributed signers as validators [#2024](https://github.com/astriaorg/astria/pull/2024)
- Direct fetching of consensus state in `RecoverIbcClient` action [#2037](https://github.com/astriaorg/astria/pull/2037)

## [2.0.0-rc.1] - 2025-03-06

### Added

- Implement `astria.sequencerblock.optimistic.v1alpha1.OptimisticBlockService` [#1839](https://github.com/astriaorg/astria/pull/1839).
- Add ASTRIA_SEQUENCER_ABCI_LISTEN_URL config variable [#1877](https://github.com/astriaorg/astria/pull/1877)

### Changed

- Bump MSRV to 1.83.0 [#1857](https://github.com/astriaorg/astria/pull/1857).
- Index all event attributes [#1786](https://github.com/astriaorg/astria/pull/1786).
- Consolidate action handling to single module [#1759](https://github.com/astriaorg/astria/pull/1759).
- Ensure all deposit assets are trace prefixed [#1807](https://github.com/astriaorg/astria/pull/1807).
- Update `idna` dependency to resolve cargo audit warning [#1869](https://github.com/astriaorg/astria/pull/1869).
- Remove events reporting on state storage creation [#1892](https://github.com/astriaorg/astria/pull/1892).
- Use bridge address to determine asset in bridge unlock cost estimation instead
of signer [#1905](https://github.com/astriaorg/astria/pull/1905).
- Add more thorough unit tests for all actions [#1916](https://github.com/astriaorg/astria/pull/1916).
- Implement `BridgeTransfer` action [#1934](https://github.com/astriaorg/astria/pull/1934).
- Implement `RecoverIbcClient` action [#2008](https://github.com/astriaorg/astria/pull/2008).

### Removed

- Remove ASTRIA_SEQUENCER_LISTEN_ADDR config variable [#1877](https://github.com/astriaorg/astria/pull/1877)

### Fixed

- Ensure getPendingNonce gRPC returns the correct nonce [#2012](https://github.com/astriaorg/astria/pull/2012).

## [1.0.0] - 2024-10-25

### Changed

- Bump penumbra dependencies [#1740](https://github.com/astriaorg/astria/pull/1740).
- Move fee event recording to transaction from block [#1718](https://github.com/astriaorg/astria/pull/1718).

## [1.0.0-rc.2] - 2024-10-23

### Changed

- Make ABCI response for account balances deterministic [#1574](https://github.com/astriaorg/astria/pull/1574).
- Move and improve transaction fee estimation [#1722](https://github.com/astriaorg/astria/pull/1722).
- Make fees optional at genesis [#1664](https://github.com/astriaorg/astria/pull/1664).
- Add test for rollup refund in [#1728](https://github.com/astriaorg/astria/pull/1728).
- Make native asset optional [#1703](https://github.com/astriaorg/astria/pull/1703).

### Removed

- Remove unused asset storage variant [#1704](https://github.com/astriaorg/astria/pull/1704).

### Fixed

- Fix fee estimation [#1701](https://github.com/astriaorg/astria/pull/1701).

## [1.0.0-rc.1] - 2024-10-17

### Added

- Add traceability to rollup deposits [#1410](https://github.com/astriaorg/astria/pull/1410).
- Report deposit events [#1447](https://github.com/astriaorg/astria/pull/1447).
- Add IBC sudo change action [#1509](https://github.com/astriaorg/astria/pull/1509).
- Transaction categories on `UnsignedTransaction` [#1512](https://github.com/astriaorg/astria/pull/1512).
- Provide astrotrek chart [#1513](https://github.com/astriaorg/astria/pull/1513).

### Changed

- Change test addresses to versions with known private keys [#1487](https://github.com/astriaorg/astria/pull/1487).
- Make mempool balance aware [#1408](https://github.com/astriaorg/astria/pull/1408).
- Migrate from `anyhow::Result` to `eyre::Result` [#1387](https://github.com/astriaorg/astria/pull/1387).
- Change `Deposit` byte length calculation [#1507](https://github.com/astriaorg/astria/pull/1507).
- Put blocks and deposits to non-verified storage (ENG-812) [#1525](https://github.com/astriaorg/astria/pull/1525).
- Replace `once_cell` with `LazyLock` [#1576](https://github.com/astriaorg/astria/pull/1576).
- Use builder pattern for transaction container tests [#1592](https://github.com/astriaorg/astria/pull/1592).
- Exclusively use Borsh encoding for stored data [#1492](https://github.com/astriaorg/astria/pull/1492).
- Genesis chart template to support latest changes [#1594](https://github.com/astriaorg/astria/pull/1594).
- Simplify boolean expressions in `transaction container` [#1595](https://github.com/astriaorg/astria/pull/1595).
- Make empty transactions invalid  [#1609](https://github.com/astriaorg/astria/pull/1609).
- Rewrite `check_tx` to be more efficient and fix regression [#1515](https://github.com/astriaorg/astria/pull/1515).
- Generate `SequencerBlock` after transaction execution in proposal phase [#1562](https://github.com/astriaorg/astria/pull/1562).
- Add limit to total amount of transactions in parked  [#1638](https://github.com/astriaorg/astria/pull/1638).
- Remove action suffix from all action types [#1630](https://github.com/astriaorg/astria/pull/1630).
- Update `futures-util` dependency based on cargo audit warning [#1644](https://github.com/astriaorg/astria/pull/1644).
- Update storage keys locations and values (ENG-898) [#1616](https://github.com/astriaorg/astria/pull/1616).
- Enforce block ordering by transaction group  [#1618](https://github.com/astriaorg/astria/pull/1618).
- Rework all fees [#1647](https://github.com/astriaorg/astria/pull/1647).
- Prefer `astria.primitive.v1.RollupId` over bytes [#1661](https://github.com/astriaorg/astria/pull/1661).
- Call transactions `Transaction`, contents `TransactionBody` [#1650](https://github.com/astriaorg/astria/pull/1650).
- Rename sequence action to rollup data submission [#1665](https://github.com/astriaorg/astria/pull/1665).
- Upgrade to proto `v1`s throughout [#1672](https://github.com/astriaorg/astria/pull/1672).

### Removed

- Remove unused enable mint env [#1673](https://github.com/astriaorg/astria/pull/1673).

### Fixed

- Add `end_block` to `app_execute_transaction_with_every_action_snapshot` [#1455](https://github.com/astriaorg/astria/pull/1455).
- Fix incorrect error message from `BridgeUnlock` actions [#1505](https://github.com/astriaorg/astria/pull/1505).
- Fix and refactor ics20 logic [#1495](https://github.com/astriaorg/astria/pull/1495).
- Install astria-eyre hook [#1552](https://github.com/astriaorg/astria/pull/1552).
- Provide context in `check_tx` response log [#1506](https://github.com/astriaorg/astria/pull/1506).
- Fix app hash in horcrux sentries [#1646](https://github.com/astriaorg/astria/pull/1646).
- Allow compat prefixed addresses when receiving ics20 transfers [#1655](https://github.com/astriaorg/astria/pull/1655).
- Remove enable mint entry from example env config [#1674](https://github.com/astriaorg/astria/pull/1674).

## [0.17.0] - 2024-09-06

### Changed

- BREAKING: Enforce withdrawals consumed [#1391](https://github.com/astriaorg/astria/pull/1391).
- BREAKING: Permit bech32 compatible addresses [#1425](https://github.com/astriaorg/astria/pull/1425).
- Memoize `address_bytes` of verification key [#1444](https://github.com/astriaorg/astria/pull/1444).

## [0.16.0] - 2024-08-22

### Added

- Add fee reporting [#1305](https://github.com/astriaorg/astria/pull/1305).

### Changed

- Update `bytemark` dependency based on cargo audit warning [#1350](https://github.com/astriaorg/astria/pull/1350).
- BREAKING: Take funds from bridge in ics20 withdrawals [#1344](https://github.com/astriaorg/astria/pull/1344).
- BREAKING: Require that bridge unlock address always be set [#1339](https://github.com/astriaorg/astria/pull/1339).
- Rewrite mempool to have per-account transaction storage and maintenance  [#1323](https://github.com/astriaorg/astria/pull/1323).

### Removed

- Remove global state [#1317](https://github.com/astriaorg/astria/pull/1317).

### Fixed

- Fix abci error code [#1280](https://github.com/astriaorg/astria/pull/1280).
- BREAKING: Fix TOCTOU issues by merging check and execution [#1332](https://github.com/astriaorg/astria/pull/1332).
- Fix block fee collection [#1343](https://github.com/astriaorg/astria/pull/1343).
- Bump penumbra dep to fix ibc state access bug [#1389](https://github.com/astriaorg/astria/pull/1389).

## [0.15.0] - 2024-07-26

### Added

- Implement transaction fee query [#1196](https://github.com/astriaorg/astria/pull/1196).
- Add metrics [#1248](https://github.com/astriaorg/astria/pull/1248).
- Add mempool benchmarks [#1238](https://github.com/astriaorg/astria/pull/1238).

### Changed

- Generate serde traits impls for all protocol protobufs [#1260](https://github.com/astriaorg/astria/pull/1260).
- Define bridge memos in proto [#1285](https://github.com/astriaorg/astria/pull/1285).

### Fixed

- Fix prepare proposal metrics [#1211](https://github.com/astriaorg/astria/pull/1211).
- Fix wrong metric and remove unused metric [#1240](https://github.com/astriaorg/astria/pull/1240).
- Store native asset ibc->trace mapping in `init_chain` [#1242](https://github.com/astriaorg/astria/pull/1242).
- Disambiguate return addresses [#1266](https://github.com/astriaorg/astria/pull/1266).
- Improve and fix instrumentation [#1255](https://github.com/astriaorg/astria/pull/1255).

## [0.14.1] - 2024-07-03

### Added

- Implement abci query for bridge account info [#1189](https://github.com/astriaorg/astria/pull/1189).

### Fixed

- Update asset query path [#1141](https://github.com/astriaorg/astria/pull/1141).

## [0.14.0] - 2024-06-27

### Added

- Add `allowed_fee_asset_ids` abci query and `sequencer_client` support [#1127](https://github.com/astriaorg/astria/pull/1127).
- Implement `bridge/account_last_tx_hash` abci query [#1158](https://github.com/astriaorg/astria/pull/1158).
- Add bech32m addresses [#1124](https://github.com/astriaorg/astria/pull/1124).
- Implement refund to rollup logic upon ics20 transfer refund [#1161](https://github.com/astriaorg/astria/pull/1161).
- Implement bridge sudo and withdrawer addresses [#1142](https://github.com/astriaorg/astria/pull/1142).
- Add ttl and invalid cache to app mempool [#1138](https://github.com/astriaorg/astria/pull/1138).
- Implement `Ics20TransferDepositMemo` format for incoming ics20 transfers to
bridge accounts [#1202](https://github.com/astriaorg/astria/pull/1202).
- Add ibc memo type snapshot tests [#1205](https://github.com/astriaorg/astria/pull/1205).
- Allow configuring base address prefix [#1201](https://github.com/astriaorg/astria/pull/1201).

### Changed

- Query full denomination from asset ID [#1067](https://github.com/astriaorg/astria/pull/1067).
- Add `clippy::arithmetic-side-effects` lint and fix resulting warnings [#1081](https://github.com/astriaorg/astria/pull/1081).
- Use macro to declare metric constants [#1129](https://github.com/astriaorg/astria/pull/1129).
- Bump penumbra deps [#1159](https://github.com/astriaorg/astria/pull/1159).
- Register all metrics during startup [#1144](https://github.com/astriaorg/astria/pull/1144).
- Parse ics20 denoms as ibc or trace prefixed variants [#1181](https://github.com/astriaorg/astria/pull/1181).
- Remove non-bech32m address bytes [#1186](https://github.com/astriaorg/astria/pull/1186).
- Bump penumbra deps [#1216](https://github.com/astriaorg/astria/pull/1216).
- Use full IBC ICS20 denoms instead of IDs [#1209](https://github.com/astriaorg/astria/pull/1209).

### Removed

- Remove mint module [#1134](https://github.com/astriaorg/astria/pull/1134).

### Fixed

- Prefix removal source non-refund ics20 packet [#1162](https://github.com/astriaorg/astria/pull/1162).

## [0.13.0] - 2024-05-23

### Added

- Implement `get_pending_nonce` for sequencer API [#1073](https://github.com/astriaorg/astria/pull/1073).

### Changed

- Fees go to sudo poa [#1104](https://github.com/astriaorg/astria/pull/1104).

## [0.12.0] - 2024-05-21

### Added

- Implement basic app side mempool with nonce ordering [#1000](https://github.com/astriaorg/astria/pull/1000).
- Add fees to genesis state [#1055](https://github.com/astriaorg/astria/pull/1055).
- Implement bridge unlock action and derestrict transfers [#1034](https://github.com/astriaorg/astria/pull/1034).
- Implement `FeeChangeAction` for the authority component [#1037](https://github.com/astriaorg/astria/pull/1037).

### Changed

- Store fees for actions in app state [#1017](https://github.com/astriaorg/astria/pull/1017).
- Update ics20 withdrawal to have a memo field [#1056](https://github.com/astriaorg/astria/pull/1056).
- Update `SignedTransaction` to contain `Any` for transaction [#1044](https://github.com/astriaorg/astria/pull/1044).

### Fixed

- Stateful check now ensures balance for total tx [#1009](https://github.com/astriaorg/astria/pull/1009).
- Set current app hash properly when creating app [#1025](https://github.com/astriaorg/astria/pull/1025).
- Panic sequencer instead of cometbft on erroring abci consensus requests [#1016](https://github.com/astriaorg/astria/pull/1016).
- Fix ibc prefix conversion [#1065](https://github.com/astriaorg/astria/pull/1065).

## [0.11.0] - 2024-04-26

### Added

- Add cargo audit to CI [#887](https://github.com/astriaorg/astria/pull/887).
- Add unit tests for state extension trait [#890](https://github.com/astriaorg/astria/pull/890).
- Create `sequencerblockapis` `v1alpha1` [#939](https://github.com/astriaorg/astria/pull/939).
- Add display for deposits in `end_block` [#864](https://github.com/astriaorg/astria/pull/864).
- Create wrapper types for `RollupId` and `Account` [#987](https://github.com/astriaorg/astria/pull/987).
- Add initial set of metrics to sequencer [#965](https://github.com/astriaorg/astria/pull/965).

### Changed

- Check for sufficient balance in `check_tx` [#869](https://github.com/astriaorg/astria/pull/869).
- Generate names for protobuf rust types [#904](https://github.com/astriaorg/astria/pull/904).
- Replace hex by base64 for display formatting, emitting tracing events [#908](https://github.com/astriaorg/astria/pull/908).
- Set revision number from chain id in `init_chain` [#935](https://github.com/astriaorg/astria/pull/935).
- Update `SequencerBlockHeader` and related proto types to not use cometbft
header [#830](https://github.com/astriaorg/astria/pull/830).
- Update to ABCI v0.38 [#831](https://github.com/astriaorg/astria/pull/831).
- Fully split `sequencerapis` and remove [#958](https://github.com/astriaorg/astria/pull/958).
- Require chain id in transactions [#973](https://github.com/astriaorg/astria/pull/973).
- Update justfile and testnet script [#985](https://github.com/astriaorg/astria/pull/985).
- Bridge account only takes a single asset [#988](https://github.com/astriaorg/astria/pull/988).

### Removed

- No telemetry for formatting db keys [#909](https://github.com/astriaorg/astria/pull/909).
- Remove `SequencerBlock::try_from_cometbft` [#1005](https://github.com/astriaorg/astria/pull/1005).

### Fixed

- Make `get_deposit_rollup_ids` not return duplicates [#916](https://github.com/astriaorg/astria/pull/916).
- `is_proposer` check now considers proposer's address [#936](https://github.com/astriaorg/astria/pull/936).
- Respect `max_tx_bytes` when preparing proposals [#911](https://github.com/astriaorg/astria/pull/911).
- Fix state setup to be consistent before transaction execution [#945](https://github.com/astriaorg/astria/pull/945).
- Don't store execution result of failed tx [#992](https://github.com/astriaorg/astria/pull/992).
- Don't allow sudo to cause consensus failures [#999](https://github.com/astriaorg/astria/pull/999).

## [0.10.1] - 2024-04-03

### Added

- Implement bridge deposits for incoming ICS20 transfers [#843](https://github.com/astriaorg/astria/pull/843).
- Add serialization to execution `v1alpha2` compliant with protobuf json
mapping [#857](https://github.com/astriaorg/astria/pull/857).
- Add unit tests for state extension traits
[#858](https://github.com/astriaorg/astria/pull/858),
[#871](https://github.com/astriaorg/astria/pull/871),
[#874](https://github.com/astriaorg/astria/pull/874),
[#875](https://github.com/astriaorg/astria/pull/875),
[#876](https://github.com/astriaorg/astria/pull/876) and
[#878](https://github.com/astriaorg/astria/pull/878).

### Changed

- Use `Arc<Self>` target in generated gRPC service traits [#853](https://github.com/astriaorg/astria/pull/853).
- Logging as human readable for account state [#898](https://github.com/astriaorg/astria/pull/898).

### Fixed

- Bump otel to resolve panics in layered span access [#820](https://github.com/astriaorg/astria/pull/820).
- Fix `is_source` prefix check [#844](https://github.com/astriaorg/astria/pull/844).
- Fix escrow channel check when receiving non-refund ics20 packet [#851](https://github.com/astriaorg/astria/pull/851).
- Fix rollup ids commitment for deposits [#863](https://github.com/astriaorg/astria/pull/863).

## [0.10.0] - 2024-03-19

### Added

- Add sequencer service proto [#701](https://github.com/astriaorg/astria/pull/701).
- Implement bridge accounts and related actions [#768](https://github.com/astriaorg/astria/pull/768).

### Changed

- Simplify emitting error fields with cause chains [#765](https://github.com/astriaorg/astria/pull/765).
- Update dependencies [#782](https://github.com/astriaorg/astria/pull/782).
- Store sequencer blocks in the sequencer state [#787](https://github.com/astriaorg/astria/pull/787).
- Include deposit data as part of rollup data [#802](https://github.com/astriaorg/astria/pull/802).
- Bump penumbra deps [#825](https://github.com/astriaorg/astria/pull/825).

### Fixed

- Filtered blocks success when no data expected [#819](https://github.com/astriaorg/astria/pull/819).
- Fix bug in `get_sequencer_block_by_hash` [#832](https://github.com/astriaorg/astria/pull/832).

## [0.9.0] - 2024-02-15

### Added

- Add `SignedTransaction::sha256_of_proto_encoding()` method [#687](https://github.com/astriaorg/astria/pull/687).
- Add `ibc_sudo_address` to genesis, only allow `IbcRelay` actions from this
address [#721](https://github.com/astriaorg/astria/pull/721).
- Use opentelemetry [#656](https://github.com/astriaorg/astria/pull/656).
- Allow specific assets for fee payment [#730](https://github.com/astriaorg/astria/pull/730).
- Metrics setup [#739](https://github.com/astriaorg/astria/pull/739) and [#750](https://github.com/astriaorg/astria/pull/750).
- Add `ibc_relayer_addresses` list and allow modifications via
`ibc_sudo_address` [#737](https://github.com/astriaorg/astria/pull/737).
- Add pretty-printing to stdout [#736](https://github.com/astriaorg/astria/pull/736).
- Implement ability to update fee assets using sudo key [#752](https://github.com/astriaorg/astria/pull/752).
- Print build info in all services [#753](https://github.com/astriaorg/astria/pull/753).

### Changed

- Transfer fees to block proposer instead of burning [#690](https://github.com/astriaorg/astria/pull/690).
- Update licenses [#706](https://github.com/astriaorg/astria/pull/706).
- Update balance queries to return every asset owned by account [#683](https://github.com/astriaorg/astria/pull/683).
- Use `IbcComponent` and penumbra `HostInterface` [#700](https://github.com/astriaorg/astria/pull/700).
- Move fee asset from `UnsignedTransaction` to `SequenceAction` and
`TransferAction` [#719](https://github.com/astriaorg/astria/pull/719).
- Relax size requirements of hash buffers [#709](https://github.com/astriaorg/astria/pull/709).
- Split protos into multiple buf repos [#732](https://github.com/astriaorg/astria/pull/732).
- Add fee for `Ics20Withdrawal` action [#733](https://github.com/astriaorg/astria/pull/733).
- Bump rust to 1.76, cargo-chef to 0.1.63 [#744](https://github.com/astriaorg/astria/pull/744).
- Upgrade to penumbra release 0.66 [#741](https://github.com/astriaorg/astria/pull/741).
- Move ibc-related code to its own module [#757](https://github.com/astriaorg/astria/pull/757).

### Fixed

- Fix `FungibleTokenPacketData` decoding [#686](https://github.com/astriaorg/astria/pull/686).
- Replace allocating display impl [#738](https://github.com/astriaorg/astria/pull/738).
- Fix docker builds [#756](https://github.com/astriaorg/astria/pull/756).

## [0.8.0] - 2024-01-10

### Added

- Add proto formatting, cleanup justfile [#637](https://github.com/astriaorg/astria/pull/637).
- Implement ICS20 withdrawals [#609](https://github.com/astriaorg/astria/pull/609).
- Add IBC gRPC server to sequencer app [#631](https://github.com/astriaorg/astria/pull/631).
- Lint debug fields in tracing events [#664](https://github.com/astriaorg/astria/pull/664).

### Changed

- Move protobuf specs to repository top level [#629](https://github.com/astriaorg/astria/pull/629).
- Bump all checkout actions in CI to v3 [#641](https://github.com/astriaorg/astria/pull/641).
- Unify construction of cometbft blocks in tests [#640](https://github.com/astriaorg/astria/pull/640).
- Store mapping of IBC asset ID to full denomination trace [#614](https://github.com/astriaorg/astria/pull/614).
- Switch tagging format in CI [#639](https://github.com/astriaorg/astria/pull/639).
- Bump penumbra deps [#655](https://github.com/astriaorg/astria/pull/655).
- Rename `astria-proto` to `astria-core` [#644](https://github.com/astriaorg/astria/pull/644).
- Break up `v1alpha1` module [#646](https://github.com/astriaorg/astria/pull/646).
- Don't deny unknown config fields [#657](https://github.com/astriaorg/astria/pull/657).
- Call abort on ABCI server on signal [#670](https://github.com/astriaorg/astria/pull/670).
- Define abci error codes in protobuf [#647](https://github.com/astriaorg/astria/pull/647).
- Use display formatting instead of debug formatting in tracing events [#671](https://github.com/astriaorg/astria/pull/671).
- Update instrumentation for all consensus & app functions [#677](https://github.com/astriaorg/astria/pull/677).
- Add max sequencer bytes per block limit [#676](https://github.com/astriaorg/astria/pull/676).

### Removed

- Remove `AppHash` [#655](https://github.com/astriaorg/astria/pull/655).

### Fixed

- Adjust input to proto breaking change linter after refactor [#635](https://github.com/astriaorg/astria/pull/635).
- Fix ABCI event handling [#666](https://github.com/astriaorg/astria/pull/666).
- Clear processed tx count in `begin_block` [#659](https://github.com/astriaorg/astria/pull/659).
- Amend Cargo.toml when building images [#672](https://github.com/astriaorg/astria/pull/672).
- Update app state to latest committed before starting round [#673](https://github.com/astriaorg/astria/pull/673).
- Allow blocksync to complete successfully [#675](https://github.com/astriaorg/astria/pull/675).

## [0.7.0] - 2023-11-30

### Added

- Implement support for arbitrary assets [#568](https://github.com/astriaorg/astria/pull/568).
- Support `IbcAction`s and implement ICS20 incoming transfer application logic [#579](https://github.com/astriaorg/astria/pull/579).

### Changed

- Replace `buf-generate` by `tonic_build` [#581](https://github.com/astriaorg/astria/pull/581).
- Bump all dependencies (mainly penumbra, celestia, tendermint) [#582](https://github.com/astriaorg/astria/pull/582).
- Enforce sequencer blob invariants [#576](https://github.com/astriaorg/astria/pull/576).
- Require `chain_id` be 32 bytes [#436](https://github.com/astriaorg/astria/pull/436).
- Update penumbra-ibc features [#615](https://github.com/astriaorg/astria/pull/615).

### Fixed

- Fix instrument logging not to log every tx [#595](https://github.com/astriaorg/astria/pull/595).
- Cap tx size at 250kB [#601](https://github.com/astriaorg/astria/pull/601).

## [0.6.0] - 2023-11-18

### Added

- Add an RFC-6962 compliant Merkle tree with flat memory representation [#554](https://github.com/astriaorg/astria/pull/554).

## [0.5.0] - 2023-11-07

### Added

- Implement sudo key changes [#431](https://github.com/astriaorg/astria/pull/431).
- Implement minting module [#435](https://github.com/astriaorg/astria/pull/435).

### Changed

- Remove byzantine validators in `begin_block` [#429](https://github.com/astriaorg/astria/pull/429).
- Bump penumbra, tendermint; prune workspace cargo of unused deps [#468](https://github.com/astriaorg/astria/pull/468).
- Bump rust to 1.72 in CI [#477](https://github.com/astriaorg/astria/pull/477).
- Use fork of tendermint with backported `reqwest` client [#498](https://github.com/astriaorg/astria/pull/498).
- Move transaction execution to prepare/process proposal [#480](https://github.com/astriaorg/astria/pull/480).

### Fixed

- Fix tests without `--all-features` [#481](https://github.com/astriaorg/astria/pull/481).
- Fix typos [#541](https://github.com/astriaorg/astria/pull/541).
- Implement `chain_ids_commitment` inclusion proof generation and verification [#548](https://github.com/astriaorg/astria/pull/548).
- Fix authority component `ValidatorSet` non determinism [#557](https://github.com/astriaorg/astria/pull/557).
- Run only `prepare_proposal` if proposer [#558](https://github.com/astriaorg/astria/pull/558).

## [0.4.1] - 2023-09-27

### Added

- Implement basic validator set updates [#359](https://github.com/astriaorg/astria/pull/359).

### Fixed

- Fix mempool nonce check [#434](https://github.com/astriaorg/astria/pull/434).

## 0.4.0 - 2023-09-22

### Added

- Initial release.

[unreleased]: https://github.com/astriaorg/astria/compare/sequencer-v3.0.0-rc.1...HEAD
[3.0.0-rc.1]: https://github.com/astriaorg/astria/compare/sequencer-v2.0.1...sequencer-v3.0.0-rc.1
[2.0.1]: https://github.com/astriaorg/astria/compare/sequencer-v2.0.0...sequencer-v2.0.1
[2.0.0]: https://github.com/astriaorg/astria/compare/sequencer-v1.0.0...sequencer-v2.0.0
[2.0.0-rc.2]: https://github.com/astriaorg/astria/compare/sequencer-v2.0.0-rc.1...sequencer-v2.0.0-rc.2
[2.0.0-rc.1]: https://github.com/astriaorg/astria/compare/sequencer-v1.0.0...sequencer-v2.0.0-rc.1
[1.0.0]: https://github.com/astriaorg/astria/compare/sequencer-v1.0.0-rc.2...sequencer-v1.0.0
[1.0.0-rc.2]: https://github.com/astriaorg/astria/compare/sequencer-v1.0.0-rc.1...sequencer-v1.0.0-rc.2
[1.0.0-rc.1]: https://github.com/astriaorg/astria/compare/sequencer-v0.17.0...sequencer-v1.0.0-rc.1
[0.17.0]: https://github.com/astriaorg/astria/compare/cli-v0.4.0...sequencer-v0.17.0
[0.16.0]: https://github.com/astriaorg/astria/compare/sequencer-v0.15.0...sequencer-v0.16.0
[0.15.0]: https://github.com/astriaorg/astria/compare/sequencer-v0.14.1...sequencer-v0.15.0
[0.14.1]: https://github.com/astriaorg/astria/compare/sequencer-v0.14.0...sequencer-v0.14.1
[0.14.0]: https://github.com/astriaorg/astria/compare/sequencer-v0.13.0...sequencer-v0.14.0
[0.13.0]: https://github.com/astriaorg/astria/compare/sequencer-v0.12.0...sequencer-v0.13.0
[0.12.0]: https://github.com/astriaorg/astria/compare/sequencer-v0.11.0...sequencer-v0.12.0
[0.11.0]: https://github.com/astriaorg/astria/compare/sequencer-v0.10.1...sequencer-v0.11.0
[0.10.1]: https://github.com/astriaorg/astria/compare/sequencer-v0.10.0...sequencer-v0.10.1
[0.10.0]: https://github.com/astriaorg/astria/compare/sequencer-v0.9.0...sequencer-v0.10.0
[0.9.0]: https://github.com/astriaorg/astria/compare/sequencer-v0.8.0...sequencer-v0.9.0
[0.8.0]: https://github.com/astriaorg/astria/compare/sequencer-v0.7.0...sequencer-v0.8.0
[0.7.0]: https://github.com/astriaorg/astria/compare/v0.6.0--sequencer...v0.7.0--sequencer
[0.6.0]: https://github.com/astriaorg/astria/compare/v0.5.0--sequencer...v0.6.0--sequencer
[0.5.0]: https://github.com/astriaorg/astria/compare/v0.4.1--sequencer...v0.5.0--sequencer
[0.4.1]: https://github.com/astriaorg/astria/compare/v0.4.0--sequencer...v0.4.1--sequencer
