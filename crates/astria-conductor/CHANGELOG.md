<!-- markdownlint-disable no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0](https://github.com/astriaorg/astria-release-test/compare/conductor-vv1.1.0...conductor-vv2.0.0) (2025-05-09)


### ⚠ BREAKING CHANGES

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

## [Unreleased]

### Added

- Include price feed oracle data in transactions when sequencer network
  provides it [#2085](https://github.com/astriaorg/astria/pull/2085).

### Fixed

- Update `crossbeam-channel` dependency to resolve cargo audit warning [#2106](https://github.com/astriaorg/astria/pull/2106).

## [1.1.0] - 2025-03-06

### Changed

- Update `idna` dependency to resolve cargo audit warning [#1869](https://github.com/astriaorg/astria/pull/1869).
- Remove panic source on shutdown [#1919](https://github.com/astriaorg/astria/pull/1919).

### Added

- Send `sequencer_block_hash` as part of `ExecuteBlockRequest` [#1999](https://github.com/astriaorg/astria/pull/1999).

## [1.0.0] - 2024-10-25

### Changed

- Bump MSRV to 1.83.0 [#1857](https://github.com/astriaorg/astria/pull/1857).
- Bump penumbra dependencies [#1740](https://github.com/astriaorg/astria/pull/1740).

## [1.0.0-rc.2] - 2024-10-23

### Changed

- Make native asset optional [#1703](https://github.com/astriaorg/astria/pull/1703).

## [1.0.0-rc.1] - 2024-10-17

### Added

- Add traceability to rollup deposits [#1410](https://github.com/astriaorg/astria/pull/1410).
- Implement restart logic [#1463](https://github.com/astriaorg/astria/pull/1463).
- Implement chain ID checks [#1482](https://github.com/astriaorg/astria/pull/1482).

### Changed

- Replace `once_cell` with `LazyLock` [#1576](https://github.com/astriaorg/astria/pull/1576).
- Migrate all instances of `#[allow]` to `#[expect]` [#1561](https://github.com/astriaorg/astria/pull/1561).
- Code freeze through github actions [#1588](https://github.com/astriaorg/astria/pull/1588).
- Upgrade to proto `v1`s throughout [#1672](https://github.com/astriaorg/astria/pull/1672).

### Fixed

- Fix flaky restart test [#1575](https://github.com/astriaorg/astria/pull/1575).
- Remove enable mint entry from example env config [#1674](https://github.com/astriaorg/astria/pull/1674).

## [0.20.1] - 2024-09-06

### Changed

- Improve instrumentation [#1330](https://github.com/astriaorg/astria/pull/1330).

## [0.20.0] - 2024-08-22

### Changed

- Update `bytemark` dependency based on cargo audit warning [#1350](https://github.com/astriaorg/astria/pull/1350).
- Update with support for celestia-node v0.15.0 [#1367](https://github.com/astriaorg/astria/pull/1367).
- Support disabled celestia auth [#1372](https://github.com/astriaorg/astria/pull/1372).

## [0.19.0] - 2024-07-26

### Fixed

- Don't panic during panic [#1252](https://github.com/astriaorg/astria/pull/1252).
- Change execution API to use primitive RollupId [#1291](https://github.com/astriaorg/astria/pull/1291).

## [0.18.0] - 2024-06-27

### Added

- Add bech32m addresses [#1124](https://github.com/astriaorg/astria/pull/1124).

### Changed

- Remove non-bech32m address bytes [#1186](https://github.com/astriaorg/astria/pull/1186).

## [0.17.0] - 2024-06-04

### Added

- Rate limit sequencer cometbft requests [#1068](https://github.com/astriaorg/astria/pull/1068).
- Add retry to execution API gRPCs [#1115](https://github.com/astriaorg/astria/pull/1115).
- Add metrics [#1091](https://github.com/astriaorg/astria/pull/1091).

### Changed

- Perform signal handling in binary and test shutdown logic [#1094](https://github.com/astriaorg/astria/pull/1094).
- Celestia base heights in commitment state [#1121](https://github.com/astriaorg/astria/pull/1121).
- Skip outdated block metadata [#1120](https://github.com/astriaorg/astria/pull/1120).

## [0.16.0] - 2024-05-21

### Changed

- Update `SignedTransaction` to contain `Any` for transaction [#1044](https://github.com/astriaorg/astria/pull/1044).

### Fixed

- Don't exit on bad Sequencer connection [#1076](https://github.com/astriaorg/astria/pull/1076).
- Respect shutdown signals during init [#1080](https://github.com/astriaorg/astria/pull/1080).

## [0.15.0] - 2024-05-09

### Added

- Fetch missing blocks as necessary [#1054](https://github.com/astriaorg/astria/pull/1054).

### Changed

- Batch multiple Sequencer blocks to save on Celestia fees [#1045](https://github.com/astriaorg/astria/pull/1045).

### Fixed

- Only execute firm blocks if firm and soft block numbers match [#1021](https://github.com/astriaorg/astria/pull/1021).
- Retry blob fetch on request timeout [#1061](https://github.com/astriaorg/astria/pull/1061).

## [0.14.0] - 2024-04-26

### Added

- Create `sequencerblockapis` `v1alpha1` [#939](https://github.com/astriaorg/astria/pull/939).
- Add blackbox tests for conductor running in soft-only mode [#917](https://github.com/astriaorg/astria/pull/917).
- Brotli compress data blobs [#1006](https://github.com/astriaorg/astria/pull/1006).

### Changed

- Update `SequencerBlockHeader` and related proto types to not use cometbft
header [#830](https://github.com/astriaorg/astria/pull/830).
- Update execution service to use sequencerblock [#954](https://github.com/astriaorg/astria/pull/954).
- Fully split `sequencerapis` and remove [#958](https://github.com/astriaorg/astria/pull/958).
- Fetch blocks pending finalization [#980](https://github.com/astriaorg/astria/pull/980).

### Fixed

- Robust Celestia blob fetch, verify, convert [#946](https://github.com/astriaorg/astria/pull/946).

## [0.13.1] - 2024-04-05

### Added

- Add serialization to execution `v1alpha2` compliant with protobuf json
mapping [#857](https://github.com/astriaorg/astria/pull/857).

### Changed

- Simplify builder types by config-like [#829](https://github.com/astriaorg/astria/pull/829).
- Use cancellation tokens for shutdown [#845](https://github.com/astriaorg/astria/pull/845).
- Generate pbjon impls for sequencer types needed to mock conductor [#905](https://github.com/astriaorg/astria/pull/905).
- Replace hex by base64 for display formatting, emitting tracing events [#908](https://github.com/astriaorg/astria/pull/908).

### Fixed

- Bump otel to resolve panics in layered span access [#820](https://github.com/astriaorg/astria/pull/820).
- Don't panic while shutting down [#846](https://github.com/astriaorg/astria/pull/846).
- Clarify conductor log [#868](https://github.com/astriaorg/astria/pull/868).
- Enable tls for grpc connections [#925](https://github.com/astriaorg/astria/pull/925).

## [0.13.0] - 2024-03-19

### Added

- Provide explicit HTTP, Websocket Celestia RPC URLs [#780](https://github.com/astriaorg/astria/pull/780).
- Report if conductor won't read more Celestia heights [#799](https://github.com/astriaorg/astria/pull/799).

### Changed

- Simplify emitting error fields with cause chains [#765](https://github.com/astriaorg/astria/pull/765).
- Assert host fulfills execution API contract [#772](https://github.com/astriaorg/astria/pull/772).
- Update increment celestia height to fetch [#801](https://github.com/astriaorg/astria/pull/801).
- Use Celestia crates published on crates.io [#806](https://github.com/astriaorg/astria/pull/806).
- Emit more information about blocks received from Sequencer, Celestia [#811](https://github.com/astriaorg/astria/pull/811).
- Use Sequencer gRPC API to fetch soft bocks. [#815](https://github.com/astriaorg/astria/pull/815).
- Migrate `v1alpha1` sequencer apis to `v1` [#817](https://github.com/astriaorg/astria/pull/817).

### Removed

- Remove all optimism functionality [#775](https://github.com/astriaorg/astria/pull/775).
- Delete unused proto file [#783](https://github.com/astriaorg/astria/pull/783).

### Fixed

- Keep `wsclient` alive [#762](https://github.com/astriaorg/astria/pull/762).
- Simplify mapping Celestia heights to Sequencer heights [#797](https://github.com/astriaorg/astria/pull/797).
- Serialize rollup IDs as strings so telemetry doesn't crash [#821](https://github.com/astriaorg/astria/pull/821).

## [0.12.0] - 2024-02-15

### Added

- Add `SignedTransaction::sha256_of_proto_encoding()` method [#687](https://github.com/astriaorg/astria/pull/687).
- Add `ibc_sudo_address` to genesis, only allow `IbcRelay` actions from this
address [#721](https://github.com/astriaorg/astria/pull/721).
- Add firm block syncing [#691](https://github.com/astriaorg/astria/pull/691).
- Use opentelemetry [#656](https://github.com/astriaorg/astria/pull/656).
- Metrics setup [#739](https://github.com/astriaorg/astria/pull/739) and [#750](https://github.com/astriaorg/astria/pull/750).
- Add `ibc_relayer_addresses` list and allow modifications via
`ibc_sudo_address` [#737](https://github.com/astriaorg/astria/pull/737).
- Add pretty-printing to stdout [#736](https://github.com/astriaorg/astria/pull/736).
- Print build info in all services [#753](https://github.com/astriaorg/astria/pull/753).

### Changed

- Transfer fees to block proposer instead of burning [#690](https://github.com/astriaorg/astria/pull/690).
- Update licenses [#706](https://github.com/astriaorg/astria/pull/706).
- Move fee asset from `UnsignedTransaction` to `SequenceAction` and
`TransferAction` [#719](https://github.com/astriaorg/astria/pull/719).
- Build all binaries, fix pr title ci [#728](https://github.com/astriaorg/astria/pull/728).
- Split protos into multiple buf repos [#732](https://github.com/astriaorg/astria/pull/732).
- Bump rust to 1.76, cargo-chef to 0.1.63 [#744](https://github.com/astriaorg/astria/pull/744).
- Aet permitted commitment spread from rollup genesis [#743](https://github.com/astriaorg/astria/pull/743).

### Fixed

- Fix `FungibleTokenPacketData` decoding [#686](https://github.com/astriaorg/astria/pull/686).
- Relax size requirements of hash buffers [#709](https://github.com/astriaorg/astria/pull/709).
- Replace allocating display impl [#738](https://github.com/astriaorg/astria/pull/738).
- Fix docker builds [#756](https://github.com/astriaorg/astria/pull/756).

## [0.11.1] - 2024-01-10

### Added

- Lint debug fields in tracing events [#664](https://github.com/astriaorg/astria/pull/664).

### Changed

- Use methods to increment, ensure macro to compare [#619](https://github.com/astriaorg/astria/pull/619).
- Add proto formatting, cleanup justfile [#637](https://github.com/astriaorg/astria/pull/637).
- Bump all checkout actions in CI to v3 [#641](https://github.com/astriaorg/astria/pull/641).
- Unify construction of cometbft blocks in tests [#640](https://github.com/astriaorg/astria/pull/640).
- Switch tagging format in CI [#639](https://github.com/astriaorg/astria/pull/639).
- Rename astria-proto to astria-core [#644](https://github.com/astriaorg/astria/pull/644).
- Break up sequencer `v1alpha1` module [#646](https://github.com/astriaorg/astria/pull/646).
- Don't deny unknown config fields [#657](https://github.com/astriaorg/astria/pull/657).
- Define abci error codes in protobuf [#647](https://github.com/astriaorg/astria/pull/647).
- Use display formatting instead of debug formatting in tracing events [#671](https://github.com/astriaorg/astria/pull/671).

### Fixed

- Add error context, simplify type conversions [#620](https://github.com/astriaorg/astria/pull/620).
- Fail hard when executing blocks fails [#621](https://github.com/astriaorg/astria/pull/621).
- Amend Cargo.toml when building images [#672](https://github.com/astriaorg/astria/pull/672).

## [0.11.0] - 2023-11-30

### Added

- Don't attempt to execute finalized blocks [#617](https://github.com/astriaorg/astria/pull/617).

### Changed

- Make block verifier a submodule of data availability [#593](https://github.com/astriaorg/astria/pull/593).
- Require chain_id be 32 bytes [#436](https://github.com/astriaorg/astria/pull/436).
- Redefine sequencer blocks, celestia blobs as protobuf [#395](https://github.com/astriaorg/astria/pull/395).

### Fixed

- Validator height should be trailing [#613](https://github.com/astriaorg/astria/pull/613).

## [0.10.0] - 2023-11-18

### Added

- Add an RFC 6962 compliant Merkle tree with flat memory representation [#554](https://github.com/astriaorg/astria/pull/554).
- Implement derivation and execution of optimism `DepositTransaction`s [#535](https://github.com/astriaorg/astria/pull/535).

## [0.9.0] - 2023-11-14

### Changed

- Implement clippy pedantic suggestions [#573](https://github.com/astriaorg/astria/pull/573).

### Fixed

- Use sequencer chain id for sequencer blobs [#577](https://github.com/astriaorg/astria/pull/577).

## [0.8.0] - 2023-11-07

### Added

- Add re-sync for missed sequencer blocks [#515](https://github.com/astriaorg/astria/pull/515).
- Add commitment grab to better set sync start height [#553](https://github.com/astriaorg/astria/pull/553).

### Changed

- Celestia-client: use eiger's version [#486](https://github.com/astriaorg/astria/pull/486).
- Replace formatted error backtraces by value impl [#516](https://github.com/astriaorg/astria/pull/516).
- `v1alpha2` integration [#528](https://github.com/astriaorg/astria/pull/528).
- Define service configs in terms of a central crate [#537](https://github.com/astriaorg/astria/pull/537).
- Verify current block commit in conductor; remove `last_commit` from
`SequencerBlockData` [#560](https://github.com/astriaorg/astria/pull/560).

### Removed

- Remove signing and signature verification of data posted to DA [#538](https://github.com/astriaorg/astria/pull/538).
- Remove disable empty block execution config setting [#556](https://github.com/astriaorg/astria/pull/556).

### Fixed

- Update rollup chain id in conductor example env to match composer [#505](https://github.com/astriaorg/astria/pull/505).
- Clarify logging in executor [#508](https://github.com/astriaorg/astria/pull/508).
- Implement `chain_ids_commitment` inclusion proof generation and verification [#548](https://github.com/astriaorg/astria/pull/548).
- Empty blocks from da get executed [#551](https://github.com/astriaorg/astria/pull/551).
- Dependency update for yanked `ahash` deps [#544](https://github.com/astriaorg/astria/pull/544).

## [0.7.0] - 2023-10-13

### Added

- Add execution commit level v2 [#474](https://github.com/astriaorg/astria/pull/474).
- Report cause of failed nonce fetch [#492](https://github.com/astriaorg/astria/pull/492).

### Changed

- Use fork of tendermint with backported `reqwest` client [#498](https://github.com/astriaorg/astria/pull/498).
- Never recycle websocket clients [#499](https://github.com/astriaorg/astria/pull/499).
- Spawn driver as task and report exit [#500](https://github.com/astriaorg/astria/pull/500).
- Resubscribe with backoff instead of failing [#501](https://github.com/astriaorg/astria/pull/501).

## [0.6.1] - 2023-10-12

### Added

- Log task exit [#479](https://github.com/astriaorg/astria/pull/479).

### Changed

- Bump penumbra, tendermint; prune workspace cargo of unused deps [#468](https://github.com/astriaorg/astria/pull/468).
- Reconnect to sequencer websocket with backoff [#483](https://github.com/astriaorg/astria/pull/483).

### Fixed

- Don't panic on empty blocks [#467](https://github.com/astriaorg/astria/pull/467).
- Fix action tree root inclusion proof verification [#469](https://github.com/astriaorg/astria/pull/469).

## [0.6.0] - 2023-10-05

### Changed

- Add genesis sequencer block height to config and env vars [#445](https://github.com/astriaorg/astria/pull/445).
- Refactor and implement full node sync from sequencer [#455](https://github.com/astriaorg/astria/pull/455).

## [0.5.1] - 2023-09-27

### Fixed

- Bug fixes related to validating data and allowing empty rollup blocks.
- Fix tendermint block to `SequencerBlockData` conversion [#424](https://github.com/astriaorg/astria/pull/424).
- Continue to execution when block subset empty [#426](https://github.com/astriaorg/astria/pull/426).

## 0.5.0 - 2023-09-22

### Added

- Initial release.

[unreleased]: https://github.com/astriaorg/astria/compare/conductor-v1.0.0...HEAD
[1.1.0]: https://github.com/astriaorg/astria/compare/conductor-v1.0.0...conductor-v1.1.0
[1.0.0]: https://github.com/astriaorg/astria/compare/conductor-v1.0.0-rc.2...conductor-v1.0.0
[1.0.0-rc.2]: https://github.com/astriaorg/astria/compare/conductor-v1.0.0-rc.1...conductor-v1.0.0-rc.2
[1.0.0-rc.1]: https://github.com/astriaorg/astria/compare/conductor-v0.20.1...conductor-v1.0.0-rc.1
[0.20.1]: https://github.com/astriaorg/astria/compare/conductor-v0.20.0...conductor-v0.20.1
[0.20.0]: https://github.com/astriaorg/astria/compare/conductor-v0.19.0...conductor-v0.20.0
[0.19.0]: https://github.com/astriaorg/astria/compare/conductor-v0.18.0...conductor-v0.19.0
[0.18.0]: https://github.com/astriaorg/astria/compare/conductor-v0.17.0...conductor-v0.18.0
[0.17.0]: https://github.com/astriaorg/astria/compare/conductor-v0.16.0...conductor-v0.17.0
[0.16.0]: https://github.com/astriaorg/astria/compare/conductor-v0.15.0...conductor-v0.16.0
[0.15.0]: https://github.com/astriaorg/astria/compare/conductor-v0.14.0...conductor-v0.15.0
[0.14.0]: https://github.com/astriaorg/astria/compare/conductor-v0.13.1...conductor-v0.14.0
[0.13.1]: https://github.com/astriaorg/astria/compare/conductor-v0.13.0...conductor-v0.13.1
[0.13.0]: https://github.com/astriaorg/astria/compare/conductor-v0.12.0...conductor-v0.13.0
[0.12.0]: https://github.com/astriaorg/astria/compare/conductor-v0.11.1...conductor-v0.12.0
[0.11.1]: https://github.com/astriaorg/astria/compare/conductor-v0.11.0...conductor-v0.11.1
[0.11.0]: https://github.com/astriaorg/astria/compare/v0.10.1--conductor...v0.11.0--conductor
[0.10.0]: https://github.com/astriaorg/astria/compare/v0.9.0--conductor...v0.10.0--conductor
[0.9.0]: https://github.com/astriaorg/astria/compare/v0.8.0--conductor...v0.9.0--conductor
[0.8.0]: https://github.com/astriaorg/astria/compare/v0.7.0--conductor...v0.8.0--conductor
[0.7.0]: https://github.com/astriaorg/astria/compare/v0.6.1--conductor...v0.7.0--conductor
[0.6.1]: https://github.com/astriaorg/astria/compare/v0.6.0--conductor...v0.6.1--conductor
[0.6.0]: https://github.com/astriaorg/astria/compare/v0.5.1--conductor...v0.6.0--conductor
[0.5.1]: https://github.com/astriaorg/astria/compare/v0.5.0--conductor...v0.5.1--conductor
