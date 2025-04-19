# Changelog

## [2.0.0](https://github.com/astriaorg/astria-release-test/compare/sequencer-relayer-vv1.0.1...sequencer-relayer-vv2.0.0) (2025-04-19)


### ⚠ BREAKING CHANGES

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
* **core:** remove `SequencerBlock::try_from_cometbft` ([#1005](https://github.com/astriaorg/astria-release-test/issues/1005)) ([0eb56c9](https://github.com/astriaorg/astria-release-test/commit/0eb56c9ca5c36b8dc095afeea153e61a1e1590f0)), closes [#969](https://github.com/astriaorg/astria-release-test/issues/969)


### Tests

* **sequencer-relayer:** reinstate black box tests ([#1033](https://github.com/astriaorg/astria-release-test/issues/1033)) ([83d23ab](https://github.com/astriaorg/astria-release-test/commit/83d23ab147b473936cb7f0638fee7651a3c33b3c))
