use criterion::{criterion_group, criterion_main, Criterion};
use std::cell::RefCell;
use std::rc::Rc;

// #[inline(never)]
fn direct_mutation(vec: &mut Vec<u8>) {
    for _ in 0..1000 {
        vec.push(1);
    }
}

// #[inline(never)]
fn rc_refcell_mutation(rc_refcell_vec: &RefCell<Vec<u8>>) {
    for _ in 0..1000 {
        rc_refcell_vec.borrow_mut().push(1);
    }
}

fn benchmark_direct_mutation(c: &mut Criterion) {
    let mut vec = vec![0u8; 1000];
    c.bench_function("direct_mutation", |b| {
        b.iter(|| {
            direct_mutation(&mut vec);
        })
    });
}

fn benchmark_rc_refcell_mutation(c: &mut Criterion) {
    // let rc_refcell_vec = Rc::new(RefCell::new(vec![0u8; 1000]));
    // let rc_refcell_vec = RefCell::new(vec![0u8; 1000]);
    let rc_refcell_vec = RefCell::new(vec![0u8; 1000]);
    c.bench_function("rc_refcell_mutation", |b| {
        b.iter(|| {
            rc_refcell_mutation(&rc_refcell_vec);
        })
    });
}

criterion_group!(
    benches,
    benchmark_direct_mutation,
    benchmark_rc_refcell_mutation
);
criterion_main!(benches);
