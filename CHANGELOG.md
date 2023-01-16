# Changelog

## [0.8.0](https://github.com/beckler/ahoy/compare/v0.7.2...v0.8.0) (2023-01-16)


### Features

* updated repo endpoint to pull releases ([f46d400](https://github.com/beckler/ahoy/commit/f46d400e5fbf728e01d350d8547e1ff02c718c10))

## [0.7.2](https://github.com/beckler/ahoy/compare/v0.7.1...v0.7.2) (2022-10-21)


### Bug Fixes

* add zip deflate algo to self update feature - windows specific ([2909c1b](https://github.com/beckler/ahoy/commit/2909c1bcbe03cff52a22f7b4e9e285e47e37d90a))

## [0.7.1](https://github.com/beckler/ahoy/compare/v0.7.0...v0.7.1) (2022-10-18)


### Bug Fixes

* added ts/trs bridge instructions to popup modal ([92119c5](https://github.com/beckler/ahoy/commit/92119c5342a55ae74b306fed87ea56bed113598f))

## [0.7.0](https://github.com/beckler/ahoy/compare/v0.6.2...v0.7.0) (2022-10-14)


### Features

* added interactive self-update! ([48013ca](https://github.com/beckler/ahoy/commit/48013ca8712b2106701dc06045a9d1511958e56d))


### Bug Fixes

* added app version in bottom corner ([b6aa00b](https://github.com/beckler/ahoy/commit/b6aa00b32353aa42dc4b5bca1a145cb69ee6a1a3))

## [0.6.2](https://github.com/beckler/ahoy/compare/v0.6.1...v0.6.2) (2022-09-13)


### Bug Fixes

* added hardware revision check ([359544d](https://github.com/beckler/ahoy/commit/359544de859a9f249cecf09186a6d5df73d2bafc))
* updated readme, [#18](https://github.com/beckler/ahoy/issues/18) is resolved ([637429d](https://github.com/beckler/ahoy/commit/637429dcf613bbb02917bf0140ecbe57e1490aa6))
* updated readme, [#20](https://github.com/beckler/ahoy/issues/20) is resolved ([715bd7f](https://github.com/beckler/ahoy/commit/715bd7fb35c3c001b72c4b116f8a17b6bcae3072))

## [0.6.1](https://github.com/beckler/ahoy/compare/v0.6.0...v0.6.1) (2022-09-02)


### Bug Fixes

* cleanup ([857078f](https://github.com/beckler/ahoy/commit/857078f9bfcde27f6cbf7e59b545b9d5dd523237))
* have progress bar working... finally ([56676c2](https://github.com/beckler/ahoy/commit/56676c2d28165fa59ee331e539a3b0ca0beaf15e))
* resolves [#19](https://github.com/beckler/ahoy/issues/19) and [#20](https://github.com/beckler/ahoy/issues/20) ([c61708d](https://github.com/beckler/ahoy/commit/c61708df52a2d1b6cf15a10dfd5dc0ff06394897))
* simplifying message passing, and attempting to improve ui performance ([4c70c4f](https://github.com/beckler/ahoy/commit/4c70c4f42d40aa5f58ea01512963fe9b8eb8a1c3))
* stupid mistake with vid/pid order ([084956e](https://github.com/beckler/ahoy/commit/084956e39b94a83fd4b0044a45f6fb09eff7579e))

## [0.6.0](https://github.com/beckler/ahoy/compare/v0.5.0...v0.6.0) (2022-08-31)


### Features

* added driver to msi ([a88177b](https://github.com/beckler/ahoy/commit/a88177bf8cbe311790656c59c61a5739b53e5484))

## [0.5.0](https://github.com/beckler/ahoy/compare/v0.4.1...v0.5.0) (2022-08-30)


### Features

* added wix installer files for windows ([547951d](https://github.com/beckler/ahoy/commit/547951d2417cf3cd8f50378d0cd2fd50c377816d))
* functional windows installer ([57aa658](https://github.com/beckler/ahoy/commit/57aa658cf3a9179150a6fc41f92a71cda7d9bc81))
* major style overhaul ([283892f](https://github.com/beckler/ahoy/commit/283892f2ef5d956e1458389e1bc023e89eb55aa1))
* switched to new async runtime engine; fixes [#15](https://github.com/beckler/ahoy/issues/15) ([1d48695](https://github.com/beckler/ahoy/commit/1d48695ffaaffdde7bab985ff1466d8a3652b90a))


### Bug Fixes

* no idea is this is gonna fix shit ([96d1047](https://github.com/beckler/ahoy/commit/96d1047802d074a1e8a1e5523f9e872302e09aec))
* windows issues ([b17fc64](https://github.com/beckler/ahoy/commit/b17fc6480664f48e1571f98e62365ab67d4f29ce))
* windows issues ([6fd7aa1](https://github.com/beckler/ahoy/commit/6fd7aa1bbad4fe8be60d16aaa3581cc17527e31b))

## [0.4.1](https://github.com/beckler/ahoy/compare/v0.4.0...v0.4.1) (2022-08-16)


### Bug Fixes

* build process ([994d56b](https://github.com/beckler/ahoy/commit/994d56be550a697c31283e5ec139385852c85f11))

## [0.4.0](https://github.com/beckler/ahoy/compare/v0.3.3...v0.4.0) (2022-08-16)


### Features

* added new flag for cli install ([f5e60b9](https://github.com/beckler/ahoy/commit/f5e60b9c28aeec906e0c9964ab8d9ce1f2b85bd9))
* added self-update cli command ([69b1d50](https://github.com/beckler/ahoy/commit/69b1d50d15c25028d9fc159c193a9000885835e0))


### Bug Fixes

* new approach for libusb on windows ([348baa0](https://github.com/beckler/ahoy/commit/348baa0e4dd5b6974030c89dd5bb46ec61733832))
* removed windows arm build due to issues ([807e052](https://github.com/beckler/ahoy/commit/807e05202a71cda679e43e02fb4ef9c0a42d2fcf))

## [0.3.3](https://github.com/beckler/ahoy/compare/v0.3.2...v0.3.3) (2022-08-12)


### Bug Fixes

* added 32bit and arm windows, removing cross from linux build ([5f55905](https://github.com/beckler/ahoy/commit/5f5590537fc9a64abe141eab78c3bb9354bad169))

## [0.3.2](https://github.com/beckler/ahoy/compare/v0.3.1...v0.3.2) (2022-08-12)


### Bug Fixes

* added cross args ([750f04f](https://github.com/beckler/ahoy/commit/750f04f1c5d1e148dc555d545327361dd5e6fee7))

## [0.3.1](https://github.com/beckler/ahoy/compare/v0.3.0...v0.3.1) (2022-08-12)


### Bug Fixes

* different approach for the static linkage ([97a565d](https://github.com/beckler/ahoy/commit/97a565d67fcec828fc874c0e0dc6e8420f482edb))

## [0.3.0](https://github.com/beckler/ahoy/compare/v0.2.4...v0.3.0) (2022-08-12)


### Features

* display device name instead of device type ([455ab6a](https://github.com/beckler/ahoy/commit/455ab6a000096c2e3190cfc3192bd6e151e9dfc3))


### Bug Fixes

* may have figured out the static linkage issue and libusb ([be207f5](https://github.com/beckler/ahoy/commit/be207f5f401624f981c6b8a9b47efa92f35890c5))

## [0.2.4](https://github.com/beckler/ahoy/compare/v0.2.3...v0.2.4) (2022-08-11)


### Bug Fixes

* more build pipeline changes ([319e78f](https://github.com/beckler/ahoy/commit/319e78f74460fd46bfe7ad0abc3270e5687219ed))

## [0.2.3](https://github.com/beckler/ahoy/compare/v0.2.2...v0.2.3) (2022-08-11)


### Bug Fixes

* small updates, more build pipeline changes ([740364b](https://github.com/beckler/ahoy/commit/740364b7615c256fafb7e70c2adf10232e9de296))

## [0.2.2](https://github.com/beckler/ahoy/compare/v0.2.1...v0.2.2) (2022-08-11)


### Bug Fixes

* moved to remote lib ([4b7878f](https://github.com/beckler/ahoy/commit/4b7878fe8b276c6aba2a58d7f31ea6e58c89ca61))
* resolved all clippy items ([f7b0102](https://github.com/beckler/ahoy/commit/f7b010219c12f732ef64ca8aa7ec3c8486eb9cd1))

## [0.2.1](https://github.com/beckler/ahoy/compare/v0.2.0...v0.2.1) (2022-08-07)


### Bug Fixes

* added pkg-config vars to build script ([fd2322b](https://github.com/beckler/ahoy/commit/fd2322b910422df51095cf565242bdfbdc426086))

## [0.2.0](https://github.com/beckler/ahoy/compare/v0.1.0...v0.2.0) (2022-08-07)


### Features

* added pirate-midi specific serial functionality lib as embedded dep ([e774b79](https://github.com/beckler/ahoy/commit/e774b79595ee296eb8937ed1b1c4ea5fb1dafd00))


### Bug Fixes

* removed --locked flag from build ([fe905ac](https://github.com/beckler/ahoy/commit/fe905acd2826ffc15332c3e83038d0579fb78618))

## 0.1.0 (2022-08-07)


### Features

* added pirate-midi specific serial functionality lib as embedded dep ([e774b79](https://github.com/beckler/ahoy/commit/e774b79595ee296eb8937ed1b1c4ea5fb1dafd00))
