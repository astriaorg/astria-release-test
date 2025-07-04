<!-- markdownlint-disable no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.4](https://github.com/astriaorg/astria-release-test/compare/cli-v0.6.3...cli-v0.6.4) (2025-05-30)


### Bug Fixes

* add documentation for Astria CLI, Composer & Sequencer ([#73](https://github.com/astriaorg/astria-release-test/issues/73)) ([003d2ba](https://github.com/astriaorg/astria-release-test/commit/003d2ba208bc548ec6d07901269e3dbf62c2da8d))

## [0.6.3](https://github.com/astriaorg/astria-release-test/compare/cli-v0.6.2...cli-v0.6.3) (2025-05-29)

## [0.6.2](https://github.com/astriaorg/astria-release-test/compare/cli-v0.6.1...cli-v0.6.2) (2025-05-16)


### Bug Fixes

* **cli:** added more comments ([#36](https://github.com/astriaorg/astria-release-test/issues/36)) ([ec95d87](https://github.com/astriaorg/astria-release-test/commit/ec95d8704f992b3ca493d50bc7d97d8b88061696))

## [0.6.1](https://github.com/astriaorg/astria-release-test/compare/cli-v0.6.0...cli-v0.6.1) (2025-05-16)


### Bug Fixes

* **cli:** added some comments ([#30](https://github.com/astriaorg/astria-release-test/issues/30)) ([fb46a32](https://github.com/astriaorg/astria-release-test/commit/fb46a320fff67d204c1cf7d9e61526e4b30ac675))

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
