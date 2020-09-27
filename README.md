# dary_heap

[![CI](https://github.com/hanmertens/dary_heap/workflows/CI/badge.svg)](https://github.com/hanmertens/dary_heap/actions?query=workflow%3ACI)

Rust implementation of a [*d*-ary heap][wiki]. The *d* = 2 version is present in
the standard library as [`BinaryHeap`][std-binaryheap], but using a higher value
for *d* can bring performance improvements in many use cases. This is because a
higher arity *d* (maximum number of children each node can have) means less
layers, but potentially more work per layer because there are more children.
This generally makes adding elements to the heap faster, but removing elements
slower. However, the latter effect is often less due to higher cache locality.
Therefore, overall performance is often increased if *d* > 2 but not too high.
If you're unsure what value of *d* to choose, the `QuaternaryHeap` with *d* = 4
is usually a good start, but benchmarking is necessary to determine the best
value of *d*.

The API of this crate aims to be analogous to that of [`BinaryHeap` in the
standard library][std-binaryheap]. Feature-gated API in the standard library is
also feature-gated in `dary_heap`, see [the section on features](#features) for
more information. In fact, the code in `dary_heap` is directly based on that of
the standard library. The `BinaryHeap` provided by this crate should therefore
provide similar performance as that of the standard library.

## Features

The following features are available on stable compilers:
- `drain_sorted`: add `drain_sorted` method which is like `drain` but yields
  elements in heap order.
- `into_iter_sorted`: add `into_iter_sorted` method which is like `into_iter`
  but yields elements in heap order.
- `retain`: add `retain` function that retains only those elements in the heap
  specified by the predicate.
- `serde`: add support for (de)serialization using [Serde][serde].

The following features require unstable compiler features and are therefore
limited to nightly compilers:
- `exact_size_is_empty`: implement methods defined by unstable feature
  `exact_size_is_empty` on `ExactSizeIterator`s in this crate.
- `extend_one`: implement methods defined by unstable feature `extend_one`.
- `shrink_to`: add `shrink_to` method to shrink heap capacity to a lower bound.
- `trusted_len`: implement `TrustedLen` for iterators if possible. Only has an
  effect when `drain_sorted` or `into_iter_sorted` is also enabled.

## License

`dary_heap` is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[wiki]: https://en.wikipedia.org/wiki/D-ary_heap
[std-binaryheap]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
[serde]: https://serde.rs
