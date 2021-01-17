#![feature(test)]

extern crate test;

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
