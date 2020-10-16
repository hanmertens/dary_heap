//! A priority queue implemented with a *d*-ary heap.
//!
//! Insertion and popping the largest element have *O*(log(*n*)) time complexity.
//! Checking the largest element is *O*(1). Converting a vector to a *d*-ary heap
//! can be done in-place, and has *O*(*n*) complexity. A *d*-ary heap can also be
//! converted to a sorted vector in-place, allowing it to be used for an *O*(*n* \* log(*n*))
//! in-place heapsort.
//!
//! # Comparison to standard library
//!
//! The standard library contains a 2-ary heap
//! ([`std::collections::BinaryHeap`][std]). The [`BinaryHeap`] of this crate
//! aims to be a drop-in replacement, both in API and in performance. Cargo
//! features are used in place of unstable Rust features. The advantage of this
//! crate over the standard library lies in the possibility of easily changing
//! the arity of the heap, which can increase performance.
//!
//! [std]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
//!
//! # Comparison of different arities *d*
//!
//! The arity *d* is defined as the maximum number of children each node can
//! have. A higher number means the heap has less layers, but may require more
//! work per layer because there are more children present. This generally makes
//! methods adding elements to the heap such as [`push`] faster, and methods
//! removing them such as [`pop`] slower. However, due to higher cache locality
//! for higher *d*, the drop in [`pop`] performance is often diminished. If you're
//! unsure what value of *d* to choose, the [`QuaternaryHeap`] with *d* = 4 is
//! usually a good start, but benchmarking is necessary to determine the best
//! value of *d*.
//!
//! [`push`]: struct.DaryHeap.html#method.push
//! [`pop`]: struct.DaryHeap.html#method.pop
//!
//! # Usage
//!
//! Rust type interference cannot infer the desired heap arity (value of *d*)
//! automatically when using [`DaryHeap`] directly. It is therefore more
//! ergonomic to  use one of the type aliases to select the desired arity:
//!
//! | Name               | Arity   |
//! |--------------------|---------|
//! | [`BinaryHeap`]     | *d* = 2 |
//! | [`TernaryHeap`]    | *d* = 3 |
//! | [`QuaternaryHeap`] | *d* = 4 |
//! | [`QuinaryHeap`]    | *d* = 5 |
//! | [`SenaryHeap`]     | *d* = 6 |
//! | [`SeptenaryHeap`]  | *d* = 7 |
//! | [`OctonaryHeap`]   | *d* = 8 |
//!
//! The difference in ergonomics illustrated in the following:
//!
//! ```
//! use dary_heap::{DaryHeap, D3, TernaryHeap};
//!
//! // Type parameter T can be inferred, but arity cannot
//! let mut heap1 = DaryHeap::<_, D3>::new();
//! heap1.push(42);
//!
//! // Type alias removes need for explicit type
//! let mut heap2 = TernaryHeap::new();
//! heap2.push(42);
//! ```
//!
//! If a different arity is desired, you can use the [`arity`] macro or
//! implement the  necessary trait [`Arity`] yourself. It should be noted that
//! *d* > 8 is rarely beneficial.
//!
//! ```
//! use dary_heap::{arity, DaryHeap};
//!
//! arity! { pub(crate) D9 = 9; }
//! pub(crate) type NovenaryHeap<T> = DaryHeap<T, D9>;
//! ```
//!
//! [`DaryHeap`]: struct.DaryHeap.html
//! [`BinaryHeap`]: type.BinaryHeap.html
//! [`TernaryHeap`]: type.TernaryHeap.html
//! [`QuaternaryHeap`]: type.QuaternaryHeap.html
//! [`QuinaryHeap`]: type.QuinaryHeap.html
//! [`SenaryHeap`]: type.SenaryHeap.html
//! [`SeptenaryHeap`]: type.SeptenaryHeap.html
//! [`OctonaryHeap`]: type.OctonaryHeap.html
//! [`arity`]: macro.arity.html
//! [`Arity`]: trait.Arity.html
//!
//! # Examples
//!
//! This is a larger example that implements [Dijkstra's algorithm][dijkstra]
//! to solve the [shortest path problem][sssp] on a [directed graph][dir_graph].
//! It shows how to use [`DaryHeap`] with custom types.
//!
//! [dijkstra]: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
//! [sssp]: https://en.wikipedia.org/wiki/Shortest_path_problem
//! [dir_graph]: https://en.wikipedia.org/wiki/Directed_graph
//! [`DaryHeap`]: struct.DaryHeap.html
//!
//! ```
//! use std::cmp::Ordering;
//! use dary_heap::TernaryHeap;
//!
//! #[derive(Copy, Clone, Eq, PartialEq)]
//! struct State {
//!     cost: usize,
//!     position: usize,
//! }
//!
//! // The priority queue depends on `Ord`.
//! // Explicitly implement the trait so the queue becomes a min-heap
//! // instead of a max-heap.
//! impl Ord for State {
//!     fn cmp(&self, other: &State) -> Ordering {
//!         // Notice that the we flip the ordering on costs.
//!         // In case of a tie we compare positions - this step is necessary
//!         // to make implementations of `PartialEq` and `Ord` consistent.
//!         other.cost.cmp(&self.cost)
//!             .then_with(|| self.position.cmp(&other.position))
//!     }
//! }
//!
//! // `PartialOrd` needs to be implemented as well.
//! impl PartialOrd for State {
//!     fn partial_cmp(&self, other: &State) -> Option<Ordering> {
//!         Some(self.cmp(other))
//!     }
//! }
//!
//! // Each node is represented as an `usize`, for a shorter implementation.
//! struct Edge {
//!     node: usize,
//!     cost: usize,
//! }
//!
//! // Dijkstra's shortest path algorithm.
//!
//! // Start at `start` and use `dist` to track the current shortest distance
//! // to each node. This implementation isn't memory-efficient as it may leave duplicate
//! // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
//! // for a simpler implementation.
//! fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
//!     // dist[node] = current shortest distance from `start` to `node`
//!     let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
//!
//!     let mut heap = TernaryHeap::new();
//!
//!     // We're at `start`, with a zero cost
//!     dist[start] = 0;
//!     heap.push(State { cost: 0, position: start });
//!
//!     // Examine the frontier with lower cost nodes first (min-heap)
//!     while let Some(State { cost, position }) = heap.pop() {
//!         // Alternatively we could have continued to find all shortest paths
//!         if position == goal { return Some(cost); }
//!
//!         // Important as we may have already found a better way
//!         if cost > dist[position] { continue; }
//!
//!         // For each node we can reach, see if we can find a way with
//!         // a lower cost going through this node
//!         for edge in &adj_list[position] {
//!             let next = State { cost: cost + edge.cost, position: edge.node };
//!
//!             // If so, add it to the frontier and continue
//!             if next.cost < dist[next.position] {
//!                 heap.push(next);
//!                 // Relaxation, we have now found a better way
//!                 dist[next.position] = next.cost;
//!             }
//!         }
//!     }
//!
//!     // Goal not reachable
//!     None
//! }
//!
//! fn main() {
//!     // This is the directed graph we're going to use.
//!     // The node numbers correspond to the different states,
//!     // and the edge weights symbolize the cost of moving
//!     // from one node to another.
//!     // Note that the edges are one-way.
//!     //
//!     //                  7
//!     //          +-----------------+
//!     //          |                 |
//!     //          v   1        2    |  2
//!     //          0 -----> 1 -----> 3 ---> 4
//!     //          |        ^        ^      ^
//!     //          |        | 1      |      |
//!     //          |        |        | 3    | 1
//!     //          +------> 2 -------+      |
//!     //           10      |               |
//!     //                   +---------------+
//!     //
//!     // The graph is represented as an adjacency list where each index,
//!     // corresponding to a node value, has a list of outgoing edges.
//!     // Chosen for its efficiency.
//!     let graph = vec![
//!         // Node 0
//!         vec![Edge { node: 2, cost: 10 },
//!              Edge { node: 1, cost: 1 }],
//!         // Node 1
//!         vec![Edge { node: 3, cost: 2 }],
//!         // Node 2
//!         vec![Edge { node: 1, cost: 1 },
//!              Edge { node: 3, cost: 3 },
//!              Edge { node: 4, cost: 1 }],
//!         // Node 3
//!         vec![Edge { node: 0, cost: 7 },
//!              Edge { node: 4, cost: 2 }],
//!         // Node 4
//!         vec![]];
//!
//!     assert_eq!(shortest_path(&graph, 0, 1), Some(1));
//!     assert_eq!(shortest_path(&graph, 0, 3), Some(3));
//!     assert_eq!(shortest_path(&graph, 3, 0), Some(7));
//!     assert_eq!(shortest_path(&graph, 0, 4), Some(5));
//!     assert_eq!(shortest_path(&graph, 4, 0), None);
//! }
//! ```

#![cfg_attr(has_alloc, no_std)]
#![cfg_attr(
    feature = "unstable_nightly",
    feature(exact_size_is_empty, extend_one, shrink_to, trusted_len)
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::needless_doctest_main)]

#[cfg(has_alloc)]
extern crate alloc;

use core::fmt;
use core::iter::{FromIterator, FusedIterator};
use core::marker::PhantomData;
use core::mem::{size_of, swap, ManuallyDrop};
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::slice;

#[cfg(has_alloc)]
use alloc::{vec, vec::Vec};
#[cfg(not(has_alloc))]
use std::{vec, vec::Vec};

/// Marker to specify arity *d* in a *d*-ary heap.
///
/// # Validity of arities in *d*-ary heaps
///
/// Only arities of two or greater are useful in *d*-ary heap, and are therefore
/// the only ones implemented by default. Lower arities are only possible if you
/// put in the effort to implement them yourself. An arity of one is possible,
/// but yields a heap where every element has one child. This essentially makes
/// it a sorted vector with poor performance. Regarding an arity of zero: this
/// is not statically prevented, but constructing a [`DaryHeap`] with it and
/// using it may (and probably will) result in a runtime panic.
///
/// [`DaryHeap`]: struct.DaryHeap.html
pub trait Arity {
    /// The value of *d*.
    const D: usize;
}

/// Convenience macro to implement `Arity` for a specific number.
///
/// This macro implements [`Arity`] for an unconstructable enum. It is a
/// shorthand such that `A` and `B` are equivalent in the following:
///
/// ```
/// use dary_heap::{arity, Arity};
///
/// arity! { pub A = 3; }
///
/// pub enum B {}
///
/// impl Arity for B {
///     const D: usize = 3;
/// }
/// ```
///
/// # Examples
///
/// ```
/// use dary_heap::{arity, DaryHeap};
///
/// arity! { pub D9 = 9; }
/// pub type NovenaryHeap<T> = DaryHeap<T, D9>;
///
/// arity! {
///     /// For a denary heap
///     D10 = 10;
///     /// For an undenary heap
///     pub(crate) D11 = 11;
/// }
/// type DenaryHeap<T> = DaryHeap<T, D10>;
/// pub(crate) type UndenaryHeap<T> = DaryHeap<T, D11>;
/// ```
///
/// This macro protects against setting the arity to zero as [`DaryHeap`] cannot
/// be used with such an arity. See [the relevant section of the `Arity`
/// trait][validity] for more information.
///
/// ```compile_fail
/// use dary_heap::{arity, DaryHeap};
///
/// arity! { D0 = 0; }
///
/// let heap = DaryHeap::<_, D0>::from(vec![42]);
/// ```
///
/// [`Arity`]: trait.Arity.html
/// [`DaryHeap`]: struct.DaryHeap.html
/// [validity]: trait.Arity.html#validity-of-arities-in-d-ary-heaps
#[macro_export]
macro_rules! arity {
    ($(#[$attr:meta])* $vis:vis $arity:ident = $num:expr; $($t:tt)*) => {
        $(#[$attr])*
        $vis enum $arity {}

        impl $crate::Arity for $arity {
            #[deny(arithmetic_overflow)]
            const D: usize = $num - 1 + 1; // Arity should be greater than zero
        }

        $crate::arity!($($t)*);
    };
    () => {}
}

arity! {
    /// Marker for arity *d* = 2.
    pub D2 = 2;

    /// Marker for arity *d* = 3.
    pub D3 = 3;

    /// Marker for arity *d* = 4.
    pub D4 = 4;

    /// Marker for arity *d* = 5.
    pub D5 = 5;

    /// Marker for arity *d* = 6.
    pub D6 = 6;

    /// Marker for arity *d* = 7.
    pub D7 = 7;

    /// Marker for arity *d* = 8.
    pub D8 = 8;
}

/// A binary heap (*d* = 2).
pub type BinaryHeap<T> = DaryHeap<T, D2>;

/// A ternary heap (*d* = 3).
pub type TernaryHeap<T> = DaryHeap<T, D3>;

/// A quaternary heap (*d* = 4).
pub type QuaternaryHeap<T> = DaryHeap<T, D4>;

/// A quinary heap (*d* = 5).
pub type QuinaryHeap<T> = DaryHeap<T, D5>;

/// A senary heap (*d* = 6).
pub type SenaryHeap<T> = DaryHeap<T, D6>;

/// A septenary heap (*d* = 7).
pub type SeptenaryHeap<T> = DaryHeap<T, D7>;

/// An octonary heap (*d* = 8).
pub type OctonaryHeap<T> = DaryHeap<T, D8>;

/// A priority queue implemented with a *d*-ary heap.
///
/// This will be a max-heap.
///
/// It is a logic error for an item to be modified in such a way that the
/// item's ordering relative to any other item, as determined by the `Ord`
/// trait, changes while it is in the heap. This is normally only possible
/// through `Cell`, `RefCell`, global state, I/O, or unsafe code.
///
/// # Usage
///
/// Rust type interference cannot infer the desired heap arity (value of *d*)
/// automatically. Therefore, it is generally more ergonomic to use one of the
/// [type aliases] instead of `DaryHeap` directly. See the [crate-level
/// documentation][usage] for more information.
///
/// [type aliases]: index.html#types
/// [usage]: index.html#usage
///
/// # Comparison to standard library
///
/// For a comparison with [`std::collections::BinaryHeap`][std], see the [crate-level
/// documentation][comparison].
///
/// [std]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
/// [comparison]: index.html#comparison-to-standard-library
///
/// # Examples
///
/// ```
/// use dary_heap::BinaryHeap;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `BinaryHeap<i32>` in this example).
/// let mut heap = BinaryHeap::new();
///
/// // We can use peek to look at the next item in the heap. In this case,
/// // there's no items in there yet so we get None.
/// assert_eq!(heap.peek(), None);
///
/// // Let's add some scores...
/// heap.push(1);
/// heap.push(5);
/// heap.push(2);
///
/// // Now peek shows the most important item in the heap.
/// assert_eq!(heap.peek(), Some(&5));
///
/// // We can check the length of a heap.
/// assert_eq!(heap.len(), 3);
///
/// // We can iterate over the items in the heap, although they are returned in
/// // a random order.
/// for x in &heap {
///     println!("{}", x);
/// }
///
/// // If we instead pop these scores, they should come back in order.
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), None);
///
/// // We can clear the heap of any remaining items.
/// heap.clear();
///
/// // The heap should now be empty.
/// assert!(heap.is_empty())
/// ```
///
/// ## Min-heap
///
/// Either `std::cmp::Reverse` or a custom `Ord` implementation can be used to
/// make `DaryHeap` a min-heap. This makes `heap.pop()` return the smallest
/// value instead of the greatest one.
///
/// ```
/// use dary_heap::TernaryHeap;
/// use std::cmp::Reverse;
///
/// let mut heap = TernaryHeap::new();
///
/// // Wrap values in `Reverse`
/// heap.push(Reverse(1));
/// heap.push(Reverse(5));
/// heap.push(Reverse(2));
///
/// // If we pop these scores now, they should come back in the reverse order.
/// assert_eq!(heap.pop(), Some(Reverse(1)));
/// assert_eq!(heap.pop(), Some(Reverse(2)));
/// assert_eq!(heap.pop(), Some(Reverse(5)));
/// assert_eq!(heap.pop(), None);
/// ```
///
/// # Time complexity
///
/// | [push] | [pop]     | [peek]/[peek\_mut] |
/// |--------|-----------|--------------------|
/// | O(1)~  | *O*(log(*n*)) | *O*(1)               |
///
/// The value for `push` is an expected cost; the method documentation gives a
/// more detailed analysis.
///
/// [push]: #method.push
/// [pop]: #method.pop
/// [peek]: #method.peek
/// [peek\_mut]: #method.peek_mut
pub struct DaryHeap<T, D: Arity> {
    data: Vec<T>,
    marker: PhantomData<D>,
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::{Arity, DaryHeap, Vec};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T: Serialize, D: Arity> Serialize for DaryHeap<T, D> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.data.serialize(serializer)
        }
    }

    impl<'de, T: Ord + Deserialize<'de>, A: Arity> Deserialize<'de> for DaryHeap<T, A> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Vec::deserialize(deserializer).map(Into::into)
        }

        fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
        where
            D: Deserializer<'de>,
        {
            place.data.clear();
            let result = Vec::deserialize_in_place(deserializer, &mut place.data);
            place.rebuild();
            result
        }
    }
}

/// Structure wrapping a mutable reference to the greatest item on a
/// `DaryHeap`.
///
/// This `struct` is created by the [`peek_mut`] method on [`DaryHeap`]. See
/// its documentation for more.
///
/// [`peek_mut`]: struct.DaryHeap.html#method.peek_mut
/// [`DaryHeap`]: struct.DaryHeap.html
pub struct PeekMut<'a, T: 'a + Ord, D: Arity> {
    heap: &'a mut DaryHeap<T, D>,
    sift: bool,
}

impl<T: Ord + fmt::Debug, D: Arity> fmt::Debug for PeekMut<'_, T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PeekMut").field(&self.heap.data[0]).finish()
    }
}

impl<T: Ord, D: Arity> Drop for PeekMut<'_, T, D> {
    fn drop(&mut self) {
        if self.sift {
            self.heap.sift_down(0);
        }
    }
}

impl<T: Ord, D: Arity> Deref for PeekMut<'_, T, D> {
    type Target = T;
    fn deref(&self) -> &T {
        debug_assert!(!self.heap.is_empty());
        // SAFE: PeekMut is only instantiated for non-empty heaps
        unsafe { self.heap.data.get_unchecked(0) }
    }
}

impl<T: Ord, D: Arity> DerefMut for PeekMut<'_, T, D> {
    fn deref_mut(&mut self) -> &mut T {
        debug_assert!(!self.heap.is_empty());
        // SAFE: PeekMut is only instantiated for non-empty heaps
        unsafe { self.heap.data.get_unchecked_mut(0) }
    }
}

impl<'a, T: Ord, D: Arity> PeekMut<'a, T, D> {
    /// Removes the peeked value from the heap and returns it.
    pub fn pop(mut this: PeekMut<'a, T, D>) -> T {
        let value = this.heap.pop().unwrap();
        this.sift = false;
        value
    }
}

impl<T: Clone, D: Arity> Clone for DaryHeap<T, D> {
    fn clone(&self) -> Self {
        DaryHeap {
            data: self.data.clone(),
            marker: PhantomData,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
    }
}

impl<T: Ord, D: Arity> Default for DaryHeap<T, D> {
    /// Creates an empty `DaryHeap<T, D>`.
    #[inline]
    fn default() -> DaryHeap<T, D> {
        DaryHeap::new()
    }
}

impl<T: fmt::Debug, D: Arity> fmt::Debug for DaryHeap<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Ord, D: Arity> DaryHeap<T, D> {
    /// Creates an empty `DaryHeap` as a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let mut heap = QuaternaryHeap::new();
    /// heap.push(4);
    /// ```
    pub fn new() -> DaryHeap<T, D> {
        DaryHeap {
            data: vec![],
            marker: PhantomData,
        }
    }

    /// Creates an empty `DaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `DaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let mut heap = QuaternaryHeap::with_capacity(10);
    /// heap.push(4);
    /// ```
    pub fn with_capacity(capacity: usize) -> DaryHeap<T, D> {
        DaryHeap {
            data: Vec::with_capacity(capacity),
            marker: PhantomData,
        }
    }

    /// Returns a mutable reference to the greatest item in the *d*-ary heap, or
    /// `None` if it is empty.
    ///
    /// Note: If the `PeekMut` value is leaked, the heap may be in an
    /// inconsistent state.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::TernaryHeap;
    /// let mut heap = TernaryHeap::new();
    /// assert!(heap.peek_mut().is_none());
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// {
    ///     let mut val = heap.peek_mut().unwrap();
    ///     *val = 0;
    /// }
    /// assert_eq!(heap.peek(), Some(&2));
    /// ```
    ///
    /// # Time complexity
    ///
    /// Cost is *O*(1) in the worst case.
    pub fn peek_mut(&mut self) -> Option<PeekMut<'_, T, D>> {
        if self.is_empty() {
            None
        } else {
            Some(PeekMut {
                heap: self,
                sift: true,
            })
        }
    }

    /// Removes the greatest item from the *d*-ary heap and returns it, or `None` if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::from(vec![1, 3]);
    ///
    /// assert_eq!(heap.pop(), Some(3));
    /// assert_eq!(heap.pop(), Some(1));
    /// assert_eq!(heap.pop(), None);
    /// ```
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                swap(&mut item, &mut self.data[0]);
                self.sift_down_to_bottom(0);
            }
            item
        })
    }

    /// Pushes an item onto the *d*-ary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let mut heap = QuaternaryHeap::new();
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert_eq!(heap.len(), 3);
    /// assert_eq!(heap.peek(), Some(&5));
    /// ```
    ///
    /// # Time complexity
    ///
    /// The expected cost of `push`, averaged over every possible ordering of
    /// the elements being pushed, and over a sufficiently large number of
    /// pushes, is *O*(1). This is the most meaningful cost metric when pushing
    /// elements that are *not* already in any sorted pattern.
    ///
    /// The time complexity degrades if elements are pushed in predominantly
    /// ascending order. In the worst case, elements are pushed in ascending
    /// sorted order and the amortized cost per push is *O*(log(*n*)) against a heap
    /// containing *n* elements.
    ///
    /// The worst case cost of a *single* call to `push` is *O*(*n*). The worst case
    /// occurs when capacity is exhausted and needs a resize. The resize cost
    /// has been amortized in the previous figures.
    pub fn push(&mut self, item: T) {
        let old_len = self.len();
        self.data.push(item);
        self.sift_up(0, old_len);
    }

    /// Consumes the `DaryHeap` and returns a vector in sorted
    /// (ascending) order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::OctonaryHeap;
    ///
    /// let mut heap = OctonaryHeap::from(vec![1, 2, 4, 5, 7]);
    /// heap.push(6);
    /// heap.push(3);
    ///
    /// let vec = heap.into_sorted_vec();
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut end = self.len();
        while end > 1 {
            end -= 1;
            self.data.swap(0, end);
            self.sift_down_range(0, end);
        }
        self.into_vec()
    }

    // The implementations of sift_up and sift_down use unsafe blocks in
    // order to move an element out of the vector (leaving behind a
    // hole), shift along the others and move the removed element back into the
    // vector at the final location of the hole.
    // The `Hole` type is used to represent this, and make sure
    // the hole is filled back at the end of its scope, even on panic.
    // Using a hole reduces the constant factor compared to using swaps,
    // which involves twice as many moves.
    fn sift_up(&mut self, start: usize, pos: usize) -> usize {
        assert_ne!(D::D, 0, "Arity should be greater than zero");
        unsafe {
            // Take out the value at `pos` and create a hole.
            let mut hole = Hole::new(&mut self.data, pos);

            while hole.pos() > start {
                let parent = (hole.pos() - 1) / D::D;
                if hole.element() <= hole.get(parent) {
                    break;
                }
                hole.move_to(parent);
            }
            hole.pos()
        }
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    fn sift_down_range(&mut self, pos: usize, end: usize) {
        assert_ne!(D::D, 0, "Arity should be greater than zero");
        unsafe {
            let mut hole = Hole::new(&mut self.data, pos);
            let mut child = D::D * pos + 1;
            while child < end {
                // compare with the greatest of the d children
                for other_child in child + 1..child + D::D {
                    if other_child < end && hole.get(child) <= hole.get(other_child) {
                        child = other_child;
                    }
                }
                // if we are already in order, stop.
                if hole.element() >= hole.get(child) {
                    break;
                }
                hole.move_to(child);
                child = D::D * hole.pos() + 1;
            }
        }
    }

    fn sift_down(&mut self, pos: usize) {
        let len = self.len();
        self.sift_down_range(pos, len);
    }

    /// Take an element at `pos` and move it all the way down the heap,
    /// then sift it up to its position.
    ///
    /// Note: This is faster when the element is known to be large / should
    /// be closer to the bottom.
    fn sift_down_to_bottom(&mut self, mut pos: usize) {
        assert_ne!(D::D, 0, "Arity should be greater than zero");
        let end = self.len();
        let start = pos;
        unsafe {
            let mut hole = Hole::new(&mut self.data, pos);
            let mut child = D::D * pos + 1;
            while child < end {
                // compare with the greatest of the d children
                for other_child in child + 1..child + D::D {
                    if other_child < end && hole.get(child) <= hole.get(other_child) {
                        child = other_child;
                    }
                }
                hole.move_to(child);
                child = D::D * hole.pos() + 1;
            }
            pos = hole.pos;
        }
        self.sift_up(start, pos);
    }

    fn rebuild(&mut self) {
        assert_ne!(D::D, 0, "Arity should be greater than zero");
        if self.len() < 2 {
            return;
        }
        let mut n = (self.len() - 1) / D::D + 1;
        while n > 0 {
            n -= 1;
            self.sift_down(n);
        }
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::OctonaryHeap;
    ///
    /// let v = vec![-10, 1, 2, 3, 3];
    /// let mut a = OctonaryHeap::from(v);
    ///
    /// let v = vec![-20, 5, 43];
    /// let mut b = OctonaryHeap::from(v);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    /// assert!(b.is_empty());
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        if self.len() < other.len() {
            swap(self, other);
        }

        if other.is_empty() {
            return;
        }

        #[inline(always)]
        fn log2_fast(x: usize) -> usize {
            8 * size_of::<usize>() - (x.leading_zeros() as usize) - 1
        }

        // `rebuild` takes O(len1 + len2) operations
        // and about n * (len1 + len2) comparisons in the worst case
        // with n = d / (d - 1)
        // while `extend` takes O(len2 * log(len1)) operations
        // and about 1 * len2 * log_d(len1) comparisons in the worst case,
        // assuming len1 >= len2.
        #[inline]
        fn better_to_rebuild<D: Arity>(len1: usize, len2: usize) -> bool {
            assert_ne!(D::D, 0, "Arity should be greater than zero");
            let logd_len1 = log2_fast(len1) / if D::D > 1 { log2_fast(D::D) } else { 1 };
            D::D * (len1 + len2) < (D::D - 1) * len2 * logd_len1
        }

        if better_to_rebuild::<D>(self.len(), other.len()) {
            self.data.append(&mut other.data);
            self.rebuild();
        } else {
            self.extend(other.drain());
        }
    }

    /// Returns an iterator which retrieves elements in heap order.
    /// The retrieved elements are removed from the original heap.
    /// The remaining elements will be removed on drop in heap order.
    ///
    /// Note:
    /// * `.drain_sorted()` is *O*(*n* \* log(*n*)); much slower than `.drain()`.
    ///   You should use the latter for most cases.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::TernaryHeap;
    ///
    /// let mut heap = TernaryHeap::from(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(heap.len(), 5);
    ///
    /// drop(heap.drain_sorted()); // removes all elements in heap order
    /// assert_eq!(heap.len(), 0);
    /// ```
    #[inline]
    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    pub fn drain_sorted(&mut self) -> DrainSorted<'_, T, D> {
        DrainSorted { inner: self }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` such that `f(&e)` returns
    /// `false`. The elements are visited in unsorted (and unspecified) order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::OctonaryHeap;
    ///
    /// let mut heap = OctonaryHeap::from(vec![-10, -5, 1, 2, 4, 13]);
    ///
    /// heap.retain(|x| x % 2 == 0); // only keep even numbers
    ///
    /// assert_eq!(heap.into_sorted_vec(), [-10, 2, 4])
    /// ```
    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.data.retain(f);
        self.rebuild();
    }
}

impl<T, D: Arity> DaryHeap<T, D> {
    /// Returns an iterator visiting all values in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::TernaryHeap;
    /// let heap = TernaryHeap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.iter() {
    ///     println!("{}", x);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.data.iter(),
        }
    }

    /// Returns an iterator which retrieves elements in heap order.
    /// This method consumes the original heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let heap = QuaternaryHeap::from(vec![1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(heap.into_iter_sorted().take(2).collect::<Vec<_>>(), vec![5, 4]);
    /// ```
    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    pub fn into_iter_sorted(self) -> IntoIterSorted<T, D> {
        IntoIterSorted { inner: self }
    }

    /// Returns the greatest item in the *d*-ary heap, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// assert_eq!(heap.peek(), None);
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// assert_eq!(heap.peek(), Some(&5));
    ///
    /// ```
    ///
    /// # Time complexity
    ///
    /// Cost is *O*(1) in the worst case.
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Returns the number of elements the *d*-ary heap can hold without reallocating.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::OctonaryHeap;
    /// let mut heap = OctonaryHeap::with_capacity(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Reserves the minimum capacity for exactly `additional` more elements to be inserted in the
    /// given `DaryHeap`. Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore
    /// capacity can not be relied upon to be precisely minimal. Prefer [`reserve`] if future
    /// insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::OctonaryHeap;
    /// let mut heap = OctonaryHeap::new();
    /// heap.reserve_exact(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    ///
    /// [`reserve`]: #method.reserve
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the
    /// `DaryHeap`. The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// heap.reserve(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Discards as much additional capacity as possible.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::TernaryHeap;
    /// let mut heap: TernaryHeap<i32> = TernaryHeap::with_capacity(100);
    ///
    /// assert!(heap.capacity() >= 100);
    /// heap.shrink_to_fit();
    /// assert!(heap.capacity() == 0);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Discards capacity with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied value.
    ///
    /// Panics if the current capacity is smaller than the supplied
    /// minimum capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(shrink_to)]
    /// use dary_heap::TernaryHeap;
    /// let mut heap: TernaryHeap<i32> = TernaryHeap::with_capacity(100);
    ///
    /// assert!(heap.capacity() >= 100);
    /// heap.shrink_to(10);
    /// assert!(heap.capacity() >= 10);
    /// ```
    #[inline]
    #[cfg(feature = "unstable_nightly")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable_nightly")))]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity)
    }

    /// Consumes the `DaryHeap` and returns the underlying vector
    /// in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let heap = QuaternaryHeap::from(vec![1, 2, 3, 4, 5, 6, 7]);
    /// let vec = heap.into_vec();
    ///
    /// // Will print in some order
    /// for x in vec {
    ///     println!("{}", x);
    /// }
    /// ```
    pub fn into_vec(self) -> Vec<T> {
        self.into()
    }

    /// Returns the length of the *d*-ary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let heap = BinaryHeap::from(vec![1, 3]);
    ///
    /// assert_eq!(heap.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the *d*-ary heap is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    ///
    /// assert!(heap.is_empty());
    ///
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert!(!heap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the *d*-ary heap, returning an iterator over the removed elements.
    ///
    /// The elements are removed in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let mut heap = QuaternaryHeap::from(vec![1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// for x in heap.drain() {
    ///     println!("{}", x);
    /// }
    ///
    /// assert!(heap.is_empty());
    /// ```
    #[inline]
    pub fn drain(&mut self) -> Drain<'_, T> {
        Drain {
            iter: self.data.drain(..),
        }
    }

    /// Drops all items from the *d*-ary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::TernaryHeap;
    /// let mut heap = TernaryHeap::from(vec![1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// heap.clear();
    ///
    /// assert!(heap.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.drain();
    }
}

/// Hole represents a hole in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<'a, T: 'a> {
    data: &'a mut [T],
    elt: ManuallyDrop<T>,
    pos: usize,
}

impl<'a, T> Hole<'a, T> {
    /// Create a new `Hole` at index `pos`.
    ///
    /// Unsafe because pos must be within the data slice.
    #[inline]
    unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
        debug_assert!(pos < data.len());
        // SAFE: pos should be inside the slice
        let elt = ptr::read(data.get_unchecked(pos));
        Hole {
            data,
            elt: ManuallyDrop::new(elt),
            pos,
        }
    }

    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    /// Returns a reference to the element removed.
    #[inline]
    fn element(&self) -> &T {
        &self.elt
    }

    /// Returns a reference to the element at `index`.
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn get(&self, index: usize) -> &T {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        self.data.get_unchecked(index)
    }

    /// Move hole to new location
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn move_to(&mut self, index: usize) {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        let index_ptr: *const _ = self.data.get_unchecked(index);
        let hole_ptr = self.data.get_unchecked_mut(self.pos);
        ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
        self.pos = index;
    }
}

impl<T> Drop for Hole<'_, T> {
    #[inline]
    fn drop(&mut self) {
        // fill the hole again
        unsafe {
            let pos = self.pos;
            ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
        }
    }
}

/// An iterator over the elements of a `DaryHeap`.
///
/// This `struct` is created by the [`iter`] method on [`DaryHeap`]. See its
/// documentation for more.
///
/// [`iter`]: struct.DaryHeap.html#method.iter
/// [`DaryHeap`]: struct.DaryHeap.html
pub struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>,
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.iter.as_slice()).finish()
    }
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<&'a T> {
        self.iter.last()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back()
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    #[cfg(feature = "unstable_nightly")]
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

impl<T> FusedIterator for Iter<'_, T> {}

/// An owning iterator over the elements of a `DaryHeap`.
///
/// This `struct` is created by the [`into_iter`] method on [`DaryHeap`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`into_iter`]: struct.DaryHeap.html#method.into_iter
/// [`DaryHeap`]: struct.DaryHeap.html
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: vec::IntoIter<T>,
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter")
            .field(&self.iter.as_slice())
            .finish()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    #[cfg(feature = "unstable_nightly")]
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

impl<T> FusedIterator for IntoIter<T> {}

#[cfg(feature = "unstable")]
#[derive(Clone, Debug)]
pub struct IntoIterSorted<T, D: Arity> {
    inner: DaryHeap<T, D>,
}

#[cfg(feature = "unstable")]
impl<T: Ord, D: Arity> Iterator for IntoIterSorted<T, D> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

#[cfg(feature = "unstable")]
impl<T: Ord, D: Arity> ExactSizeIterator for IntoIterSorted<T, D> {}

#[cfg(feature = "unstable")]
impl<T: Ord, D: Arity> FusedIterator for IntoIterSorted<T, D> {}

#[cfg(all(feature = "unstable", feature = "unstable_nightly"))]
unsafe impl<T: Ord, D: Arity> core::iter::TrustedLen for IntoIterSorted<T, D> {}

/// A draining iterator over the elements of a `DaryHeap`.
///
/// This `struct` is created by the [`drain`] method on [`DaryHeap`]. See its
/// documentation for more.
///
/// [`drain`]: struct.DaryHeap.html#method.drain
/// [`DaryHeap`]: struct.DaryHeap.html
#[derive(Debug)]
pub struct Drain<'a, T: 'a> {
    iter: vec::Drain<'a, T>,
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for Drain<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> ExactSizeIterator for Drain<'_, T> {
    #[cfg(feature = "unstable_nightly")]
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
}

impl<T> FusedIterator for Drain<'_, T> {}

/// A draining iterator over the elements of a `DaryHeap`.
///
/// This `struct` is created by the [`drain_sorted`] method on [`DaryHeap`]. See its
/// documentation for more.
///
/// [`drain_sorted`]: struct.DaryHeap.html#method.drain_sorted
/// [`DaryHeap`]: struct.DaryHeap.html
#[cfg(feature = "unstable")]
#[derive(Debug)]
pub struct DrainSorted<'a, T: Ord, D: Arity> {
    inner: &'a mut DaryHeap<T, D>,
}

#[cfg(feature = "unstable")]
impl<'a, T: Ord, D: Arity> Drop for DrainSorted<'a, T, D> {
    /// Removes heap elements in heap order.
    fn drop(&mut self) {
        use core::mem::forget;

        struct DropGuard<'r, 'a, T: Ord, D: Arity>(&'r mut DrainSorted<'a, T, D>);

        impl<'r, 'a, T: Ord, D: Arity> Drop for DropGuard<'r, 'a, T, D> {
            fn drop(&mut self) {
                while self.0.inner.pop().is_some() {}
            }
        }

        while let Some(item) = self.inner.pop() {
            let guard = DropGuard(self);
            drop(item);
            forget(guard);
        }
    }
}

#[cfg(feature = "unstable")]
impl<T: Ord, D: Arity> Iterator for DrainSorted<'_, T, D> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

#[cfg(feature = "unstable")]
impl<T: Ord, D: Arity> ExactSizeIterator for DrainSorted<'_, T, D> {}

#[cfg(feature = "unstable")]
impl<T: Ord, D: Arity> FusedIterator for DrainSorted<'_, T, D> {}

#[cfg(all(feature = "unstable", feature = "unstable_nightly"))]
unsafe impl<T: Ord, D: Arity> core::iter::TrustedLen for DrainSorted<'_, T, D> {}

impl<T: Ord, D: Arity> From<Vec<T>> for DaryHeap<T, D> {
    /// Converts a `Vec<T>` into a `DaryHeap<T, D>`.
    ///
    /// This conversion happens in-place, and has *O*(*n*) time complexity.
    fn from(vec: Vec<T>) -> DaryHeap<T, D> {
        let mut heap = DaryHeap {
            data: vec,
            marker: PhantomData,
        };
        heap.rebuild();
        heap
    }
}

#[cfg(rustc_1_41)]
/// # Compatibility
/// This trait is only implemented on Rust version 1.41.0 or greater. On earlier
/// versions `Into<Vec<T>>` is implemented for `DaryHeap<T, D>` instead.
impl<T, D: Arity> From<DaryHeap<T, D>> for Vec<T> {
    fn from(heap: DaryHeap<T, D>) -> Vec<T> {
        heap.data
    }
}

#[cfg(not(rustc_1_41))]
impl<T, D: Arity> Into<Vec<T>> for DaryHeap<T, D> {
    fn into(self) -> Vec<T> {
        self.data
    }
}

impl<T: Ord, D: Arity> FromIterator<T> for DaryHeap<T, D> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> DaryHeap<T, D> {
        DaryHeap::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<T, D: Arity> IntoIterator for DaryHeap<T, D> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the *d*-ary heap in arbitrary order. The *d*-ary heap cannot be used
    /// after calling this.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let heap = BinaryHeap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.into_iter() {
    ///     // x has type i32, not &i32
    ///     println!("{}", x);
    /// }
    /// ```
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            iter: self.data.into_iter(),
        }
    }
}

impl<'a, T, D: Arity> IntoIterator for &'a DaryHeap<T, D> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T: Ord, D: Arity> Extend<T> for DaryHeap<T, D> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.extend_desugared(iter.into_iter());
    }

    #[inline]
    #[cfg(feature = "unstable_nightly")]
    fn extend_one(&mut self, item: T) {
        self.push(item);
    }

    #[inline]
    #[cfg(feature = "unstable_nightly")]
    fn extend_reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }
}

impl<T: Ord, D: Arity> DaryHeap<T, D> {
    fn extend_desugared<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iterator = iter.into_iter();
        let (lower, _) = iterator.size_hint();

        self.reserve(lower);

        iterator.for_each(move |elem| self.push(elem));
    }
}

impl<'a, T: 'a + Ord + Copy, D: Arity> Extend<&'a T> for DaryHeap<T, D> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }

    #[inline]
    #[cfg(feature = "unstable_nightly")]
    fn extend_one(&mut self, &item: &'a T) {
        self.push(item);
    }

    #[inline]
    #[cfg(feature = "unstable_nightly")]
    fn extend_reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }
}

#[cfg(any(test, fuzzing))]
impl<T: Ord + fmt::Debug, D: Arity> DaryHeap<T, D> {
    /// Panics if the heap is in an inconsistent state
    #[track_caller]
    pub fn assert_valid_state(&self) {
        assert_ne!(D::D, 0, "Arity should be greater than zero");
        for (i, v) in self.iter().enumerate() {
            let children = D::D * i + 1..D::D * i + D::D;
            if children.start > self.len() {
                break;
            }
            for j in children {
                if let Some(x) = self.data.get(j) {
                    assert!(v >= x);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{seq::SliceRandom, thread_rng};

    fn pop<D: Arity>() {
        let mut rng = thread_rng();
        let ntest = 10;
        let nelem = 1000;
        for _ in 0..ntest {
            let mut data: Vec<_> = (0..nelem).collect();
            data.shuffle(&mut rng);
            let mut heap = DaryHeap::<_, D>::from(data);
            heap.assert_valid_state();
            for i in (0..nelem).rev() {
                assert_eq!(heap.pop(), Some(i));
                heap.assert_valid_state();
            }
            assert_eq!(heap.pop(), None);
        }
    }

    enum D0 {}

    impl Arity for D0 {
        const D: usize = 0;
    }

    #[test]
    #[should_panic]
    fn push_d0() {
        let mut heap = DaryHeap::<_, D0>::new();
        heap.push(42);
    }

    #[test]
    #[should_panic]
    fn from_vec_d0() {
        let _heap = DaryHeap::<_, D0>::from(vec![42]);
    }

    #[test]
    fn pop_d1() {
        arity! { D1 = 1; }
        pop::<D1>();
    }

    #[test]
    fn pop_d2() {
        pop::<D2>();
    }

    #[test]
    fn pop_d3() {
        pop::<D3>();
    }

    #[test]
    fn pop_d4() {
        pop::<D4>();
    }

    #[test]
    fn pop_d5() {
        pop::<D5>();
    }

    #[test]
    fn pop_d6() {
        pop::<D6>();
    }

    #[test]
    fn pop_d7() {
        pop::<D7>();
    }

    #[test]
    fn pop_d8() {
        pop::<D8>();
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        use serde_test::Token::{Seq, SeqEnd, I32};

        impl<T: PartialEq, D: Arity> PartialEq for DaryHeap<T, D> {
            fn eq(&self, other: &Self) -> bool {
                self.iter().zip(other).all(|(a, b)| a == b)
            }
        }

        let empty = [Seq { len: Some(0) }, SeqEnd];
        let part = [Seq { len: Some(3) }, I32(3), I32(1), I32(2), SeqEnd];
        let full = [Seq { len: Some(4) }, I32(4), I32(3), I32(2), I32(1), SeqEnd];

        let mut dary = BinaryHeap::<i32>::new();
        serde_test::assert_tokens(&dary, &empty);
        dary.extend(&[1, 2, 3]);
        serde_test::assert_tokens(&dary, &part);
        dary.push(4);
        serde_test::assert_tokens(&dary, &full);

        let mut std = alloc::collections::BinaryHeap::<i32>::new();
        serde_test::assert_ser_tokens(&std, &empty);
        std.extend(&[1, 2, 3]);
        serde_test::assert_ser_tokens(&std, &part);
        std.push(4);
        serde_test::assert_ser_tokens(&std, &full);
    }
}
