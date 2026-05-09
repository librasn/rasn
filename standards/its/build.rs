use std::path::PathBuf;

use anyhow::{Context, Ok, Result};
use rasn_compiler::{OutputMode::SingleFile, prelude::*};

fn compile_ts102894_2() -> Result<()> {
    Compiler::<RasnBackend, _>::new_with_config(RasnConfig {
        opaque_open_types: false,
        default_wildcard_imports: false,
        no_std_compliant_bindings: true,
        ..Default::default()
    })
    .add_asn_by_path("src/ts102894_2/asn1/ETSI-ITS-CDD.asn")
    .set_output_mode(SingleFile(PathBuf::from("./src/ts102894_2/mod.rs")))
    .compile()
    .map_err(|e| anyhow::anyhow!("{e:?}"))
    .context("Failed to compile CDD ASN source")?;

    Ok(())
}

fn main() -> Result<()> {
    compile_ts102894_2()?;

    Ok(())
}
