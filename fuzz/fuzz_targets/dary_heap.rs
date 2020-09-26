#![no_main]
use dary_heap::{Arity, DaryHeap, D2, D3, D4, D5, D6, D7, D8};
use libfuzzer_sys::fuzz_target;

fn to_u16_slice(data: &[u8]) -> &[u16] {
    unsafe { data.align_to().1 }
}

fn heap<D: Arity>(data: &[u16]) -> DaryHeap<u16, D> {
    DaryHeap::<_, D>::from(Vec::from(data))
}

fn sorted(data: &[u16]) -> Vec<u16> {
    let mut sort_data = Vec::from(data);
    sort_data.sort();
    sort_data
}

fn peek_mut<D: Arity>(data: &[u16]) {
    if let Some((&first, data)) = data.split_first() {
        let mut heap = heap::<D>(data);
        if let Some(mut peek) = heap.peek_mut() {
            *peek = first;
        }
        heap.assert_valid_state();
    }
}

fn pop<D: Arity>(data: &[u16]) {
    let mut heap = heap::<D>(data);
    let sort_data = sorted(data);
    assert_eq!(sort_data.len(), heap.len());
    for &x in sort_data.iter().rev() {
        assert_eq!(heap.pop(), Some(x));
        heap.assert_valid_state();
    }
    assert_eq!(heap.pop(), None);
}

fn push<D: Arity>(data: &[u16]) {
    let mut heap = DaryHeap::<_, D>::with_capacity(data.len());
    for &x in data {
        heap.push(x);
        heap.assert_valid_state();
    }
}

fn into_sorted_vec<D: Arity>(data: &[u16]) {
    let heap = heap::<D>(data);
    let sort_data = sorted(data);
    let sorted = heap.into_sorted_vec();
    assert_eq!(sorted, sort_data);
}

fn append<D: Arity>(data: &[u16]) {
    if let Some((&first, data)) = data.split_first() {
        let first = first as usize;
        if first > data.len() {
            return;
        }
        let (data1, data2) = data.split_at(first);
        let mut heap1 = heap::<D>(data1);
        let mut heap2 = heap(data2);
        heap1.append(&mut heap2);
        assert!(heap2.is_empty());
        assert!(heap1.len() == data.len());
        heap1.assert_valid_state();
    }
}

fn make_heap<D: Arity>(data: &[u16]) {
    let heap = heap::<D>(data);
    heap.assert_valid_state();
}

macro_rules! fuzz_match {
    ($first:expr, $start:expr, $arity:ident, $data:expr) => {
        match $first.wrapping_sub($start) {
            0 => peek_mut::<$arity>($data),
            1 => pop::<$arity>($data),
            2 => push::<$arity>($data),
            3 => into_sorted_vec::<$arity>($data),
            4 => append::<$arity>($data),
            5 => make_heap::<$arity>($data),
            _ => {}
        }
    };
}

fuzz_target!(|data: &[u8]| {
    if let Some((first, data)) = data.split_first() {
        let data = to_u16_slice(data);
        fuzz_match!(first, 0, D2, data);
        fuzz_match!(first, 32, D3, data);
        fuzz_match!(first, 64, D4, data);
        fuzz_match!(first, 96, D5, data);
        fuzz_match!(first, 128, D6, data);
        fuzz_match!(first, 160, D7, data);
        fuzz_match!(first, 192, D8, data);
    }
});
