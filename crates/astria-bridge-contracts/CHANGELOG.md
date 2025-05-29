# Changelog

## [0.1.2](https://github.com/astriaorg/astria-release-test/compare/bridge-contracts-v0.1.1...bridge-contracts-v0.1.2) (2025-05-29)

## [0.1.1](https://github.com/astriaorg/astria-release-test/compare/bridge-contracts-v0.1.0...bridge-contracts-v0.1.1) (2025-05-29)

<!-- markdownlint-disable no-duplicate-heading -->

## Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Update `idna` dependency to resolve cargo audit warning [#1869](https://github.com/astriaorg/astria/pull/1869).

### Fixed

- Read the provided contract's `decimals` function, falling back to a hardcoded
  value of 18 if the call fails.
  [#1762](https://github.com/astriaorg/astria/pull/1762)

### Added

- Initial release.

### Fixed

- Fixed ICS20 withdrawal source when using channel with more than one
  port/channel combo. [#1768](https://github.com/astriaorg/astria/pull/1768)
