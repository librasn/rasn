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

## [0.28.8](https://github.com/librasn/rasn/compare/rasn-pkix-v0.28.7...rasn-pkix-v0.28.8) - 2026-02-20

### Other

- release v0.28.8 ([#535](https://github.com/librasn/rasn/pull/535))
- *(rustc)* bump workspace to rust 2024 edition ([#539](https://github.com/librasn/rasn/pull/539))

## [0.28.8](https://github.com/librasn/rasn/compare/rasn-pkix-v0.28.7...rasn-pkix-v0.28.8) - 2026-02-20

### Other

- *(rustc)* bump workspace to rust 2024 edition ([#539](https://github.com/librasn/rasn/pull/539))

## [0.27.3](https://github.com/librasn/rasn/compare/rasn-pkix-v0.27.2...rasn-pkix-v0.27.3) - 2025-09-27

### Other

- fix link.
- fix rustdoc errors

## [0.27.0](https://github.com/librasn/rasn/compare/rasn-pkix-v0.26.6...rasn-pkix-v0.27.0) - 2025-06-19

### Fixed

- [**breaking**] typo in RevokedCertificate ([#473](https://github.com/librasn/rasn/pull/473))

### Other

- reexport rasn default-features in standard crates ([#477](https://github.com/librasn/rasn/pull/477))

## [0.26.3](https://github.com/librasn/rasn/compare/rasn-pkix-v0.26.2...rasn-pkix-v0.26.3) - 2025-04-27

### Other

- Always emit and accept initial octet for empty BIT STRING ([#458](https://github.com/librasn/rasn/pull/458))
- Fix default value of TrustAnchorInfoVersion ([#455](https://github.com/librasn/rasn/pull/455))

## [0.26.2](https://github.com/librasn/rasn/compare/rasn-pkix-v0.26.1...rasn-pkix-v0.26.2) - 2025-04-17

### Fixed

- improve constraint performance and bounded intersection ([#440](https://github.com/librasn/rasn/pull/440))

### Other

- Add PrivateKeyUsagePeriod certificate extension. ([#448](https://github.com/librasn/rasn/pull/448))

## [0.25.0](https://github.com/librasn/rasn/compare/rasn-pkix-v0.24.0...rasn-pkix-v0.25.0) - 2025-03-07

### Other

- Feat/xml encoding rules ([#416](https://github.com/librasn/rasn/pull/416))

## [0.21.0](https://github.com/librasn/rasn/compare/rasn-pkix-v0.20.2...rasn-pkix-v0.21.0) - 2024-11-12

### Added

- Implement Algorithms and Identifiers PKIX module

### Fixed

- fmt
- Fix most issues related to dependency update, except hashbrown version ([#349](https://github.com/librasn/rasn/pull/349))

### Other

- Optimize field presence tracking of default/optional/extended fields ([#324](https://github.com/librasn/rasn/pull/324))
- update dependencies

## [0.20.2](https://github.com/librasn/rasn/compare/rasn-pkix-v0.20.1...rasn-pkix-v0.20.2) - 2024-10-18

### Fixed

- Run clippy and rustdoc only on stable channel in CI ([#342](https://github.com/librasn/rasn/pull/342))

## [0.18.0](https://github.com/librasn/rasn/compare/rasn-pkix-v0.17.3...rasn-pkix-v0.18.0) - 2024-09-17

### Added

- [**breaking**] Rework for SetOf type ([#325](https://github.com/librasn/rasn/pull/325))
- Constraint utilities, const default, more const functions ([#321](https://github.com/librasn/rasn/pull/321))

## [0.17.3](https://github.com/librasn/rasn/compare/rasn-pkix-v0.17.2...rasn-pkix-v0.17.3) - 2024-09-12

### Other

- update Cargo.toml dependencies

## [0.17.0](https://github.com/librasn/rasn/compare/rasn-pkix-v0.16.6...rasn-pkix-v0.17.0) - 2024-09-05

### Added
- [**breaking**] `Integer` as enum type and optimized constrained and variable-sized integer encoding ([#289](https://github.com/librasn/rasn/pull/289))

## [0.10.6](https://github.com/XAMPPRocky/rasn/compare/rasn-pkix-v0.10.5...rasn-pkix-v0.10.6) - 2023-10-26

### Other
- Add CI check for formatted files and reformat source ([#181](https://github.com/XAMPPRocky/rasn/pull/181))

## [0.10.0](https://github.com/XAMPPRocky/rasn/compare/rasn-pkix-v0.9.5...rasn-pkix-v0.10.0) - 2023-10-03

### Other
- Fix/issue 141 ([#158](https://github.com/XAMPPRocky/rasn/pull/158))

## [0.9.0](https://github.com/XAMPPRocky/rasn/compare/rasn-pkix-v0.8.2...rasn-pkix-v0.9.0) - 2023-07-30

### Other
- Replace ConstOid with Oid

## [0.8.0](https://github.com/XAMPPRocky/rasn/compare/rasn-pkix-v0.7.0...rasn-pkix-v0.8.0) - 2023-07-11

### Other
- Add constraints to PKIX
- Fix tag on PKIX CRL extension ([#121](https://github.com/XAMPPRocky/rasn/pull/121))
- Use workspace metadata
- Implement Unpacked Encoding Rules (UPER)
- Look at data string and pick best guess parse format. Does not yet ha… ([#118](https://github.com/XAMPPRocky/rasn/pull/118))
