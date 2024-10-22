mod common;

use common::*;

use iai_callgrind::{main, library_benchmark_group, library_benchmark};
use std::hint::black_box;

macro_rules! encoding_rules {
    ($($encoding: ident ($encoding_fn:ident, $decoding_fn:ident)),+ $(,)?) => {
        $(
            #[library_benchmark]
            fn $encoding_fn() {
                let data = black_box(bench_default());

                rasn::$encoding::encode(&data).unwrap();
            }

            #[library_benchmark]
            fn $decoding_fn() {
                use once_cell::sync::Lazy;

                static DATA: Lazy<Vec<u8>> = Lazy::new(|| {
                    rasn::$encoding::encode(&bench_default()).unwrap()
                });

                rasn::$encoding::decode::<Bench>(&DATA).unwrap();
            }

        )+

        library_benchmark_group!(name = encode; benchmarks = $($encoding_fn),+);
        library_benchmark_group!(name = decode; benchmarks = $($decoding_fn),+);
        main!(library_benchmark_groups = encode, decode);
    }
}

encoding_rules! {
    ber(ber_encode, ber_decode),
    cer(cer_encode, cer_decode),
    der(der_encode, der_decode),
}
