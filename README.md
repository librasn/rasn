# Rasn
[![crates.io](https://img.shields.io/crates/d/rasn.svg)](https://crates.io/crates/rasn)
[![Help Wanted](https://img.shields.io/github/issues/XAMPPRocky/rasn/help%20wanted?color=green)](https://github.com/XAMPPRocky/rasn/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![Lines Of Code](https://tokei.rs/b1/github/XAMPPRocky/rasn?category=code)](https://github.com/XAMPPRocky/tokei)
[![Documentation](https://docs.rs/rasn/badge.svg)](https://docs.rs/rasn/)

Welcome to the `rasn` (pronounced "raisin"), a safe `#[no_std]` ASN.1 codec framework.
That enables you to safely create, share, and handle ASN.1 data types from and to different encoding rules. If you are unfamiliar with ASN.1 and encoding formats like BER/DER, I would recommend reading [*"A Warm Welcome to ASN.1 and DER"*][lenc] by Let's Encrypt as a quick introduction before continuing. In short it is an Interface Description Language (and data model) with a set of encoding formats (called rules) for that model. It was originally designed in the late 1980s and is used throughout the industry especially in telecommunications and cryptography.

[ghs]: https://github.com/sponsors/XAMPPRocky
[lenc]: https://letsencrypt.org/docs/a-warm-welcome-to-asn1-and-der/

## Features

### Abstract Codec Data Model
There are quite a few existing ASN.1 related Rust crates already, however they are currently specific to a single format or even a single standard, this makes it hard to share and re-use standards that are specified in ASN.1. Now with `rasn`'s abstract model you can build and share ASN.1 data types as crates that work with any encoder or decoder regardless of the underlying encoding rules, whether it's BER, CER, DER, or your own custom encoding.

### `#[no_std]` Support
Rasn is entirely `#[no_std]`, so you can handle and share the same ASN.1 code with a wide variety of platforms and devices.

### Rich Data Types
Rasn currently has support for nearly all of ASN.1's data types. `rasn` uses popular community libraries such as `bitvec`, `bytes`, and `chrono` for some of its data types as well as providing a couple of its own. Check out the [`types`][mod:types] module for what's currently available.

[mod:types]: http://docs.rs/rasn/0.2.0/rasn/types/index.html

### Safe BER, CER, and DER Codecs
Included with the framework is a implementation of the X.690 standard also known as the Basic Encoding Rules, Canonical Encoding Rules, and Distinguished Encoding Rules codecs. The encoder and decoder have been written in 100% safe Rust and fuzzed with [American Fuzzy Lop][bun] to ensure that the decoder correctly handles random input, and if valid that the encoder can correctly re-encode that value.

[bun]: https://lcamtuf.coredump.cx/afl/

### RFC implementations
Rasn also provides implementations for a number of IETF RFCs using the `rasn`
framework for use out of the box. These crates provide strongly typed
definitions for the necessary data types. Like `rasn` they are `#[no_std]`,
transport layer, and encoding rule agnostic.

- [SMI](https://docs.rs/rasn-smi)
- [SNMP](https://docs.rs/rasn-snmp)
- [MIB-II](https://docs.rs/rasn-mib)


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
use rasn::{AsnType, Tag};

impl AsnType for Person {
    // Default tag for sequences.
    const TAG: Tag = Tag::SEQUENCE;
}
```

Next is the `Decode` and `Encode` traits. These are mirrors of each other and both have one provided method (`decode`/`encode`) and one required method (`decode_with_tag`/`encode_with_tag`). Since in ASN.1 nearly every type can be implicitly tagged allowing anyone to override the tag associated with the type, having `*_with_tag` as a required method requires the implementer to correctly handle this case, and the provided methods simply calls `*_with_tag` with the type's associated `AsnType::TAG`. Let's look at what the codec implementation of `Person` looks like.

```rust
# struct Person { name: Utf8String, age: Integer }
# impl rasn::AsnType for Person { const TAG: Tag = Tag::SEQUENCE; }
use rasn::{Decode, Decoder, Encode, Encoder, Tag, types::{Integer, Utf8String}};

impl Decode for Person {
    fn decode_with_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
        // Accepts a closure that decodes the contents of the sequence.
        decoder.decode_sequence(tag, |decoder| {
            let age = Integer::decode(decoder)?;
            let name = Utf8String::decode(decoder)?;
            Ok(Self { age, name })
        })
    }
}

impl Encode for Person {
    fn encode_with_tag<E: Encoder>(&self, encoder: &mut E, tag: Tag) -> Result<(), E::Error> {
        // Accepts a closure that encodes the contents of the sequence.
        encoder.encode_sequence(tag, |encoder| {
            self.age.encode(encoder)?;
            self.name.encode(encoder)?;
            Ok(())
        })?;

        Ok(())
    }
}
```

That's it!  We've just created a new ASN.1 that be encoded and decoded to BER, CER, and DER; and nowhere did we have to check the tag, the length, or whether the string was primitive or constructed encoded. All those nasty encoding rules details are completely abstracted away so your type only has handle how to map to and from ASN.1's data model.

With all the actual conversion code isolated to the codec implementations you can know that your model is always safe to use. The API has also been designed to prevent you from making common logic errors that can lead to invalid encoding. For example; if we look back our `Encode` implementation, and what if we forgot to use the encoder we were given in `encode_sequence` and tired to use the parent instead?

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
So far we've shown how rasn's API takes steps to be safe and protect from accidentally creating an invalid model. However, it's often hard to cover everything in an imperative API. Something that is important to understand about ASN.1 that isn't obvious in the above examples is that; in ASN.1, all types can be identified by a tag (essentially two numbers e.g. `INTEGER`'s tag is `0, 2`). Field and variant names are not transmitted in most encoding rules, so this tag is also used to identify fields or variants in a `SEQUENCE` or `CHOICE`. This means that every that in a ASN.1 struct or enum every field and variant  **must have** a distinct tag for the whole type to be considered valid. For example ; If we changed `age` in `Person` to be a `String` like below it would be invalid ASN.1 even though it compiles and runs correctly, we have to either use a different type or override `age`'s tag to be distinct from `name`'s. When implementing the `AsnType` trait yourself this requirement must checked by manually, however as we'll see you generally won't need to do that.

Included with rasn is a set of derive macros that enable you to have your ASN.1 model implementation implemented declaratively. The `Encode` and `Decode` macros will essentially auto-generate the implementations we showed earlier, but the real magic is the `AsnType` derive macro. Thanks to the `static-assertations` crate and recent developments in `const fn`; the `AsnType` derive will not only generate your `AsnType` implementation, it will also generate a check that asserts that every field or variant has a distinct tag at *compile-time*. This means now if for some reason we made a change to one of the types in person, we don't have re-check that our model is still valid, the compiler takes care of that for us.

```rust,no_compile
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
  --> tests/derive.rs:80:14
   |
80 |     #[derive(AsnType, Debug, Default, Decode, Encode, PartialEq)]
   |              ^^^^^^^ attempt to compute `0_usize - 1_usize` which would overflow
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
```

While not the most obvious error message at the moment, validating your model at compile-time enables you to work on ASN.1 code without fear that you're unintentionally changing something in the background. I bet you're wondering now though, how we are supposed to have a struct with two strings for fields? The answer is thankfully pretty simple, you just add `#[rasn(tag)]` attribute to override the tags of one or more of the types. However we can actually go further, because in ASN.1 there's the concept of having `AUTOMATIC TAGS` which essentially tells your ASN.1 compiler to automatically generate distinct tags for your ASN.1 definition. Now with rasn you can do that in Rust! Applying `#[rasn(automatic_tags)]` to the container  automatically generate tags will apply the same automatic tagging transformation you'd expect from an ASN.1 compiler.

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

### What's Missing & What's Next?
While rasn is starting out relatively full featured, there are still plenty of missing features and types. The main limitation that exists currently, is the lack of support for constraints. In ASN.1 you can arbitrarily put constraints on types (e.g. array always of length 4, or a number between 1–10). There are encoding rules that take advantage of these constraints to save space, and you can't currently you can't communicate those constraints to a codec in a safe abstract fashion. So this means for example you wouldn't be able to implement a Packed Encoding Rules (PER) codec that handles constraints correctly. I want to eventually support constraints and PER directly, but it still requires a lot of design work to be thought through first.

## Disclaimer
The software is provided "as is" and the authors disclaim all warranties with regard to this software including all implied warranties of merchant-ability and fitness. In no event shall the authors be liable for any special, direct, indirect, or consequential damages or any damages whatsoever resulting from loss of use, data or profits, whether in an action of contract, negligence or other tortuous action, arising out of or in connection with the use or performance of this software.
