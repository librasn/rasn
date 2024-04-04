# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.0](https://github.com/librasn/rasn/compare/rasn-derive-v0.13.1...rasn-derive-v0.14.0) - 2024-04-04

### Other
- Feat/identifier annotation ([#239](https://github.com/librasn/rasn/pull/239))

## [0.12.6](https://github.com/librasn/rasn/compare/rasn-derive-v0.12.5...rasn-derive-v0.12.6) - 2024-03-09

### Other
- *(macros)* Treat Unit Structs as ASN.1 NULL ([#227](https://github.com/librasn/rasn/pull/227))
- Fix calling the specified default fn
- Add default_initializer_fn optimisation

## [0.12.5](https://github.com/librasn/rasn/compare/rasn-derive-v0.12.4...rasn-derive-v0.12.5) - 2024-02-02

### Fixed
- *(macro)* recognize option references ([#219](https://github.com/librasn/rasn/pull/219))

## [0.12.1](https://github.com/XAMPPRocky/rasn/compare/rasn-derive-v0.12.0...rasn-derive-v0.12.1) - 2023-11-14

### Other
- Fix PER ObjectIdentifier, Alignment for Choice index encoding ([#202](https://github.com/XAMPPRocky/rasn/pull/202))

## [0.12.0](https://github.com/XAMPPRocky/rasn/compare/rasn-derive-v0.11.1...rasn-derive-v0.12.0) - 2023-11-12

### Fixed
- *(macros)* handle negative discriminants

### Other
- Add `Option<T::EXTENDED_VARIANTS>` for Choice, clippy cleanup for relevant macros ([#200](https://github.com/XAMPPRocky/rasn/pull/200))
- Feature/jer ([#187](https://github.com/XAMPPRocky/rasn/pull/187))
- Field_error improved, Boxed error `kind`, explicit naming also for `DecodeErrorKind` ([#197](https://github.com/XAMPPRocky/rasn/pull/197))
- run cargo fmt

## [0.11.0](https://github.com/XAMPPRocky/rasn/compare/rasn-derive-v0.10.6...rasn-derive-v0.11.0) - 2023-10-28

### Other
- Shared error module ([#164](https://github.com/XAMPPRocky/rasn/pull/164))

## [0.10.6](https://github.com/XAMPPRocky/rasn/compare/rasn-derive-v0.10.5...rasn-derive-v0.10.6) - 2023-10-26

### Other
- Add CI check for formatted files and reformat source ([#181](https://github.com/XAMPPRocky/rasn/pull/181))

## [0.10.4](https://github.com/XAMPPRocky/rasn/compare/rasn-derive-v0.10.3...rasn-derive-v0.10.4) - 2023-10-16

### Other
- four uper issues ([#177](https://github.com/XAMPPRocky/rasn/pull/177))

## [0.10.2](https://github.com/XAMPPRocky/rasn/compare/rasn-derive-v0.10.1...rasn-derive-v0.10.2) - 2023-10-10

### Other
- Fix/issue 165 ([#172](https://github.com/XAMPPRocky/rasn/pull/172))

## [0.10.0](https://github.com/XAMPPRocky/rasn/compare/rasn-derive-v0.9.5...rasn-derive-v0.10.0) - 2023-10-03

### Other
- Gensym field names ([#166](https://github.com/XAMPPRocky/rasn/pull/166))
- Delegate newtype EOC ([#163](https://github.com/XAMPPRocky/rasn/pull/163))
- Fix Result scoping ([#162](https://github.com/XAMPPRocky/rasn/pull/162))
- Fix/infinite recursion ([#157](https://github.com/XAMPPRocky/rasn/pull/157))
- Fix/constrained extension ([#156](https://github.com/XAMPPRocky/rasn/pull/156))

## [0.6.1](https://github.com/XAMPPRocky/rasn/compare/rasn-derive-v0.6.0...rasn-derive-v0.6.1) - 2023-07-11

### Other
- Add constraints to PKIX
- clippy
- Implement Unpacked Encoding Rules (UPER)
