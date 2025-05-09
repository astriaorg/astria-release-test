<!-- markdownlint-disable no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0](https://github.com/astriaorg/astria-release-test/compare/cli-vv0.6.0...cli-vv0.7.0) (2025-05-09)


### ⚠ BREAKING CHANGES

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

## [Unreleased]

## [0.6.0] - 2025-03-06

### Added

- Add `fee-assets` subcommand to `sequencer` CLI [#1816](https://github.com/astriaorg/astria/pull/1816).
- Add option `--withdrawer-address <ADDRESS>` to `init-bridge-account` subcommand
[#2055](https://github.com/astriaorg/astria/pull/2055).

### Changed

- Bump MSRV to 1.83.0 [#1857](https://github.com/astriaorg/astria/pull/1857).
- Update `idna` dependency to resolve cargo audit warning [#1869](https://github.com/astriaorg/astria/pull/1869).
- Remove default values from `--sequencer.chain-id` and `--sequencer-url` arguments
  [#1792](https://github.com/astriaorg/astria/pull/1792)

### Fixed

- Fix ICS20 withdrawal source when using channel with more than one
  port/channel combo. [#1768](https://github.com/astriaorg/astria/pull/1768)

## [0.5.1] - 2024-10-23

### Added

- Implement frost_ed25519 threshold signing CLI [#1654](https://github.com/astriaorg/astria/pull/1654).
- Add `sign` and `submit` subcommands to `sequencer` CLI [#1696](https://github.com/astriaorg/astria/pull/1696).

### Changed

- Return Bech32m Prefixed Address [#1621](https://github.com/astriaorg/astria/pull/1621).

## [0.5.0] - 2024-10-17

### Added

- Add command to perform ics20 withdrawals [#1631](https://github.com/astriaorg/astria/pull/1631).

### Changed

- Replace `once_cell` with `LazyLock` [#1576](https://github.com/astriaorg/astria/pull/1576).
- Migrate all instances of `#[allow]` to `#[expect]` [#1561](https://github.com/astriaorg/astria/pull/1561).
- Merge argument parsing and command execution [#1568](https://github.com/astriaorg/astria/pull/1568).
- Remove action suffix from all action types [#1630](https://github.com/astriaorg/astria/pull/1630).
- Prefer `astria.primitive.v1.RollupId` over bytes [#1661](https://github.com/astriaorg/astria/pull/1661).
- Call transactions `Transaction`, contents `TransactionBody` [#1650](https://github.com/astriaorg/astria/pull/1650).
- Rename sequence action to rollup data submission [#1665](https://github.com/astriaorg/astria/pull/1665).
- Upgrade to proto `v1`s throughout [#1672](https://github.com/astriaorg/astria/pull/1672).

### Fixed

- Migrate from `broadcast_tx_commit` to `broadcast_tx_sync` [#1376](https://github.com/astriaorg/astria/pull/1376).
- Ensure `checkTx` passes before waiting for inclusion [#1636](https://github.com/astriaorg/astria/pull/1636).

## [0.4.1] - 2024-09-06

### Fixed

- Don't fail entire block due to bad withdraw event [#1409](https://github.com/astriaorg/astria/pull/1409).

## [0.4.0] - 2024-08-28

### Changed

- Update to support dusk-10 as default network [#1418](https://github.com/astriaorg/astria/pull/1418).

## [0.3.1] - 2024-01-23

### Changed

- Bump rpc websocket for dusk-3 [#705](https://github.com/astriaorg/astria/pull/705).

## [0.3.0] - 2024-01-23

### Changed

- Update licenses [#706](https://github.com/astriaorg/astria/pull/706).

### Fixed

- Refactor yaml serialization to match format in rollup's values.yaml [#707](https://github.com/astriaorg/astria/pull/707).

## [0.2.2] - 2024-01-18

### Changed

- Bump for dusk-3 [#689](https://github.com/astriaorg/astria/pull/689).

## [0.2.1] - 2023-12-19

### Changed

- New release with new chart version [#658](https://github.com/astriaorg/astria/pull/658).

## 0.2.0 - 2023-12-11

### Changed

- Update to work with latest rollup charts, and utilize dusk-2 network.

## 0.1.0 - 2023-12-11

### Added

- Dusk 1 CLI release

[unreleased]: https://github.com/astriaorg/astria/compare/cli-v0.6.0...HEAD
[0.6.0]: https://github.com/astriaorg/astria/compare/cli-v0.5.1...cli-v0.6.0
[0.5.1]: https://github.com/astriaorg/astria/compare/cli-v0.5.0...cli-v0.5.1
[0.5.0]: https://github.com/astriaorg/astria/compare/cli-v0.4.1...cli-v0.5.0
[0.4.1]: https://github.com/astriaorg/astria/compare/cli-v0.4.0...cli-v0.4.1
[0.3.1]: https://github.com/astriaorg/astria/compare/cli-v0.3.0...cli-v0.3.1
[0.3.0]: https://github.com/astriaorg/astria/compare/cli-v0.2.2...cli-v0.3.0
[0.2.2]: https://github.com/astriaorg/astria/compare/cli-v0.2.1...cli-v0.2.2
[0.2.1]: https://github.com/astriaorg/astria/compare/cli-v0.2.0...cli-v0.2.1
