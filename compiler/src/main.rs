use clap::{clap_app, crate_description, crate_version};
use log::{debug, LevelFilter};

use rasn_compiler::NotationCompiler;

fn main() {
    let matches = clap_app!(casn1 =>
        (version: crate_version!())
        (about: crate_description!())
        (@arg dependencies: -d --dependencies
            +takes_value
            "Specify the dependency directory. Will automatically parse the headers of \
            the files, and import them if necessary. Default: \"./definitions\"")
        (@arg input: ... "ASN.1 files to parse.")
        (@arg verbose: -v --verbose ...
            "Set log output level")
    )
    .get_matches();

    let mut builder = ::env_logger::Builder::new();

    let filter_level = match matches.occurrences_of("verbose") {
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Warn,
    };

    builder.filter(None, filter_level);
    builder.init();

    debug!("LOG Level: {:?}", filter_level);
    debug!("CLI Config: {:#?}", matches);

    let directory = matches.value_of("dependencies").unwrap_or("./asn1");

    let module = NotationCompiler::new(matches.value_of("input").unwrap())
        .dependencies(directory)
        .build()
        .unwrap_or_else(|e| panic!("{}", e));

    println!("{}", module);
}
