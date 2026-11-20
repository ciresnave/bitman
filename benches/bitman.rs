//! Benchmarks for `BitMan`. The array/slice case is the one to watch: indexing a bit walks the
//! elements (O(elements) per bit), so cost should grow with the array length.
//! Profile with `cargo flamegraph --bench bitman -- --bench` (see CONTRIBUTING.md).

use std::hint::black_box;

use bitman::{Bit, BitMan, Bits, Field};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn integer(c: &mut Criterion) {
    let mut value = 0xA5A5_A5A5_A5A5_A5A5u64;
    c.bench_function("u64/bit", |b| {
        b.iter(|| black_box(value).bit(black_box(37)))
    });
    c.bench_function("u64/set_bit", |b| {
        b.iter(|| value.set_bit(black_box(37), Bit(true)))
    });
    c.bench_function("u64/bits", |b| b.iter(|| black_box(value).bits()));
}

fn arrays(c: &mut Criterion) {
    let mut group = c.benchmark_group("array/bit_last");
    for elements in [1usize, 16, 256, 4096] {
        let data = vec![0xA5u8; elements];
        let last = elements * 8 - 1;
        group.bench_with_input(BenchmarkId::from_parameter(elements), &data, |b, d| {
            b.iter(|| d.as_slice().bit(black_box(last)))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("array/set_bit_last");
    for elements in [1usize, 16, 256, 4096] {
        let mut data = vec![0xA5u8; elements];
        let last = elements * 8 - 1;
        group.bench_function(BenchmarkId::from_parameter(elements), |b| {
            b.iter(|| data.as_mut_slice().set_bit(black_box(last), Bit(true)))
        });
    }
    group.finish();
}

fn field(c: &mut Criterion) {
    let value = 0xDEAD_BEEFu32;
    let f = Field::new(8, 12);
    c.bench_function("field/read_u32", |b| {
        b.iter(|| f.read::<u32, _>(black_box(&value)))
    });
    let mut target = 0u32;
    c.bench_function("field/write_u32", |b| {
        b.iter(|| f.write(&mut target, black_box(0x5A5u32)))
    });
}

fn conversions(c: &mut Criterion) {
    let bytes = vec![0xA5u8; 1024];
    c.bench_function("Bits/from_be_bytes_1KiB", |b| {
        b.iter(|| Bits::from_be_bytes(black_box(&bytes)))
    });
}

criterion_group!(benches, integer, arrays, field, conversions);
criterion_main!(benches);
