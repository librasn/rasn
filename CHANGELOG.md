# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.1](https://github.com/librasn/rasn/compare/rasn-v0.16.0...rasn-v0.16.1) - 2024-08-04

### Other
- Add integer value constraint checking for PER encoding ([#287](https://github.com/librasn/rasn/pull/287))

## [0.16.0](https://github.com/librasn/rasn/compare/rasn-v0.15.3...rasn-v0.16.0) - 2024-07-17

### Fixed
- pass constraints correctly when encoding optional type ([#276](https://github.com/librasn/rasn/pull/276))
- Hide `jer` behind feature (implying `std`). ([#266](https://github.com/librasn/rasn/pull/266))

### Other
- Fix error name regression introduced in a75b26b ([#285](https://github.com/librasn/rasn/pull/285))
- Unify and Improve error handling of restricted strings ([#269](https://github.com/librasn/rasn/pull/269))
- fix untagged sequence bitfield bits ([#279](https://github.com/librasn/rasn/pull/279))
- fix encoding of empty extension values ([#278](https://github.com/librasn/rasn/pull/278))
- correctly encode null in option in extensions ([#277](https://github.com/librasn/rasn/pull/277))
- fix empty length decoding, no zero padding allowed ([#274](https://github.com/librasn/rasn/pull/274))
- decode error when unused preamble bits are not zero ([#273](https://github.com/librasn/rasn/pull/273))
- check quantity data size on sequence of ([#275](https://github.com/librasn/rasn/pull/275))
- Fix invalid handling of zero padding on length of length on COER ([#268](https://github.com/librasn/rasn/pull/268))
- Hide `snafu` backtraces behind `std`. ([#265](https://github.com/librasn/rasn/pull/265))

## [0.15.3](https://github.com/librasn/rasn/compare/rasn-v0.15.2...rasn-v0.15.3) - 2024-06-14

### Other
- *(ber)* don't allocate `2 + N` `BigInt`s when parsing OIDs ([#263](https://github.com/librasn/rasn/pull/263))

## [0.15.2](https://github.com/librasn/rasn/compare/rasn-v0.15.1...rasn-v0.15.2) - 2024-06-11

### Fixed
- decoding integers encoded with byte lengths greater than `{integer}::BITS / 8` would cause a subtraction underflow and subsequent index panic ([#257](https://github.com/librasn/rasn/pull/257))

## [0.15.1](https://github.com/librasn/rasn/compare/rasn-v0.15.0...rasn-v0.15.1) - 2024-06-10

### Other
- Add OID for MGF1. ([#255](https://github.com/librasn/rasn/pull/255))
- don't heap allocate a BigInt for every integer that is decoded ([#256](https://github.com/librasn/rasn/pull/256))
- run Clippy also for tests ([#252](https://github.com/librasn/rasn/pull/252))

## [0.15.0](https://github.com/librasn/rasn/compare/rasn-v0.14.0...rasn-v0.15.0) - 2024-05-17

### Other
- conditional update on Windows builds ([#251](https://github.com/librasn/rasn/pull/251))
- oer enumerated de handled zero padding when it should not ([#249](https://github.com/librasn/rasn/pull/249))
- Remove `backtraces` from `rasn`'s `default` feature ([#247](https://github.com/librasn/rasn/pull/247))
- Octet Encoding Rules ([#154](https://github.com/librasn/rasn/pull/154))

## [0.14.0](https://github.com/librasn/rasn/compare/rasn-v0.13.1...rasn-v0.14.0) - 2024-04-04

### Fixed
- *(jer)* ENUMERATED values as strings ([#242](https://github.com/librasn/rasn/pull/242))

### Other
- Fix/jer compliance ([#243](https://github.com/librasn/rasn/pull/243))
- Feat/identifier annotation ([#239](https://github.com/librasn/rasn/pull/239))

## [0.13.1](https://github.com/librasn/rasn/compare/rasn-v0.13.0...rasn-v0.13.1) - 2024-03-21

### Other
- Manually fix the incompatible versions (a result of v0.13.0 release) and some clippy warnings ([#237](https://github.com/librasn/rasn/pull/237))

## [0.12.6](https://github.com/librasn/rasn/compare/rasn-v0.12.5...rasn-v0.12.6) - 2024-03-09

### Fixed
- fix char and index maps, enable tests ([#230](https://github.com/librasn/rasn/pull/230))
- fix no_std being commented out
- *(types)* stack overflow in ObjectIdentifier PartialEq ([#223](https://github.com/librasn/rasn/pull/223))

### Other
- *(macros)* Treat Unit Structs as ASN.1 NULL ([#227](https://github.com/librasn/rasn/pull/227))
- *(README)* add declaration reference ([#159](https://github.com/librasn/rasn/pull/159))
- Fix calling the specified default fn
- Add default_initializer_fn optimisation

## [0.12.5](https://github.com/librasn/rasn/compare/rasn-v0.12.4...rasn-v0.12.5) - 2024-02-02

### Fixed
- *(der)* sort SET OF items ([#220](https://github.com/librasn/rasn/pull/220))
- *(macro)* recognize option references ([#219](https://github.com/librasn/rasn/pull/219))

### Other
- remove legacy compiler ([#221](https://github.com/librasn/rasn/pull/221))
- Make backtraces as feature, enabled by default ([#214](https://github.com/librasn/rasn/pull/214))

## [0.12.4](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.12.3...rasn-v0.12.4) - 2023-12-04

### Other
- Remove deprecated chrono functions, redundant constant lifetimes ([#212](https://github.com/XAMPPRocky/rasn/pull/212))

## [0.12.3](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.12.2...rasn-v0.12.3) - 2023-11-25

### Other
- Fix issue with decoding of BasicOcspResponse. ([#208](https://github.com/XAMPPRocky/rasn/pull/208))

## [0.12.2](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.12.1...rasn-v0.12.2) - 2023-11-23

### Other
- Fix/issue 204 ([#206](https://github.com/XAMPPRocky/rasn/pull/206))

## [0.12.1](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.12.0...rasn-v0.12.1) - 2023-11-14

### Other
- Fix PER ObjectIdentifier, Alignment for Choice index encoding ([#202](https://github.com/XAMPPRocky/rasn/pull/202))

## [0.12.0](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.11.1...rasn-v0.12.0) - 2023-11-12

### Fixed
- *(macros)* handle negative discriminants

### Other
- Add `Option<T::EXTENDED_VARIANTS>` for Choice, clippy cleanup for relevant macros ([#200](https://github.com/XAMPPRocky/rasn/pull/200))
- Fix issue [#192](https://github.com/XAMPPRocky/rasn/pull/192), add APER ExtensiblePersonnelRecord with bug fixes  ([#199](https://github.com/XAMPPRocky/rasn/pull/199))
- Feature/jer ([#187](https://github.com/XAMPPRocky/rasn/pull/187))
- Field_error improved, Boxed error `kind`, explicit naming also for `DecodeErrorKind` ([#197](https://github.com/XAMPPRocky/rasn/pull/197))
- run cargo fmt

## [0.11.1](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.11.0...rasn-v0.11.1) - 2023-11-06

### Other
- Use `ok_or_else` instead of `ok_or` in Error handling for performance ([#195](https://github.com/XAMPPRocky/rasn/pull/195))
- Move some generally useful functions from PER to utility module ([#190](https://github.com/XAMPPRocky/rasn/pull/190))

## [0.11.0](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.10.6...rasn-v0.11.0) - 2023-10-28

### Other
- Shared error module ([#164](https://github.com/XAMPPRocky/rasn/pull/164))

## [0.10.6](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.10.5...rasn-v0.10.6) - 2023-10-26

### Other
- Fix deprecated chrono functions ([#184](https://github.com/XAMPPRocky/rasn/pull/184))
- Add CI check for formatted files and reformat source ([#181](https://github.com/XAMPPRocky/rasn/pull/181))

## [0.10.5](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.10.4...rasn-v0.10.5) - 2023-10-19

### Fixed
- *(per)* encoding extensible string types ([#179](https://github.com/XAMPPRocky/rasn/pull/179))

## [0.10.4](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.10.3...rasn-v0.10.4) - 2023-10-16

### Other
- four uper issues ([#177](https://github.com/XAMPPRocky/rasn/pull/177))

## [0.10.3](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.10.2...rasn-v0.10.3) - 2023-10-11

### Other
- Fixes [#146](https://github.com/XAMPPRocky/rasn/pull/146), VisibleString character set ([#147](https://github.com/XAMPPRocky/rasn/pull/147))
- Fix/uper integer ([#174](https://github.com/XAMPPRocky/rasn/pull/174))

## [0.10.2](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.10.1...rasn-v0.10.2) - 2023-10-10

### Other
- Fix/issue 165 ([#172](https://github.com/XAMPPRocky/rasn/pull/172))

## [0.10.1](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.10.0...rasn-v0.10.1) - 2023-10-10

### Fixed
- *(de)* missing `decode_default_with_tag_and_constraints` ([#170](https://github.com/XAMPPRocky/rasn/pull/170))

### Other
- Fix nested choice when no struct/set present ([#169](https://github.com/XAMPPRocky/rasn/pull/169))
- Update rust.yml

## [0.10.0](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.9.5...rasn-v0.10.0) - 2023-10-03

### Other
- Gensym field names ([#166](https://github.com/XAMPPRocky/rasn/pull/166))
- Delegate newtype EOC ([#163](https://github.com/XAMPPRocky/rasn/pull/163))
- Fix Result scoping ([#162](https://github.com/XAMPPRocky/rasn/pull/162))
- Fix/issue 141 ([#158](https://github.com/XAMPPRocky/rasn/pull/158))
- Fix/infinite recursion ([#157](https://github.com/XAMPPRocky/rasn/pull/157))
- Fix/constrained extension ([#156](https://github.com/XAMPPRocky/rasn/pull/156))
- fmt
- Add FixedBitString, use BitStr for encoding
- use existing codec type
- Split and improve UTCTime type ([#152](https://github.com/XAMPPRocky/rasn/pull/152))
- split OID codec functionality to be usable by other codecs ([#144](https://github.com/XAMPPRocky/rasn/pull/144))
- document Codec and methods
- Add `Codec` enum, update `rasn_snmp::v3::Message`
- Improves BER GeneralizedTime decoders, support for CER/DER strict mode ([#150](https://github.com/XAMPPRocky/rasn/pull/150))

## [0.9.5](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.9.4...rasn-v0.9.5) - 2023-08-24

### Other
- Add TryFrom for str type for PrintableString, docs, minor fixes ([#149](https://github.com/XAMPPRocky/rasn/pull/149))
- Improved size constraint ([#142](https://github.com/XAMPPRocky/rasn/pull/142))

## [0.9.4](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.9.3...rasn-v0.9.4) - 2023-08-08

### Other
- Fix OID validation, add docs build for CI, prevent spaces and globbin… ([#140](https://github.com/XAMPPRocky/rasn/pull/140))
- Possibly some safer arithmetics ([#137](https://github.com/XAMPPRocky/rasn/pull/137))

## [0.9.3](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.9.2...rasn-v0.9.3) - 2023-08-06

### Other
- fmt
- Fix #[no_std] missing from lib

## [0.9.2](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.9.1...rasn-v0.9.2) - 2023-08-04

### Other
- Update Cargo.toml
- Use workspace for macros
- correctly handle non-byte-aligned BIT STRING length ([#135](https://github.com/XAMPPRocky/rasn/pull/135))
- Fix Clippy warnings ([#132](https://github.com/XAMPPRocky/rasn/pull/132))
- Fix SNMP test

## [0.9.1](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.9.0...rasn-v0.9.1) - 2023-08-01

### Fixed
- remove duplicates in PrintableString char set ([#130](https://github.com/XAMPPRocky/rasn/pull/130))

### Other
- Update version in standards

## [0.9.0](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.8.2...rasn-v0.9.0) - 2023-07-30

### Fixed
- fix/AttributeCertificateValidityPeriod pub fields
- fixing kdc/as rep application tag
- fix explicit prefix decode & encode for macros
- fix errors in Kerberos OTP
- fix est
- fix missing trait in test
- fix snmp tests
- fix path dep versions
- fixed implicit tagging of sequences
- fix kws
- fix
- fix missing pieces for derive

### Other
- Add FixedOctetString to enforce octet string length at type level
- Use explicit prefix for Nested
- Remove BIT STRING null byte truncation in BER
- Replace ConstOid with Oid
- Update dependencies
- update workspace version
- Release 0.8.2
- bump macros
- Release 0.8.1
- release ([#122](https://github.com/XAMPPRocky/rasn/pull/122))
- Create release-plz.yaml
- Add size comparison example
- Add constraints to PKIX
- Update README.md
- Update README.md
- Update README.md
- clippy
- Implement Aligned Encoding Rules (APER)
- Fix tag on PKIX CRL extension ([#121](https://github.com/XAMPPRocky/rasn/pull/121))
- Use workspace metadata
- Implement Unpacked Encoding Rules (UPER)
- Updates for SHA AlgorithmIdentifier Oids; use `==` for ConstOid and ObjectIdentifier ([#120](https://github.com/XAMPPRocky/rasn/pull/120))
- Look at data string and pick best guess parse format. Does not yet ha… ([#118](https://github.com/XAMPPRocky/rasn/pull/118))
- Disable *android
- Disable mips*
- Fix chrono deprecation warning
- Release 0.7.0
- Trust Anchors according to RFC 5914 added, including Oid for new content type. Passes cargo test, but no new specific tests added ([#116](https://github.com/XAMPPRocky/rasn/pull/116))
- Allow buffer reuse in `Encoder` ([#106](https://github.com/XAMPPRocky/rasn/pull/106))
- Fixes [#97](https://github.com/XAMPPRocky/rasn/pull/97) Update SNMP v2 module to better match RFC 3416 ([#104](https://github.com/XAMPPRocky/rasn/pull/104))
- Added Microsoft Authenticode structures ([#102](https://github.com/XAMPPRocky/rasn/pull/102))
- Release 0.6.1
- Bugfix for indefinite Any parsing, fixes [#66](https://github.com/XAMPPRocky/rasn/pull/66) ([#95](https://github.com/XAMPPRocky/rasn/pull/95))
- Release 0.6.0
- very very basic compiler implementation ([#92](https://github.com/XAMPPRocky/rasn/pull/92))
- Update rust.yml
- add old compiler
- Convert `Result` in macros to `core::result::Result`
- Make AttributeCertificateClearAttributes fields public
- Add:
- Release 0.5.3
- cleaning as_req test
- adding as_rep test
- making KrbCredInfo tags explicit
- making EncryptedData tags explicit
- Release 0.5.2
- fmt
- Account for every possible position of #[tag(explicit)]
- Add repository field to rasn-pkix
- Release 0.5.1
- updating test to full as_req
- fmt
- Update dependnencies
- More explicit prefix macro fixes
- Update deps
- Fix explicit tagging in kerberos
- Fix explicit tag parsing
- Fixes [#67](https://github.com/XAMPPRocky/rasn/pull/67). Fix SHA1_RSA OID typo and add RSAES_OAEP OID
- Add pkcs7 compat module.
- Update README.md
- Update README.md
- Update README.md
- Release 0.5.0
- Replace static_assertions with 1.57 const assert
- Fix tests and docs
- Move the top-level documentation into README.md
- Rename IntoOpaque to ToOpaque
- Formatting
- Add initial implementation of S/MIME
- Add documentation for OCSP
- Add documentation to Kerberos
- Remove OID suffix in CMS
- Add Attribute Certificate in PKIX
- Add docs and minor improvements to Kerberos
- format macros
- Add more derives for OCSP
- Add more docs and format CMS
- Implement IETF RFC 3370 and RFC 4108
- impl Hash for all types in PKIX
- impl Hash for Any, and add a bunch more OIDs
- Fix default parsing and TAG_TREE for SEQUENCE variants
- Fix long form tag decoding
- Add support for Kerberos & OCSP, and fix explicit tag codegen
- Add PersonnelRecord test
- Add address family numbers
- Formatting
- Use ty directly instead of inference in macro default
- Release 0.4.3
- Added Decoder::decoded_len method to support stream-oriented protocols
- Release 0.4.2
- Added missing pub keyword to some fields
- Added contstructors for non-exhaustive LDAP structs. Fixes [#48](https://github.com/XAMPPRocky/rasn/pull/48).
- Update README.md
- Update README.md
- Update README.md
- Release 0.4.1
- Added CMS standard: https://datatracker.ietf.org/doc/html/rfc5652 ([#47](https://github.com/XAMPPRocky/rasn/pull/47))
- Update README.md
- Fix wording in README.md ([#46](https://github.com/XAMPPRocky/rasn/pull/46))
- Update README.md
- Release 0.4.0
- fmt benches
- Clippy fixes and formatting
- Add rfc links
- misc improvements
- Add more documentation
- Add DecryptKey OID and prelude module
- Reformat, and Copy implementations, document, and add convienvience methods for version
- Add x509 benchmark
- update dependencies
- Add new example for RFC 7030 standard ([#41](https://github.com/XAMPPRocky/rasn/pull/41))
- Make is_unique an associated method
- Add LDAP implementation
- Update and full test for PKIX certificates
- refactor field encoding and decoding to be more DRY and use default attribute
- Fix remaining codec bugs found
- Switch ObjectIdentifier to using Cow internally
- Fix explicit prefix encoding
- Add public methods for Any
- Update docs and teletex string
- Add CHOICE Tag alias.
- More fuzzing fixes
- Improve error message more
- Validate constructed bit is present
- Update fuzzing code
- Add more derives to SNMP v3 types
- Add rasn-pkix crate
- Fix time decoding & encoding
- Add better error messages for fields and choices
- Add support for SET and explicit tags to macros
- Add support for SET decoding
- Add NumericString alias and add Ord to InstanceOf
- cleanup tests
- Add SET encoding
- Add issue 34 test case, closes [#34](https://github.com/XAMPPRocky/rasn/pull/34)
- Update issue 35 test
- Update README.md
- Update README.md
- rm dbg
- Ensure explict prefix encodes constructed values correctly. Closes [#35](https://github.com/XAMPPRocky/rasn/pull/35)
- fmt
- Add SEQUENCE OF alias
- Remove unneeded newtype
- Move OID structure and add more OIDs
- Add support for ANY type
- Add support for SET OF types
- Add all OIDs related to RFC 5280
- Hide warnings for now
- Rename Ia5String
- Add Any type
- Use wildcard for standards directory
- Fix option decoding
- Add SNMPv3 to rasn-snmp. ([#33](https://github.com/XAMPPRocky/rasn/pull/33))
- Add more trait impls
- Fix derive codegen
- Fix OID encoding and decoding for BER and DER ([#29](https://github.com/XAMPPRocky/rasn/pull/29))
- Making Implicit and Explicit value's public ([#30](https://github.com/XAMPPRocky/rasn/pull/30))
- Move tagging inside the trait implementations
- Revert "Make field encoding generation more DRY"
- Add all OIDs from UsefulDefinitions ASN.1 module
- Make field encoding generation more DRY
- Add SNMP fuzzing
- Fix proc macro struct field codecs
- Improved README, and automatic_tags
- Fix tag detection in newtype wrappers
- Changed `slice_range(foo, _, foo.len())` to `slice_from(foo, _)` ([#23](https://github.com/XAMPPRocky/rasn/pull/23))
- Fix tagged choice variants
- Small improvements to error messages, and clean up newly-dead tag constant defs ([#22](https://github.com/XAMPPRocky/rasn/pull/22))
- Add proc macros and add trap message test
- Add docs and tidy up
- Create TagTree and refactor Tag validation and CHOICE decoding
- Add iec61850 test
- Update build.bash
- Update test.bash
- Update README.md
- Release 0.3.1
- Make SNMP fields public, fixes [#14](https://github.com/XAMPPRocky/rasn/pull/14)
- fix some typos ([#15](https://github.com/XAMPPRocky/rasn/pull/15))
- add decription and language
- Update README.md
- Remove bad files
- Release 0.3.0
- Add more documentation
- Use IpAddress directly
- Implement MIB-II, Add #[rasn(delegate)] and #[rasn(default)]
- Add support for generics in proc macro
- Move into v1 modules
- Implement SMIv1 and MIB-1
- Add initial benchmarks
- Add more fuzzing input data
- Add test case
- Reverted back to stable compatiblity, fixed various issues.
- Improved various aspects of rasn-macros
- version bump
- Add Oid and use const generics for tag
- Fix docs.rs badge
- bump version
- Update rust.yml
- Release 0.2.1
- Add a new `#[rasn(choice)]` field attribute
- Upgrade bitvec to 0.19.3
- Upgrade nom to 0.6.0-beta1
- Update README.md
- add more docs and licence files
- Update README.md
- Update README.md
- Update README.md
- Update set_rust_version.bash
- remove old dependency
- Release 0.2.0
- Update rust.yml
- rm println
- upgrade nom
- Update README.md
- add ci scripts
- add docs
- Add metadata to macros crate
- Add more metadata
- Create rust.yml
- more docs
- Add docs and fix constructed encoding for strings
- correctly handle indefinite lengths
- Added more fuzzing fixes
- Implement DER & CER
- Implement automatic tagging
- Implement tagging attribute for all valid positions
- Refactor tests, add option impl, add docs
- split up proc macro exports
- Add static asserts and time types
- Create README.md
- Fix more macro bugs and make open type complete
- fuzzing fixes
- fmt
- Implemented initial support for choice enums
- Add initial macro support for structs and enums
- Removing .vscode from git history
- Add encoder impl and tests
- Add encoder stub
- Refactor tag out of decode
- TagValue -> AsnType
- refactored parser into its own file
- Fix dependency features
- Refactor trait and error handling
- Refactor Decoder trait
- add string
- Refactored constructed encoding parsing to be generic
- Add bitstring
- add types module
- Added octet string, null, and object identifier
- Refactor error handling
- refactor tag from identifier
- Add initial ber parser
- new project

## [0.8.0](https://github.com/XAMPPRocky/rasn/compare/rasn-v0.7.0...rasn-v0.8.0) - 2023-07-11

### Fixed
- fix/AttributeCertificateValidityPeriod pub fields
- fixing kdc/as rep application tag
- fix explicit prefix decode & encode for macros
- fix errors in Kerberos OTP
- fix est
- fix missing trait in test
- fix snmp tests
- fix path dep versions
- fixed implicit tagging of sequences
- fix kws
- fix
- fix missing pieces for derive

### Other
- Create release-plz.yaml
- Add size comparison example
- Add constraints to PKIX
- Update README.md
- Update README.md
- Update README.md
- clippy
- Implement Aligned Encoding Rules (APER)
- Fix tag on PKIX CRL extension ([#121](https://github.com/XAMPPRocky/rasn/pull/121))
- Use workspace metadata
- Implement Unpacked Encoding Rules (UPER)
- Updates for SHA AlgorithmIdentifier Oids; use `==` for ConstOid and ObjectIdentifier ([#120](https://github.com/XAMPPRocky/rasn/pull/120))
- Look at data string and pick best guess parse format. Does not yet ha… ([#118](https://github.com/XAMPPRocky/rasn/pull/118))
- Disable *android
- Disable mips*
- Fix chrono deprecation warning
- Release 0.7.0
- Trust Anchors according to RFC 5914 added, including Oid for new content type. Passes cargo test, but no new specific tests added ([#116](https://github.com/XAMPPRocky/rasn/pull/116))
- Allow buffer reuse in `Encoder` ([#106](https://github.com/XAMPPRocky/rasn/pull/106))
- Fixes [#97](https://github.com/XAMPPRocky/rasn/pull/97) Update SNMP v2 module to better match RFC 3416 ([#104](https://github.com/XAMPPRocky/rasn/pull/104))
- Added Microsoft Authenticode structures ([#102](https://github.com/XAMPPRocky/rasn/pull/102))
- Release 0.6.1
- Bugfix for indefinite Any parsing, fixes [#66](https://github.com/XAMPPRocky/rasn/pull/66) ([#95](https://github.com/XAMPPRocky/rasn/pull/95))
- Release 0.6.0
- very very basic compiler implementation ([#92](https://github.com/XAMPPRocky/rasn/pull/92))
- Update rust.yml
- add old compiler
- Convert `Result` in macros to `core::result::Result`
- Make AttributeCertificateClearAttributes fields public
- Add:
- Release 0.5.3
- cleaning as_req test
- adding as_rep test
- making KrbCredInfo tags explicit
- making EncryptedData tags explicit
- Release 0.5.2
- fmt
- Account for every possible position of #[tag(explicit)]
- Add repository field to rasn-pkix
- Release 0.5.1
- updating test to full as_req
- fmt
- Update dependnencies
- More explicit prefix macro fixes
- Update deps
- Fix explicit tagging in kerberos
- Fix explicit tag parsing
- Fixes [#67](https://github.com/XAMPPRocky/rasn/pull/67). Fix SHA1_RSA OID typo and add RSAES_OAEP OID
- Add pkcs7 compat module.
- Update README.md
- Update README.md
- Update README.md
- Release 0.5.0
- Replace static_assertions with 1.57 const assert
- Fix tests and docs
- Move the top-level documentation into README.md
- Rename IntoOpaque to ToOpaque
- Formatting
- Add initial implementation of S/MIME
- Add documentation for OCSP
- Add documentation to Kerberos
- Remove OID suffix in CMS
- Add Attribute Certificate in PKIX
- Add docs and minor improvements to Kerberos
- format macros
- Add more derives for OCSP
- Add more docs and format CMS
- Implement IETF RFC 3370 and RFC 4108
- impl Hash for all types in PKIX
- impl Hash for Any, and add a bunch more OIDs
- Fix default parsing and TAG_TREE for SEQUENCE variants
- Fix long form tag decoding
- Add support for Kerberos & OCSP, and fix explicit tag codegen
- Add PersonnelRecord test
- Add address family numbers
- Formatting
- Use ty directly instead of inference in macro default
- Release 0.4.3
- Added Decoder::decoded_len method to support stream-oriented protocols
- Release 0.4.2
- Added missing pub keyword to some fields
- Added contstructors for non-exhaustive LDAP structs. Fixes [#48](https://github.com/XAMPPRocky/rasn/pull/48).
- Update README.md
- Update README.md
- Update README.md
- Release 0.4.1
- Added CMS standard: https://datatracker.ietf.org/doc/html/rfc5652 ([#47](https://github.com/XAMPPRocky/rasn/pull/47))
- Update README.md
- Fix wording in README.md ([#46](https://github.com/XAMPPRocky/rasn/pull/46))
- Update README.md
- Release 0.4.0
- fmt benches
- Clippy fixes and formatting
- Add rfc links
- misc improvements
- Add more documentation
- Add DecryptKey OID and prelude module
- Reformat, and Copy implementations, document, and add convienvience methods for version
- Add x509 benchmark
- update dependencies
- Add new example for RFC 7030 standard ([#41](https://github.com/XAMPPRocky/rasn/pull/41))
- Make is_unique an associated method
- Add LDAP implementation
- Update and full test for PKIX certificates
- refactor field encoding and decoding to be more DRY and use default attribute
- Fix remaining codec bugs found
- Switch ObjectIdentifier to using Cow internally
- Fix explicit prefix encoding
- Add public methods for Any
- Update docs and teletex string
- Add CHOICE Tag alias.
- More fuzzing fixes
- Improve error message more
- Validate constructed bit is present
- Update fuzzing code
- Add more derives to SNMP v3 types
- Add rasn-pkix crate
- Fix time decoding & encoding
- Add better error messages for fields and choices
- Add support for SET and explicit tags to macros
- Add support for SET decoding
- Add NumericString alias and add Ord to InstanceOf
- cleanup tests
- Add SET encoding
- Add issue 34 test case, closes [#34](https://github.com/XAMPPRocky/rasn/pull/34)
- Update issue 35 test
- Update README.md
- Update README.md
- rm dbg
- Ensure explict prefix encodes constructed values correctly. Closes [#35](https://github.com/XAMPPRocky/rasn/pull/35)
- fmt
- Add SEQUENCE OF alias
- Remove unneeded newtype
- Move OID structure and add more OIDs
- Add support for ANY type
- Add support for SET OF types
- Add all OIDs related to RFC 5280
- Hide warnings for now
- Rename Ia5String
- Add Any type
- Use wildcard for standards directory
- Fix option decoding
- Add SNMPv3 to rasn-snmp. ([#33](https://github.com/XAMPPRocky/rasn/pull/33))
- Add more trait impls
- Fix derive codegen
- Fix OID encoding and decoding for BER and DER ([#29](https://github.com/XAMPPRocky/rasn/pull/29))
- Making Implicit and Explicit value's public ([#30](https://github.com/XAMPPRocky/rasn/pull/30))
- Move tagging inside the trait implementations
- Revert "Make field encoding generation more DRY"
- Add all OIDs from UsefulDefinitions ASN.1 module
- Make field encoding generation more DRY
- Add SNMP fuzzing
- Fix proc macro struct field codecs
- Improved README, and automatic_tags
- Fix tag detection in newtype wrappers
- Changed `slice_range(foo, _, foo.len())` to `slice_from(foo, _)` ([#23](https://github.com/XAMPPRocky/rasn/pull/23))
- Fix tagged choice variants
- Small improvements to error messages, and clean up newly-dead tag constant defs ([#22](https://github.com/XAMPPRocky/rasn/pull/22))
- Add proc macros and add trap message test
- Add docs and tidy up
- Create TagTree and refactor Tag validation and CHOICE decoding
- Add iec61850 test
- Update build.bash
- Update test.bash
- Update README.md
- Release 0.3.1
- Make SNMP fields public, fixes [#14](https://github.com/XAMPPRocky/rasn/pull/14)
- fix some typos ([#15](https://github.com/XAMPPRocky/rasn/pull/15))
- add decription and language
- Update README.md
- Remove bad files
- Release 0.3.0
- Add more documentation
- Use IpAddress directly
- Implement MIB-II, Add #[rasn(delegate)] and #[rasn(default)]
- Add support for generics in proc macro
- Move into v1 modules
- Implement SMIv1 and MIB-1
- Add initial benchmarks
- Add more fuzzing input data
- Add test case
- Reverted back to stable compatiblity, fixed various issues.
- Improved various aspects of rasn-macros
- version bump
- Add Oid and use const generics for tag
- Fix docs.rs badge
- bump version
- Update rust.yml
- Release 0.2.1
- Add a new `#[rasn(choice)]` field attribute
- Upgrade bitvec to 0.19.3
- Upgrade nom to 0.6.0-beta1
- Update README.md
- add more docs and licence files
- Update README.md
- Update README.md
- Update README.md
- Update set_rust_version.bash
- remove old dependency
- Release 0.2.0
- Update rust.yml
- rm println
- upgrade nom
- Update README.md
- add ci scripts
- add docs
- Add metadata to macros crate
- Add more metadata
- Create rust.yml
- more docs
- Add docs and fix constructed encoding for strings
- correctly handle indefinite lengths
- Added more fuzzing fixes
- Implement DER & CER
- Implement automatic tagging
- Implement tagging attribute for all valid positions
- Refactor tests, add option impl, add docs
- split up proc macro exports
- Add static asserts and time types
- Create README.md
- Fix more macro bugs and make open type complete
- fuzzing fixes
- fmt
- Implemented initial support for choice enums
- Add initial macro support for structs and enums
- Removing .vscode from git history
- Add encoder impl and tests
- Add encoder stub
- Refactor tag out of decode
- TagValue -> AsnType
- refactored parser into its own file
- Fix dependency features
- Refactor trait and error handling
- Refactor Decoder trait
- add string
- Refactored constructed encoding parsing to be generic
- Add bitstring
- add types module
- Added octet string, null, and object identifier
- Refactor error handling
- refactor tag from identifier
- Add initial ber parser
- new project
