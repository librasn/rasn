# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.27.1](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.27.0...rasn-derive-impl-v0.27.1) - 2025-08-07

### Other

- clippy pendantic fixes ([#444](https://github.com/librasn/rasn/pull/444))
+# Changelog
+All notable changes to this project will be documented in this file.
+
+The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
+and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
+
+## [Unreleased]

## [0.28.8](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.28.7...rasn-derive-impl-v0.28.8) - 2026-02-20

### Other

- *(rustc)* bump workspace to rust 2024 edition ([#539](https://github.com/librasn/rasn/pull/539))
- release v0.28.7 ([#534](https://github.com/librasn/rasn/pull/534))

## [0.28.7](https://github.com/librasn/rasn/releases/tag/rasn-derive-impl-v0.28.7) - 2026-01-22

### Fixed

- Actually use number of root variants in CHOICE variance constraints ([#532](https://github.com/librasn/rasn/pull/532))

### Other

- release v0.28.6 ([#531](https://github.com/librasn/rasn/pull/531))

## [0.28.7](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.28.6...rasn-derive-impl-v0.28.7) - 2026-01-22

### Fixed

- Actually use number of root variants in CHOICE variance constraints ([#532](https://github.com/librasn/rasn/pull/532))

## [0.28.3](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.28.2...rasn-derive-impl-v0.28.3) - 2026-01-08

### Added

- #[derive(Decode)] changes to make generics more robust ([#515](https://github.com/librasn/rasn/pull/515))

## [0.28.2](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.28.1...rasn-derive-impl-v0.28.2) - 2025-12-23

### Other

- Fixing #[derive(Decode)] issue where generics were not passed into the decode op, causing an error when deriving/compiling ([#513](https://github.com/librasn/rasn/pull/513))

## [0.28.1](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.28.0...rasn-derive-impl-v0.28.1) - 2025-11-19

### Fixed

- Sanitize field attributes when expanding Decode and Encode ([#509](https://github.com/librasn/rasn/pull/509))

## [0.28.0](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.27.4...rasn-derive-impl-v0.28.0) - 2025-10-20

### Fixed

- use full paths for field encode/decode, remove nightly CI ([#499](https://github.com/librasn/rasn/pull/499))

## [0.27.0](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.26.6...rasn-derive-impl-v0.27.0) - 2025-06-19

### Fixed

- *(OER/PER)* detect choice type always correctly on explicit prefix ([#479](https://github.com/librasn/rasn/pull/479))

### Other

- *(ci)* fix clippy lints

## [0.26.2](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.26.1...rasn-derive-impl-v0.26.2) - 2025-04-17

### Added

- support multi-field tuple-structs with delegate if other types are ([#436](https://github.com/librasn/rasn/pull/436))

### Fixed

- improve constraint performance and bounded intersection ([#440](https://github.com/librasn/rasn/pull/440))

## [0.26.0](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.25.1...rasn-derive-impl-v0.26.0) - 2025-03-23

### Other

- Improve constraint compiler errors ([#428](https://github.com/librasn/rasn/pull/428))
- Add decoding iterator, decode_with_remainder, clippy fixes ([#421](https://github.com/librasn/rasn/pull/421))

## [0.25.1](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.25.0...rasn-derive-impl-v0.25.1) - 2025-03-08

### Other

- Fixes librasn/rasn#418. Add support for explict tags and default values to encode correctly ([#419](https://github.com/librasn/rasn/pull/419))

## [0.25.0](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.24.0...rasn-derive-impl-v0.25.0) - 2025-03-07

### Other

- Feat/xml encoding rules ([#416](https://github.com/librasn/rasn/pull/416))

## [0.22.2](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.22.1...rasn-derive-impl-v0.22.2) - 2025-01-05

### Other

- Better error messages for derives ([#400](https://github.com/librasn/rasn/pull/400))

## [0.22.1](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.22.0...rasn-derive-impl-v0.22.1) - 2025-01-03

### Fixed

- *(derive)* use `const` blocks and add generic bounds when generating an enum's `AsnType` impl (#398)

### Other

- OER: remove nom usage, improve errors (#384)

## [0.22.0](https://github.com/librasn/rasn/compare/rasn-derive-impl-v0.21.0...rasn-derive-impl-v0.22.0) - 2024-11-26

### Other

- [**breaking**] Add lifetime for `encoder` trait and add allocation improvements based on that (OER) ([#370](https://github.com/librasn/rasn/pull/370))
- OER: improve decoding presence tracking ([#375](https://github.com/librasn/rasn/pull/375))
- Make constraints explicitly constant and evaluated in compile time & move some computation there (OER/PER) ([#318](https://github.com/librasn/rasn/pull/318))
