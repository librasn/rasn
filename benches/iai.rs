mod common;

use common::*;

macro_rules! encoding_rules {
    ($($encoding: ident ($encoding_fn:ident, $decoding_fn:ident)),+ $(,)?) => {
        $(
            fn $encoding_fn() {
                let data = iai::black_box(bench_default());

                rasn::$encoding::encode(&data).unwrap();
            }

            fn $decoding_fn() {
                use once_cell::sync::Lazy;

                static DATA: Lazy<Vec<u8>> = Lazy::new(|| {
                    rasn::$encoding::encode(&bench_default()).unwrap()
                });

                rasn::$encoding::decode::<Bench>(&DATA).unwrap();
            }

        )+

        iai::main!{$($encoding_fn, $decoding_fn),+}
    }
}

encoding_rules! {
    ber(ber_encode, ber_decode),
    cer(cer_encode, cer_decode),
    der(der_encode, der_decode),
}
