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

fn compile_ts103301() -> Result<()> {
    Compiler::<RasnBackend, _>::new_with_config(RasnConfig {
        opaque_open_types: false,
        default_wildcard_imports: false,
        no_std_compliant_bindings: true,
        ..Default::default()
    })
    .add_asn_sources_by_path(
        [
            PathBuf::from("src/ts103301/asn1/DSRC.asn"),
            PathBuf::from("src/ts103301/asn1/DSRC-addgrp-C.asn"),
            PathBuf::from("src/ts103301/asn1/DSRC-region.asn"),
            PathBuf::from("src/ts103301/asn1/MAPEM-PDU-Descriptions.asn"),
            PathBuf::from("src/ts103301/asn1/RTCMEM-PDU-Descriptions.asn"),
            PathBuf::from("src/ts103301/asn1/SPATEM-PDU-Descriptions.asn"),
            PathBuf::from("src/ts103301/asn1/SREM-PDU-Descriptions.asn"),
            PathBuf::from("src/ts103301/asn1/SSEM-PDU-Descriptions.asn"),
        ]
        .iter(),
    )
    .set_output_mode(SingleFile(PathBuf::from("./src/ts103301/ts103301.rs")))
    .compile()
    .map_err(|e| anyhow::anyhow!("{e:?}"))
    .context("Failed to compile ts103301 source")?;

    Ok(())
}

fn main() -> Result<()> {
    compile_ts102894_2()?;
    compile_ts103301()?;

    Ok(())
}
