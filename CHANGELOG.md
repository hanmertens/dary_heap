# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Use const generics to specify arity instead of `Arity` trait.
- Raise MSRV to 1.51.0 for const generics support.

## [0.2.3] &ndash; 2021-03-27
### Changed
- Synchronize source code with standard library of Rust version 1.51.0.
- Performance improvement for `DaryHeap::append`.

## [0.2.2] &ndash; 2021-01-13
### Changed
- Synchronize source code with standard library of Rust version 1.49.0.
- Performance improvements, especially for arities up to four due to specialized
  code for those arities.

## [0.2.1] &ndash; 2020-11-20
### Added
- Implement `SourceIter` and `InplaceIterable` for `IntoIter` when
  `unstable_nightly` is enabled.

### Changed
- Synchronize source code with standard library of Rust version 1.48.0.

## [0.2.0] &ndash; 2020-10-26
### Changed
- Change `serde` serialization format to be the same as sequence types in the
  standard library like `std::collections::BinaryHeap`.
- MSRV lowered to 1.31.0, with caveats (`Vec::from(DaryHeap)` requires 1.41.0+;
  `no_std` support and `serde` feature require 1.36.0+).

### Fixed
- Ensure heaps are valid after deserializing via `serde`.

## [0.1.1] &ndash; 2020-10-08
### Added
- Add support for Serde behind `serde` feature.
- Establish stability guidelines and set MSRV at 1.41.0.

### Changed
- Extra safeguards against constructing and using a nullary heap.
- Simpler unstable Cargo features: `unstable` for everything available on stable
  compilers (previously `drain_sorted`, `into_iter_sorted`, and `retain`) and
  `unstable_nightly` for everything only available on nightly (previously
  `exact_size_is_empty`, `extend_one`, `shrink_to`, and `trusted_len`).
- Synchronize source code with standard library of Rust version 1.47.0.

### Fixed
- Fix division by zero for unary heap in `append`.

## [0.1.0] &ndash; 2020-09-26
### Added
- `DaryHeap` based on `std::collections::BinaryHeap` (Rust version 1.46.0).
- `Arity` trait and `arity` macro to specify heap arity.
- Arity markers for two to eight, `D2`&ndash;`D8`, and type aliases for heaps
  with those arities, `BinaryHeap`&ndash;`OctonaryHeap`.
- Cargo features corresponding to unstable Rust features, specifically the
  features `drain_sorted`, `into_iter_sorted`, and `retain` that are available
  on stable compilers, and the features `exact_size_is_empty`, `extend_one`,
  `shrink_to`, and `trusted_len` that are only available on nightly compilers.

[Unreleased]: https://github.com/hanmertens/dary_heap/compare/v0.2.3...HEAD
[0.2.3]: https://github.com/hanmertens/dary_heap/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/hanmertens/dary_heap/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/hanmertens/dary_heap/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/hanmertens/dary_heap/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/hanmertens/dary_heap/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/hanmertens/dary_heap/releases/tag/v0.1.0
