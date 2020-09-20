# dary_heap

Rust implementation of a [*d*-ary heap][wiki]. The *d* = 2 version is present in
the standard library as [`BinaryHeap`][std-binaryheap], but using a higher value
for *d* can bring performance improvements in many use cases. Benchmark your use
case to determine a good value of *d*.

The API of this crate aims to be analogous to that of [`BinaryHeap` in the
standard library][std-binaryheap]. Feature-gated API in the standard library is
also feature-gated in `dary_heap`, see [the section on features](#features) for
more information. In fact, the code in `dary_heap` is directly based on that of
the standard library.

## Features

The following features are available on stable compilers:
- `drain_sorted`: add `drain_sorted` method which is like `drain` but yields
  elements in heap order.
- `into_iter_sorted`: add `into_iter_sorted` method which is like `into_iter`
  but yields elements in heap order.
- `retain`: add `retain` function that retains only those elements in the heap
  specified by the predicate.

The following features require unstable compiler features and are therefore
limited to nightly compilers:
- `exact_size_is_empty`: implement methods defined by unstable feature
  `exact_size_is_empty` on `ExactSizeIterator`s in this crate.
- `extend_one`: implement methods defined by unstable feature `extend_one`.
- `shrink_to`: adds `shrink_to` method to shrink heap capacity to a lower bound.
- `specialization_extend`: use specialization to potentially speed up the
  `extend` method when extending with another *d*-ary heap (with the same *d*).
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
