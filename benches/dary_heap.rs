use criterion::*;
use dary_heap::{Dary, DaryHeap, D2, D3, D4, D8};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::BinaryHeap;
use std::convert::identity;
use std::iter::FromIterator;

/// Data type we want to use
type T = u16;

/// Produce shuffled vector containing values 0..n
fn random_data(n: T) -> Vec<T> {
    let mut data = Vec::from_iter(0..n);
    data.shuffle(&mut thread_rng());
    data
}

fn make_std_heap(data: Vec<T>) -> BinaryHeap<T> {
    BinaryHeap::from(data)
}

fn make_dary_heap<D: Dary>(data: Vec<T>) -> DaryHeap<T, D> {
    DaryHeap::<T, D>::from(data)
}

fn std_heap_pop(mut heap: BinaryHeap<T>) -> BinaryHeap<T> {
    heap.pop();
    heap
}

fn dary_heap_pop<D: Dary>(mut heap: DaryHeap<T, D>) -> DaryHeap<T, D> {
    heap.pop();
    heap
}

fn std_heap_push((mut heap, elem): (BinaryHeap<T>, T)) -> BinaryHeap<T> {
    heap.push(elem);
    heap
}

fn dary_heap_push<D: Dary>((mut heap, elem): (DaryHeap<T, D>, T)) -> DaryHeap<T, D> {
    heap.push(elem);
    heap
}

fn push_data<H: From<Vec<T>>>(mut vec: Vec<T>) -> (H, T) {
    let elem = vec.pop().unwrap();
    (vec.into(), elem)
}

macro_rules! heap_bench {
    ($name:ident, $std_fn:ident, $dary_fn:ident $(,)?) => {
        heap_bench!($name, $std_fn, $dary_fn, identity);
    };
    ($name:ident, $std_fn:ident, $dary_fn:ident, $data:expr $(,)?) => {
        fn $name(c: &mut Criterion) {
            let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
            let mut group = c.benchmark_group(stringify!($name));
            group.plot_config(plot_config);
            for &i in &[10, 100, 1000, 10000] {
                let size = BatchSize::SmallInput;
                group.bench_function(BenchmarkId::new("BinaryHeap", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $std_fn, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<D2>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<D2>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<D3>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<D3>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<D4>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<D4>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<D8>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<D8>, size)
                });
            }
        }
    };
}

heap_bench!(make_heap, make_std_heap, make_dary_heap);
heap_bench!(pop, std_heap_pop, dary_heap_pop, Vec::into);
heap_bench!(push, std_heap_push, dary_heap_push, push_data);

criterion_group!(benches, make_heap, pop, push);
criterion_main!(benches);