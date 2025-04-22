# Changelog

## [2.0.0](https://github.com/astriaorg/astria-release-test/compare/composer-vv1.0.1...composer-vv2.0.0) (2025-04-22)


### ⚠ BREAKING CHANGES

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
* **composer:** add capacity to bundle factory ([#937](https://github.com/astriaorg/astria-release-test/issues/937)) ([3f23291](https://github.com/astriaorg/astria-release-test/commit/3f2329177c5c26a509c247f1cef7f0f6ab83cfa0))
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
