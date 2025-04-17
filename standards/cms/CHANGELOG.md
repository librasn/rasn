# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.26.2](https://github.com/librasn/rasn/compare/rasn-cms-v0.26.1...rasn-cms-v0.26.2) - 2025-04-17

### Other

- RFC 3161 Time-Stamp Protocol (TSP) ASN.1 module.  ([#451](https://github.com/librasn/rasn/pull/451))
- Add CMS signed attribute CMSAlgorithmProtection. ([#449](https://github.com/librasn/rasn/pull/449))

## [0.21.0](https://github.com/librasn/rasn/compare/rasn-cms-v0.20.2...rasn-cms-v0.21.0) - 2024-11-12

### Fixed

- Fix most issues related to dependency update, except hashbrown version ([#349](https://github.com/librasn/rasn/pull/349))

### Other

- Optimize field presence tracking of default/optional/extended fields ([#324](https://github.com/librasn/rasn/pull/324))

## [0.20.2](https://github.com/librasn/rasn/compare/rasn-cms-v0.20.1...rasn-cms-v0.20.2) - 2024-10-18

### Fixed

- Run clippy and rustdoc only on stable channel in CI ([#342](https://github.com/librasn/rasn/pull/342))

## [0.18.0](https://github.com/librasn/rasn/compare/rasn-cms-v0.17.3...rasn-cms-v0.18.0) - 2024-09-17

### Added

- [**breaking**] Rework for SetOf type ([#325](https://github.com/librasn/rasn/pull/325))

## [0.9.3](https://github.com/XAMPPRocky/rasn/compare/rasn-cms-v0.9.2...rasn-cms-v0.9.3) - 2023-08-06

### Other
- fmt

## [0.9.0](https://github.com/XAMPPRocky/rasn/compare/rasn-cms-v0.8.2...rasn-cms-v0.9.0) - 2023-07-30

### Other
- Replace ConstOid with Oid

## [0.8.0](https://github.com/XAMPPRocky/rasn/compare/rasn-cms-v0.7.0...rasn-cms-v0.8.0) - 2023-07-11

### Other
- Use workspace metadata
