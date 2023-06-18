use criterion::*;
use dary_heap::DaryHeap;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::BinaryHeap;
use std::convert::identity;

/// Data type we want to use
type T = u32;

/// Produce shuffled vector containing values 0..n
fn random_data(n: T) -> Vec<T> {
    let mut data: Vec<_> = (0..n).collect();
    data.shuffle(&mut thread_rng());
    data
}

fn make_std_heap(data: Vec<T>) -> BinaryHeap<T> {
    BinaryHeap::from(data)
}

fn make_dary_heap<const D: usize>(data: Vec<T>) -> DaryHeap<T, D> {
    DaryHeap::<T, D>::from(data)
}

fn std_heap_pop(mut heap: BinaryHeap<T>) -> BinaryHeap<T> {
    heap.pop();
    heap
}

fn dary_heap_pop<const D: usize>(mut heap: DaryHeap<T, D>) -> DaryHeap<T, D> {
    heap.pop();
    heap
}

fn std_heap_push((mut heap, elem): (BinaryHeap<T>, T)) -> BinaryHeap<T> {
    heap.push(elem);
    heap
}

fn dary_heap_push<const D: usize>((mut heap, elem): (DaryHeap<T, D>, T)) -> DaryHeap<T, D> {
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
            for &i in &[10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000] {
                let size = BatchSize::SmallInput;
                group.bench_function(BenchmarkId::new("BinaryHeap", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $std_fn, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<2>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<2>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<3>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<3>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<4>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<4>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<5>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<5>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<6>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<6>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<7>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<7>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<8>", i), |b| {
                    b.iter_batched(|| $data(random_data(i)), $dary_fn::<8>, size)
                });
            }
        }
    };
}

heap_bench!(make_heap, make_std_heap, make_dary_heap);
heap_bench!(pop, std_heap_pop, dary_heap_pop, Vec::into);
heap_bench!(push, std_heap_push, dary_heap_push, push_data);

fn two_heaps<H: From<Vec<T>>>(mut vec1: Vec<T>, i: usize) -> (H, H) {
    let vec2 = vec1.split_off(i);
    (vec1.into(), vec2.into())
}

fn std_heap_append((mut heap1, mut heap2): (BinaryHeap<T>, BinaryHeap<T>)) -> BinaryHeap<T> {
    heap1.append(&mut heap2);
    heap1
}

fn dary_heap_append<const D: usize>(
    (mut heap1, mut heap2): (DaryHeap<T, D>, DaryHeap<T, D>),
) -> DaryHeap<T, D> {
    heap1.append(&mut heap2);
    heap1
}

fn std_heap_extend((mut heap1, mut heap2): (BinaryHeap<T>, BinaryHeap<T>)) -> BinaryHeap<T> {
    if heap1.len() < heap2.len() {
        core::mem::swap(&mut heap1, &mut heap2);
    }
    heap1.extend(heap2.drain());
    heap1
}

fn dary_heap_extend<const D: usize>(
    (mut heap1, mut heap2): (DaryHeap<T, D>, DaryHeap<T, D>),
) -> DaryHeap<T, D> {
    if heap1.len() < heap2.len() {
        core::mem::swap(&mut heap1, &mut heap2);
    }
    heap1.extend(heap2.drain());
    heap1
}

macro_rules! heap_bench_merge {
    ($name:ident, $std_fn:ident, $dary_fn:ident $(,)?) => {
        heap_bench_merge!($name, $std_fn, $dary_fn, two_heaps);
    };
    ($name:ident, $std_fn:ident, $dary_fn:ident, $data:expr $(,)?) => {
        fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($name));
            let base = 100000;
            let step = 2500;
            for i in (step..=base as usize / 2).step_by(step) {
                let size = BatchSize::SmallInput;
                group.bench_function(BenchmarkId::new("BinaryHeap", i), |b| {
                    b.iter_batched(|| $data(random_data(base), i), $std_fn, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<2>", i), |b| {
                    b.iter_batched(|| $data(random_data(base), i), $dary_fn::<2>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<3>", i), |b| {
                    b.iter_batched(|| $data(random_data(base), i), $dary_fn::<3>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<4>", i), |b| {
                    b.iter_batched(|| $data(random_data(base), i), $dary_fn::<4>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<5>", i), |b| {
                    b.iter_batched(|| $data(random_data(base), i), $dary_fn::<5>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<6>", i), |b| {
                    b.iter_batched(|| $data(random_data(base), i), $dary_fn::<6>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<7>", i), |b| {
                    b.iter_batched(|| $data(random_data(base), i), $dary_fn::<7>, size)
                });
                group.bench_function(BenchmarkId::new("DaryHeap<8>", i), |b| {
                    b.iter_batched(|| $data(random_data(base), i), $dary_fn::<8>, size)
                });
            }
        }
    };
}

heap_bench_merge!(append, std_heap_append, dary_heap_append);
heap_bench_merge!(extend, std_heap_extend, dary_heap_extend);

criterion_group!(benches, make_heap, pop, push, append, extend);
criterion_main!(benches);
