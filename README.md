# Rasn
[![crates.io](https://img.shields.io/crates/d/rasn.svg)](https://crates.io/crates/rasn)
[![Help Wanted](https://img.shields.io/github/issues/XAMPPRocky/rasn/help%20wanted?color=green)](https://github.com/XAMPPRocky/rasn/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![Lines Of Code](https://tokei.rs/b1/github/XAMPPRocky/rasn?category=code)](https://github.com/XAMPPRocky/tokei)
[![Documentation](https://docs.rs/rasn/badge.svg)](https://docs.rs/rasn/)
[![Benchmarks](https://img.shields.io/badge/bencher-benchmarks-orange?link=https%3A%2F%2Fbencher.dev%2Fconsole%2Fprojects%2Frasn%2Fperf)](https://bencher.dev/perf/rasn/plots)

Welcome to `rasn` (pronounced "raisin"), a safe `#[no_std]` ASN.1 codec framework.
That enables you to safely create, share, and handle ASN.1 data types from and to different encoding rules. If you are unfamiliar with ASN.1 and encoding formats like BER/DER, I would recommend reading [*"A Warm Welcome to ASN.1 and DER"*][lenc] by Let's Encrypt as a quick introduction before continuing. In short it is an "Interface Description Language" (and data model) with a set of encoding formats (called rules) for that model. It was originally designed in the late 1980s and is used throughout the industry especially in telecommunications and cryptography.

The [`rasn` compiler][compiler] can be used to generate `rasn` bindings for ASN.1 modules.

[ghs]: https://github.com/sponsors/XAMPPRocky
[lenc]: https://letsencrypt.org/docs/a-warm-welcome-to-asn1-and-der/
[compiler]: https://github.com/librasn/compiler

## Features

### Abstract Codec Data Model
There are quite a few existing ASN.1 related Rust crates already, however they are currently specific to a single format or even a single standard, this makes it hard to share and re-use standards that are specified in ASN.1. Now with `rasn`'s abstract model you can build and share ASN.1 data types as crates that work with any encoder or decoder regardless of the underlying encoding rules, whether it's BER, CER, DER, or your own custom encoding.

### `#[no_std]` Support
Rasn is entirely `#[no_std]`, so you can share the same ASN.1 implementation on any Rust target platform that can support `alloc`.

### Rich Data Types
Rasn currently has support for nearly all of ASN.1's data types. `rasn` uses popular community libraries such as `bitvec`, `bytes`, and `chrono` for some of its data types as well as providing a couple of its own. Check out the [`types`][mod:types] module for what's currently available.

[mod:types]: https://docs.rs/rasn/latest/rasn/types/index.html

### Safe  Codecs
The encoder and decoder have been written in 100% safe Rust and fuzzed with [American Fuzzy Lop Plus Plus][bun] to ensure that the decoder correctly handles random input, and if valid that the encoder can correctly re-encode that value.

#### Supported Codecs

- Basic Encoding Rules (BER)
- Canonical Encoding Rules (CER)
- Distinguished Encoding Rules (DER)
- Aligned Packed Encoding Rules (APER)
- Unaligned Packed Encoding Rules (UPER)
- JSON Encoding Rules (JER)
- Octet Encoding Rules (OER)
- Canonical Octet Encoding Rules (COER)
- XML Encoding Rules (XER)

[bun]: https://aflplus.plus

### RFC implementations
Rasn also provides implementations for a number of IETF RFCs using the `rasn`
framework for use out of the box. These crates provide strongly typed
definitions for the necessary data types. Like `rasn` they are `#[no_std]`,
as well as being transport layer and encoding rule agnostic.

- [**CMS:** Cryptographic Message Syntax](https://docs.rs/rasn-cms)
- [**Kerberos** Authentication Framework](https://docs.rs/rasn-kerberos)
- [**LDAP:** Lightweight Directory Access Protocol](https://docs.rs/rasn-ldap)
- [**MIB-II:** Management of Information Base](https://docs.rs/rasn-mib)
- [**OCSP:** Online Certificate Status Protocol](https://docs.rs/rasn-ocsp)
- [**PKIX:** Public Key Infrastructure](https://docs.rs/rasn-pkix)
- [**SMI:** Structure of Management Information](https://docs.rs/rasn-smi)
- [**SNMP:** Simple Network Management Protocol](https://docs.rs/rasn-snmp)
- [**S/MIME:** Secure/Multipurpose Internet Mail Extensions](https://docs.rs/rasn-smime)


#### C-ITS Standards

- [**IEEE 1609.2:** IEEE Standard for Wireless Access in Vehicular Environments (WAVE) - Security Services for Application and Management Messages ](standards/its/src/ieee1609dot2/)
- [**ETSI TS 103 097:** Intelligent Transport Systems (ITS) - Security header and certificate formats](standards/its/src/ts103097/)


### Powerful Derive Macros
Easily model your structs and enums with derive equivalents of all of the traits. These macros provide a automatic implementation that ensures your model is a valid ASN.1 type at *compile-time*. To explain that though, first we have to explain…

## How It Works
The codec API has been designed for ease of use, safety, and being hard to *misuse*. The most common mistakes are around handling the length and ensuring it's correctly encoded and decoded. In `rasn` this is completely abstracted away letting you focus on the abstract model. Let's look at what decoding a simple custom `SEQUENCE` type looks like.

```asn1
Person ::= SEQUENCE {
  age INTEGER,
  name UTF8String
}
```

Which we want to map to the following equivalent Rust code.

```rust
struct Person {
    age: rasn::types::Integer,
    name: String, // or rasn::types::Utf8String
}
```


### Implementing The Traits
When modelling an ASN.1 data type, there are three traits we'll need to implement. `Decode` and `Encode` for converting to and from encoding rules, and the shared `AsnType` trait; which defines some associated data needed to be given to the encoder and decoder. Currently the only thing we have define is the tag to use to identify our type.

```rust
# struct Person;
use rasn::{AsnType, types::Tag};

impl AsnType for Person {
    // Default tag for sequences.
    const TAG: Tag = Tag::SEQUENCE;
}
```

Next is the `Decode` and `Encode` traits. These are mirrors of each other and both have one provided method (`decode`/`encode`) and one required method (`decode_with_tag`/`encode_with_tag`). Since in ASN.1 nearly every type can be implicitly tagged allowing anyone to override the tag associated with the type, having `*_with_tag` as a required method requires the implementer to correctly handle this case, and the provided methods simply calls `*_with_tag` with the type's associated `AsnType::TAG`. Let's look at what the codec implementation of `Person` looks like.

```rust
# use rasn::{AsnType, types::{Constructed, fields::{Field, Fields}}};
# struct Person { name: Utf8String, age: Integer }
# impl AsnType for Person {
#    const TAG: Tag = Tag::SEQUENCE;
#    const IDENTIFIER: Identifier = Identifier(Some("Person"));
# }
# impl Constructed<2, 0> for Person {
#     const FIELDS: Fields<2> = Fields::from_static([
#          Field::new_required(0, Utf8String::TAG, Utf8String::TAG_TREE, "age"),
#          Field::new_required(1, Integer::TAG, Integer::TAG_TREE, "name"),
#     ]);
# }
use rasn::{prelude::*, types::{Integer, Utf8String}};

impl Decode for Person {
    fn decode_with_tag_and_constraints<D: Decoder>(decoder: &mut D, tag: Tag, constraints: Constraints) -> Result<Self, D::Error> {
        // Accepts a closure that decodes the contents of the sequence.
        decoder.decode_sequence(tag, None::<fn () -> Self>, |decoder| {
            let age = Integer::decode(decoder)?;
            let name = Utf8String::decode(decoder)?;
            Ok(Self { age, name })
        })
    }
}

impl Encode for Person {
    fn encode_with_tag_and_constraints<'encoder, E: Encoder<'encoder>>(&self, encoder: &mut E, tag: Tag, constraints: Constraints, identifier: Identifier) -> Result<(), E::Error> {
        // Accepts a closure that encodes the contents of the sequence.
        encoder.encode_sequence::<2, 0, Self, _>(tag, |encoder| {
            self.age.encode(encoder)?;
            self.name.encode(encoder)?;
            Ok(())
        }, identifier)?;

        Ok(())
    }
}
```

That's it!  We've just created a new ASN.1 that can be encoded and decoded to BER, CER, and DER; and nowhere did we have to check the tag, the length, or whether the string was primitive or constructed encoded. All those nasty encoding rules details are completely abstracted away so your type only has handle how to map to and from ASN.1's data model.

With all the actual conversion code isolated to the codec implementations you can know that your model is always safe to use. The API has also been designed to prevent you from making common logic errors that can lead to invalid encoding. For example; if we look back at our `Encode` implementation, what if we forgot to use the encoder we were given in `encode_sequence` and tried to use the parent instead?

```text
error[E0501]: cannot borrow `*encoder` as mutable because previous closure requires unique access
   --> tests/derive.rs:122:9
    |
122 |           encoder.encode_sequence(tag, |sequence| {
    |           ^       ---------------      ---------- closure construction occurs here
    |           |       |
    |  _________|       first borrow later used by call
    | |
123 | |             self.age.encode(encoder)?;
    | |                             ------- first borrow occurs due to use of `encoder` in closure
124 | |             self.name.encode(sequence)?;
125 | |             Ok(())
126 | |         })?;
    | |__________^ second borrow occurs here

error[E0500]: closure requires unique access to `encoder` but it is already borrowed
   --> tests/derive.rs:122:38
    |
122 |         encoder.encode_sequence(tag, |sequence| {
    |         ------- ---------------      ^^^^^^^^^^ closure construction occurs here
    |         |       |
    |         |       first borrow later used by call
    |         borrow occurs here
123 |             self.age.encode(encoder)?;
    |                             ------- second borrow occurs due to use of `encoder` in closure

```

Our code fails to compile! Which, in this case is great, there's no chance that our contents will accidentally be encoded in the wrong sequence because we forgot to change the name of a variable. These ownership semantics also mean that an `Encoder` can't accidentally encode the contents of a sequence multiple times in their implementation.  Let's see how we can try to take this even further.

### Compile-Safe ASN.1 With Macros
So far we've shown how rasn's API takes steps to be safe and protect from accidentally creating an invalid model. However, it's often hard to cover everything in an imperative API. Something that is important to understand about ASN.1 that isn't obvious in the above examples is that; in ASN.1, all types can be identified by a tag (essentially two numbers e.g. `INTEGER`'s tag is `0, 2`). Field and variant names are not transmitted in most encoding rules, so this tag is also used to identify fields or variants in a `SEQUENCE` or `CHOICE`. This means that in a ASN.1 struct or enum every field and variant  **must have** a distinct tag for the whole type to be considered valid. For example ; If we changed `age` in `Person` to be a `String` like below it would be invalid ASN.1 even though it compiles and runs correctly, we have to either use a different type or override `age`'s tag to be distinct from `name`'s. When implementing the `AsnType` trait yourself this requirement must be checked manually, however as we'll see you generally won't need to do that.

Included with rasn is a set of derive macros that enable you to have your ASN.1 model implementation implemented declaratively. The `Encode` and `Decode` macros will essentially auto-generate the implementations we showed earlier, but the real magic is the `AsnType` derive macro. Thanks to the `static-assertations` crate and recent developments in `const fn`; the `AsnType` derive will not only generate your `AsnType` implementation, it will also generate a check that asserts that every field or variant has a distinct tag at *compile-time*. This means now if for some reason we made a change to one of the types in person, we don't have re-check that our model is still valid, the compiler takes care of that for us.

```no_compile
// Invalid
#[derive(rasn::AsnType)]
struct Person {
    age: Option<String>,
    name: Option<String>,
}
```

We'll now get the following error trying to compile the above definition.

```text
error[E0080]: evaluation of constant value failed
   --> tests/derive.rs:146:10
    |
146 | #[derive(rasn::AsnType)]
    |          ^^^^^^^^^^^^^ the evaluated program panicked at 'Person's fields is not a valid order of ASN.1 tags, ensure that your field's tags and OPTIONAL
s are correct.', tests/derive.rs:146:10
    |
    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
```

Validating your model at compile-time enables you to work on ASN.1 code without fear that you're unintentionally changing something in the background. I bet you're wondering now though, how we are supposed to have a struct with two strings for fields? The answer is thankfully pretty simple, you just add `#[rasn(tag)]` attribute to override the tags of one or more of the types. However we can actually go further, because in ASN.1 there's the concept of having `AUTOMATIC TAGS` which essentially tells your ASN.1 compiler to automatically generate distinct tags for your ASN.1 definition. Now with rasn you can do that in Rust! Applying `#[rasn(automatic_tags)]` to the container will apply the same automatic tagging transformation you'd expect from an ASN.1 compiler.

```rust
use rasn::AsnType;

// Valid
#[derive(AsnType)]
struct Person {
    #[rasn(tag(context, 0))] // or just #[rasn(tag(0))]
    age: Option<String>,
    name: Option<String>,
}

// Also valid
#[derive(AsnType)]
#[rasn(automatic_tags)]
struct Person2 {
    age: Option<String>,
    name: Option<String>,
}
```

## Reference
The following table provides a range of examples showing how to declare data types with `rasn`.

<table>
<tr>
<td></td> <td> ASN1 </td> <td> rasn </td>
</tr>
<tr>
<td>Type alias</td>
<td>

```asn
Test-type-b ::= BOOLEAN
Test-type-a ::= Test-type-b
```

</td>
<td>

```rust
// either
use rasn::prelude::*;

type TestTypeB = bool;

type TestTypeA = TestTypeB;
```
```rust
// or
use rasn::prelude::*;

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeB(pub bool);

/// or
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub TestTypeB);
```

</td>
</tr>
<tr>
<td>BOOLEAN type</td>
<td>

```asn
Test-type-a ::= BOOLEAN
```

</td>
<td>

```rust
// either
use rasn::prelude::*;

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub bool);
```
```rust
// or
use rasn::prelude::*;

type TestTypeA = bool;
```
</td>
</tr>
<tr>
<td>NULL type</td>
<td>

```asn
Test-type-a ::= NULL
```

</td>
<td>

```rust
// either
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
struct TestTypeA;
```
```rust
// or
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(());
```
```rust
// or
use rasn::prelude::*;

type TestTypeA = ();
```

</td>
</tr>
<tr>
<td>INTEGER type</td>
<td>

```asn
Test-type-a ::= INTEGER
```

</td>
<td>

```rust
use rasn::prelude::*;
// either
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub u8 /* or any other rust integer type */);
```
```rust
// or
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub Integer);
```
```rust
// or
use rasn::prelude::*;
type TestTypeA = Integer;
```
```rust
// or
use rasn::prelude::*;
type TestTypeA = u8; // or any other rust integer type
```

</td>
</tr>
<tr>
<td>Single-value contraint</td>
<td>

```asn
Test-type-a ::= INTEGER (8)
```

</td>
<td>

```rust
// either
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("8"))]
struct TestTypeA(pub u8);
```
```rust
// or
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("8"))]
struct TestTypeA(pub Integer);
```

</td>
</tr>
<tr>
<td>Value range constraint</td>
<td>

```asn
Test-type-a ::= INTEGER (-8..360)
Test-type-b ::= INTEGER (MIN..360)
Test-type-c ::= INTEGER (42..MAX)
```

</td>
<td>

```rust
use rasn::prelude::*;
/// of course a primitive rust integer would still work in these examples
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("-8..=360"))]
struct TestTypeA(pub Integer);

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("..=360"))]
struct TestTypeB(pub Integer);

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("42..="))]
struct TestTypeC(pub Integer);
```

</td>
</tr>

<tr>
<td>Extensible value constraint</td>
<td>

```asn
Test-type-a ::= INTEGER (42,...)
Test-type-b ::= INTEGER (1..360,...)
```

</td>
<td>

```rust
use rasn::prelude::*;
/// of course a primitive rust integer would still work in these examples
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("42", extensible))]
struct TestTypeA(pub Integer);

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("1..=360", extensible))]
struct TestTypeB(pub Integer);
```

</td>
</tr>

<tr>
<td>ENUMERATED type</td>
<td>

```asn
Test-type-a ::= ENUMERATED { seed, grape, raisin }
```

</td>
<td>

```rust
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode, Copy, Clone, PartialEq, Debug)]
#[rasn(enumerated, automatic_tags)] /// See below
enum TestTypeA {
    Seed,
    Grape,
    Raisin
}
```

</td>
</tr>
<tr>
<td>Extensible ENUMERATED type</td>
<td>

```asn
Test-type-a ::= ENUMERATED { seed, grape, ..., raisin }
```

</td>
<td>

```rust
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode, Copy, Clone, PartialEq, Debug)]
#[rasn(enumerated, automatic_tags)] /// See below
#[non_exhaustive]
enum TestTypeA {
    Seed,
    Grape,
    #[rasn(extension_addition)]
    Raisin
}
```

</td>
</tr>
<tr>
<td>AUTOMATIC TAGS environment</td>
<td>

```asn
TestModule DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
Test-type-a ::= ENUMERATED { seed, grape, raisin }
Test-type-b ::= ENUMERATED { juice, wine, grappa }
END
```

</td>
<td>

```rust
use rasn::prelude::*;
/// The tagging encironment has to be declared for every rasn-annotated struct or enum
/// There is no implicit extensibility
#[derive(AsnType, Decode, Encode, Copy, Clone, PartialEq, Debug)]
#[rasn(enumerated, automatic_tags)]
enum TestTypeB {
    Juice,
    Wine,
    Grappa
}

#[derive(AsnType, Decode, Encode, Copy, Clone, PartialEq, Debug)]
#[rasn(enumerated, automatic_tags)]
enum TestTypeA {
    Seed,
    Grape,
    Raisin
}
```

</td>
</tr>
<tr>
<td>EXPLICIT TAGS environment</td>
<td>

```asn
TestModule DEFINITIONS EXPLICIT TAGS ::=
BEGIN
Test-type-a ::= [APPLICATION 1] ENUMERATED { seed, grape, raisin }
Test-type-b ::= [APPLICATION 2] ENUMERATED { juice, wine, grappa }
END
```

</td>
<td>

```rust
use rasn::prelude::*;
/// The tagging encironment has to be declared for every rasn-annotated struct or enum
/// There is no implicit extensibility
#[derive(AsnType, Decode, Encode, Copy, Clone, PartialEq, Debug)]
#[rasn(enumerated, tag(explicit(application, 1)))]
enum TestTypeB {
    #[rasn(tag(explicit(0)))]
    Juice,
    #[rasn(tag(explicit(1)))]
    Wine,
    #[rasn(tag(explicit(2)))]
    Grappa
}

#[derive(AsnType, Decode, Encode, Copy, Clone, PartialEq, Debug)]
#[rasn(enumerated, tag(explicit(application, 2)))]
enum TestTypeA {
    #[rasn(tag(explicit(0)))]
    Seed,
    #[rasn(tag(explicit(1)))]
    Grape,
    #[rasn(tag(explicit(2)))]
    Raisin
}
```

</td>
</tr>
<tr>
<td>IMPLICIT TAGS environment</td>
<td>

```asn
TestModule DEFINITIONS IMPLICIT TAGS ::=
BEGIN
Test-type-a ::= [APPLICATION 1] ENUMERATED { seed, grape, raisin }
Test-type-b ::= [APPLICATION 2] ENUMERATED { juice, wine, grappa }
END
```

</td>
<td>

```rust
use rasn::prelude::*;
/// The tagging encironment has to be declared for every rasn-annotated struct or enum
/// There is no implicit extensibility
#[derive(AsnType, Decode, Encode, Copy, Clone, PartialEq, Debug)]
#[rasn(enumerated, tag(application, 1))]
enum TestTypeB {
    Juice = 0,
    Wine = 1,
    Grappa = 2
}

#[derive(AsnType, Decode, Encode, Copy, Clone, PartialEq, Debug)]
#[rasn(enumerated, tag(application, 2))]
enum TestTypeA {
    Seed = 0,
    Grape = 1,
    Raisin = 2
}
```

</td>
</tr>
<tr>
<td>CHOICE type</td>
<td>

```asn
Test-type-a ::= CHOICE {
    seed BOOLEAN,
    grape BIT STRING SIZE(1,...),
    raisin OCTET STRING
}

Test-type-b ::= CHOICE {
    juice INTEGER (0..3,...),
    wine OCTET STRING,
    ...,
    grappa INTEGER
}
```

</td>
<td>

```rust
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
#[rasn(choice, automatic_tags)]
enum TestTypeA {
    Seed(bool),
    #[rasn(size("1", extensible))]
    Grape(BitString),
    Raisin(OctetString)
}

#[derive(AsnType, Decode, Encode)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
enum TestTypeB {
    #[rasn(value("0..3", extensible))]
    Juice(Integer),
    Wine(OctetString),
    #[rasn(extension_addition)]
    Grappa(Integer)
}

```

</td>
</tr>
<tr>
<td>SEQUENCE type</td>
<td>

```asn
Test-type-a ::= SEQUENCE {
    juice INTEGER (0..3,...),
    wine OCTET STRING,
    ...,
    grappa INTEGER OPTIONAL,
    water BIT STRING (SIZE(1)) OPTIONAL
}
```

</td>
<td>

```rust
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct TestTypeA {
    #[rasn(value("0..3", extensible))]
    juice: Integer,
    wine: OctetString,
    #[rasn(extension_addition)]
    grappa: Option<Integer>,
    #[rasn(extension_addition, size("1"))]
    water: Option<BitString>
}

```

</td>
</tr>
<tr>
<td>SET type</td>
<td>

```asn
Test-type-a ::= SET {
    seed NULL,
    grape BOOLEAN,
    raisin INTEGER
}
```

</td>
<td>

```rust
use rasn::prelude::*;
/// the SET declaration is basically identical to a SEQUENCE declaration,
/// except for the `set` annotation
#[derive(AsnType, Decode, Encode)]
#[rasn(set, automatic_tags)]
struct TestTypeA {
    seed: (),
    grape: bool,
    raisin: Integer
}

```

</td>
</tr>
<tr>
<td>Renaming fields</td>
<td>

```asn
Test-type-a ::= SEQUENCE {
    notQuiteRustCase INTEGER
}
```

</td>
<td>

```rust
use rasn::prelude::*;

#[derive(AsnType, Decode, Encode)]
#[rasn(automatic_tags, identifier = "Test-type-a")]
struct TestTypeA {
    #[rasn(identifier = "notQuiteRustCase")]
    rust_case_indeed: Integer
}

```

</td>
</tr>
<tr>
<td>OPTIONAL and DEFAULT fields</td>
<td>

```asn
Test-type-a ::= SEQUENCE {
    seed BOOLEAN DEFAULT TRUE,
    grape INTEGER OPTIONAL,
    raisin INTEGER DEFAULT 1
}
```

</td>
<td>

```rust
use rasn::prelude::*;
/// DEFAULTs are provided via linked helper functions
#[derive(AsnType, Decode, Encode)]
#[rasn(automatic_tags)]
struct TestTypeA {
    #[rasn(default = "default_seed")]
    seed: bool,
    grape: Option<Integer>,
    #[rasn(default = "default_raisin")]
    raisin: Integer
}

fn default_seed() -> bool {
    true
}

fn default_raisin() -> Integer {
    1.into()
}
```

</td>
</tr>
<tr>
<td>SEQUENCE OF type</td>
<td>

```asn
Test-type-a ::= SEQUENCE OF BOOLEAN
Test-type-b ::= SEQUENCE OF INTEGER(1,...)
```

</td>
<td>

```rust
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub SequenceOf<bool>);

/// Constrained inner primitive types need to be wrapped in a helper newtype
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("1", extensible))]
struct InnerTestTypeB(pub Integer);

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeB(pub SequenceOf<InnerTestTypeB>);
```

</td>
</tr>
<tr>
<td>Character string types</td>
<td>

```asn
Test-type-a ::= UTF8String
```

</td>
<td>

```rust
use rasn::prelude::*;
/// the other charater types supported by rasn behave exactly the same:
/// NumericString, VisibleString, Ia5String, TeletexString, GeneralString, BmpString, PrintableString
/// (and also for BIT STRING and OCTET STRING)
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub Utf8String);
```

</td>
</tr>
<tr>
<td>BIT STRING type</td>
<td>

```asn
Test-type-a ::= BIT STRING
```

</td>
<td>

```rust
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub BitString);
```

</td>
</tr>
<tr>
<td>OCTET STRING type</td>
<td>

```asn
Test-type-a ::= OCTET STRING
```

</td>
<td>

```rust
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub OctetString);
```

</td>
</tr>
<tr>
<td>Size contraint</td>
<td>

```asn
Test-type-a ::= UTF8String (SIZE (42,...))
Test-type-b ::= SEQUENCE (SIZE (1..8)) OF BOOLEAN
```

</td>
<td>

```rust
use rasn::prelude::*;
/// The size constraint definition behaves similar to the value definition (see above)
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, size("42", extensible))]
struct TestTypeA(pub Utf8String);

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, size("1..=8"))]
struct TestTypeB(pub SequenceOf<bool>);
```

</td>
</tr>
<tr>
<td>Permitted alphabet contraint</td>
<td>

```asn
Test-type-a ::= UTF8String (FROM ("A".."Z"))
```

</td>
<td>

```rust
use rasn::prelude::*;
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, from("\u{0041}..\u{005A}"))]
struct TestTypeA(pub Utf8String);
```

</td>
</tr>
</table>

## Sponsorship
This project was funded through the [NGI Assure Fund](https://nlnet.nl/project/RASN), a fund established by NLnet with financial support from the European Commission's Next Generation Internet programme, under the aegis of DG Communications Networks, Content and Technology under grant agreement No 957073.

Octet Encoding Rules (OER/COER) were added as part of the [RE-ROUTE project](https://reroute-project.eu/), which received funding from the European Union’s Horizon Europe Marie Skłodowska-Curie Actions (MSCA), Staff Exchanges under grant agreement No 101086343.

## Disclaimer
The software is provided "as is" and the authors disclaim all warranties with regard to this software including all implied warranties of merchant-ability and fitness. In no event shall the authors be liable for any special, direct, indirect, or consequential damages or any damages whatsoever resulting from loss of use, data or profits, whether in an action of contract, negligence or other tortuous action, arising out of or in connection with the use or performance of this software.
