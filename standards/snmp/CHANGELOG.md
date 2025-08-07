# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.27.0](https://github.com/librasn/rasn/compare/rasn-snmp-v0.26.6...rasn-snmp-v0.27.0) - 2025-06-19

### Other

- reexport rasn default-features in standard crates ([#477](https://github.com/librasn/rasn/pull/477))
- *(octet-string)* [**breaking**] wrap octet string in new type ([#469](https://github.com/librasn/rasn/pull/469))
- *(ci)* fix clippy lints

## [0.21.0](https://github.com/librasn/rasn/compare/rasn-snmp-v0.20.2...rasn-snmp-v0.21.0) - 2024-11-12

### Other

- Optimize field presence tracking of default/optional/extended fields ([#324](https://github.com/librasn/rasn/pull/324))

## [0.20.2](https://github.com/librasn/rasn/compare/rasn-snmp-v0.20.1...rasn-snmp-v0.20.2) - 2024-10-18

### Fixed

- Run clippy and rustdoc only on stable channel in CI ([#342](https://github.com/librasn/rasn/pull/342))

## [0.18.0](https://github.com/librasn/rasn/compare/rasn-snmp-v0.17.3...rasn-snmp-v0.18.0) - 2024-09-17

### Fixed

- [**breaking**] Remove Tag and TagTree from module root

## [0.17.3](https://github.com/librasn/rasn/compare/rasn-snmp-v0.17.2...rasn-snmp-v0.17.3) - 2024-09-12

### Other

- update Cargo.toml dependencies

## [0.12.0](https://github.com/XAMPPRocky/rasn/compare/rasn-snmp-v0.11.1...rasn-snmp-v0.12.0) - 2023-11-12

### Other
- Feature/jer ([#187](https://github.com/XAMPPRocky/rasn/pull/187))

## [0.10.0](https://github.com/XAMPPRocky/rasn/compare/rasn-snmp-v0.9.5...rasn-snmp-v0.10.0) - 2023-10-03

### Other
- fmt
- use existing codec type
- document Codec and methods
- Add `Codec` enum, update `rasn_snmp::v3::Message`

## [0.9.3](https://github.com/XAMPPRocky/rasn/compare/rasn-snmp-v0.9.2...rasn-snmp-v0.9.3) - 2023-08-06

### Other
- fmt

## [0.9.2](https://github.com/XAMPPRocky/rasn/compare/rasn-snmp-v0.9.1...rasn-snmp-v0.9.2) - 2023-08-04

### Other
- Fix SNMP test

## [0.9.0](https://github.com/XAMPPRocky/rasn/compare/rasn-snmp-v0.8.2...rasn-snmp-v0.9.0) - 2023-07-30

### Other
- Use explicit prefix for Nested

## [0.8.0](https://github.com/XAMPPRocky/rasn/compare/rasn-snmp-v0.7.0...rasn-snmp-v0.8.0) - 2023-07-11

### Other
- Use workspace metadata
- Implement Unpacked Encoding Rules (UPER)
