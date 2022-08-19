#![feature(test)]

extern crate test;

/// Returns a `rand::Rng` seeded with a consistent seed.
///
/// This is done to avoid introducing nondeterminism in benchmark results.
fn bench_rng() -> rand_xorshift::XorShiftRng {
    const SEED: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    rand::SeedableRng::from_seed(SEED)
}

#[path = "upstream"]
mod std_binary_heap {
    use std::collections::BinaryHeap;
    mod binary_heap;
}

#[path = "upstream"]
mod dary_heap_d2 {
    use dary_heap::BinaryHeap;
    mod binary_heap;
}

#[path = "upstream"]
mod dary_heap_d3 {
    use dary_heap::TernaryHeap as BinaryHeap;
    mod binary_heap;
}

#[path = "upstream"]
mod dary_heap_d4 {
    use dary_heap::QuaternaryHeap as BinaryHeap;
    mod binary_heap;
}

#[path = "upstream"]
mod dary_heap_d5 {
    use dary_heap::QuinaryHeap as BinaryHeap;
    mod binary_heap;
}

#[path = "upstream"]
mod dary_heap_d6 {
    use dary_heap::SenaryHeap as BinaryHeap;
    mod binary_heap;
}

#[path = "upstream"]
mod dary_heap_d7 {
    use dary_heap::SeptenaryHeap as BinaryHeap;
    mod binary_heap;
}

#[path = "upstream"]
mod dary_heap_d8 {
    use dary_heap::OctonaryHeap as BinaryHeap;
    mod binary_heap;
}
