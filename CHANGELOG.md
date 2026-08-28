# Changelog

## [0.1.1](https://github.com/cooper-square-technologies/profound-rust-sdk/compare/v0.1.0...v0.1.1) (2026-08-28)


### Chores

* release 0.1.1 ([729d5f4](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/729d5f4895f46b16e23f2fd2c86676fad5ca0ece))
* release 0.1.1 ([7300c85](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/7300c8531c187e82482f2a7996715fee0f5224b0))

## [0.1.0](https://github.com/cooper-square-technologies/profound-rust-sdk/compare/v0.0.1...v0.1.0) (2026-08-28)


### Features

* **api:** initial SDK generation ([cda7777](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/cda777735e063e1cc7a042f8e88a3386c33398c2))


### Chores

* **api:** regenerate SDK ([3f8abbb](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/3f8abbb3e7c7d5f312f5069398a995ceac079521))
* **api:** update generated SDK content ([58cfb76](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/58cfb760b169648388ab2d04453a451bf68c39d5))
* set version to 0.0.1 ([5344cf5](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/5344cf5b546ed5e1812c33688c1dc9cb7f195000))
* set version to 0.0.1 so the first release is 0.1.0 ([8abbcbe](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/8abbcbe3bff1da606dbf3ed36fdd8cb39f4038ed))

## Changelog

All notable changes to `profound` are documented here. Release
tooling appends a section per released version below.

## Unreleased

- Initial generation of the `profound` SDK.
- Response-only models are marked `#[non_exhaustive]`, so new response
  fields can be added in future versions without a breaking release;
  request models stay literally constructible.
