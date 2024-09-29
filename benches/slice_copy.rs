use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn copy_and_reverse<const N: usize>(bytes: &[u8; N], needed: usize) -> [u8; N] {
    let mut slice_reversed: [u8; N] = [0; N];
    slice_reversed[..needed].copy_from_slice(&bytes[..needed]);
    slice_reversed[..needed].reverse();
    slice_reversed
}

fn loop_reverse<const N: usize>(bytes: &[u8; N], needed: usize) -> [u8; N] {
    let mut slice_reversed: [u8; N] = [0; N];
    for i in 0..needed {
        slice_reversed[i] = bytes[needed - 1 - i];
    }
    slice_reversed
}

fn criterion_benchmark(c: &mut Criterion) {
    let bytes: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let needed = 4;

    c.bench_function("copy_and_reverse", |b| {
        b.iter(|| copy_and_reverse(black_box(&bytes), black_box(needed)))
    });

    c.bench_function("loop_reverse", |b| {
        b.iter(|| loop_reverse(black_box(&bytes), black_box(needed)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
