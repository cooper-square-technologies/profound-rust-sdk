# Changelog

## [0.2.0](https://github.com/cooper-square-technologies/profound-rust-sdk/compare/v0.1.0...v0.2.0) (2026-08-28)


### Features

* **api:** initial SDK generation ([cda7777](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/cda777735e063e1cc7a042f8e88a3386c33398c2))


### Chores

* **api:** update generated SDK content ([58cfb76](https://github.com/cooper-square-technologies/profound-rust-sdk/commit/58cfb760b169648388ab2d04453a451bf68c39d5))

## Changelog

All notable changes to `profound` are documented here. Release
tooling appends a section per released version below.

## Unreleased

- Initial generation of the `profound` SDK.
- Response-only models are marked `#[non_exhaustive]`, so new response
  fields can be added in future versions without a breaking release;
  request models stay literally constructible.
