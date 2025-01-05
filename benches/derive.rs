use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use quote::ToTokens;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use syn::DeriveInput;

fn recurse_dir(path: &PathBuf, prefix: &PathBuf, files: &mut Vec<(String, Vec<DeriveInput>)>) {
    for file in
        fs::read_dir(path).unwrap_or_else(|_| panic!("Unable to recurse into dir {:?}", path))
    {
        let Ok(file) = file else {
            panic!("Unable to read dir entry inside {:?}", path);
        };
        if file
            .metadata()
            .unwrap_or_else(|_| panic!("Unable to read file {:?} metadata", file.path()))
            .is_dir()
        {
            recurse_dir(&file.path(), prefix, files);
        } else if Some(OsStr::new("rs")) == file.path().extension() {
            let name = file
                .path()
                .to_string_lossy()
                .into_owned()
                .trim_start_matches(&*prefix.to_string_lossy())
                .to_string();
            let content = fs::read_to_string(file.path())
                .unwrap_or_else(|_| panic!("Unable to read file {:?}", file.path()));
            let parsed = syn::parse_file(&content)
                .unwrap_or_else(|_| panic!("Unable to parse file {:?}", file.path()));
            let interesting_items: Vec<_> = parsed
                .items
                .iter()
                .filter(|item| match item {
                    syn::Item::Enum(e) => check_attrs(&e.attrs),
                    syn::Item::Struct(s) => check_attrs(&s.attrs),
                    _ => false,
                })
                .map(|item| syn::parse2(item.to_token_stream()).unwrap())
                .collect();
            if !interesting_items.is_empty() {
                files.push((name, interesting_items));
            }
        }
    }
}

fn check_attrs(attrs: &[syn::Attribute]) -> bool {
    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }
        let mut attr_found = false;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("AsnType")
                || meta.path.is_ident("Encode")
                || meta.path.is_ident("Decode")
            {
                attr_found = true;
            }
            Ok(())
        })
        .unwrap();
        if attr_found {
            return true;
        }
    }
    false
}

macro_rules! derive_bench {
    ($bench_name:ident, $name:literal, $fn:ident ) => {
        fn $bench_name(c: &mut Criterion) {
            let crate_root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let mut files = Vec::new();
            recurse_dir(&crate_root_dir, &crate_root_dir, &mut files);

            let mut group = c.benchmark_group($name);

            for (filename, items) in files {
                group.bench_function(filename, |b| {
                    b.iter_batched(
                        || items.clone(),
                        |data| {
                            for x in data {
                                let _ = black_box(rasn_derive_impl::$fn(x));
                            }
                        },
                        BatchSize::SmallInput,
                    )
                });
            }

            group.finish();
        }
    };
}

derive_bench!(derive_asntype, "derive_asntype", asn_type_derive_inner);
derive_bench!(derive_decode, "derive_decode", decode_derive_inner);
derive_bench!(derive_encode, "derive_encode", encode_derive_inner);

criterion_group!(
    name = derive;
    config = Criterion::default().sample_size(200);
    targets = derive_encode, derive_decode, derive_asntype
);
criterion_main!(derive);
