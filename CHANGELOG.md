# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Add support for Serde behind `serde` feature.

### Changed
- Extra safeguards against constructing and using a nullary heap.
- Simpler unstable Cargo features: `unstable` for everything available on stable
  compilers (previously `drain_sorted`, `into_iter_sorted`, and `retain`) and
  `unstable_nightly` for everything only available on nightly (previously
  `exact_size_is_empty`, `extend_one`, `shrink_to`, and `trusted_len`).

## [0.1.0] &ndash; 2020-09-26
### Added
- `DaryHeap` based on `std::collections::BinaryHeap`.
- `Arity` trait and `arity` macro to specify heap arity.
- Arity markers for two to eight, `D2`&ndash;`D8`, and type aliases for heaps
  with those arities, `BinaryHeap`&ndash;`OctonaryHeap`.
- Cargo features corresponding to unstable Rust features, specifically the
  features `drain_sorted`, `into_iter_sorted`, and `retain` that are available
  on stable compilers, and the features `exact_size_is_empty`, `extend_one`,
  `shrink_to`, and `trusted_len` that are only available on nightly compilers.
- Benchmarks to compare performance against standard library and between
  different arities.
- Fuzzing to catch possible bugs.

[Unreleased]: https://github.com/hanmertens/dary_heap/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/hanmertens/dary_heap/releases/tag/v0.1.0
