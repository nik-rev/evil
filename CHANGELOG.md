# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/nik-rev/evil/compare/v0.2.1...HEAD

## [v0.2.1] - 2026-03-04

[v0.2.1]: https://github.com/nik-rev/evil/compare/v0.2.0...v0.2.1

- Documentation improvements

## [v0.2.0] - 2026-03-04

[v0.2.0]: https://github.com/nik-rev/evil/compare/v0.1.3...v0.2.0

### Added

- Added methods `into_ok`, `as_ok`, and `as_ok_mut` for extracting value inside the `Ok` variant 
- Implemented `Copy` and `Clone`
- Implemented `PartialOrd` and `Ord`
- Implemented `PartialEq` and `Eq`
- Added `FromResidual` implementation for `Poll` types
- Implemented `IntoIterator` for `&evil::Result`, `&mut evil::Result` and `Result`
- Implemented `Sum` and `Product` on iterators of `Result`, `evil::Result`, and `Option`s

### Fixed

- Fixed the `Termination` implementation to be the same as `std`'s

## [v0.1.3] - 2026-03-04

[v0.1.3]: https://github.com/nik-rev/evil/compare/v0.1.2...v0.1.3

- Documentation improvements

## [v0.1.2] - 2026-03-04

[v0.1.2]: https://github.com/nik-rev/evil/compare/v0.1.1...v0.1.2

- Made the panic messages less verbose

## [v0.1.1] - 2026-03-04

[v0.1.1]: https://github.com/nik-rev/evil/compare/v0.1.0...v0.1.1

- Update `description` field
