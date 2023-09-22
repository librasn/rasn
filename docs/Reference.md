# Reference

<table>
<tr>
<td/> <td> ASN1 </td> <td> rasn </td>
</tr>
<tr>
<td>Type alias</td>
<td>

```asn
Test-type-a ::= Test-type-b
```

</td>
<td>

```rust
/// either
type TestTypeA = TestTypeB;

/// or
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub TestTypeB)
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
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub bool)
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
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(())
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
// either
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub u8 /* or any other rust integer type */)

// or
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub Integer)
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
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("8..=8"))]
struct TestTypeA(pub u8)

// or
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("8..=8"))]
struct TestTypeA(pub Integer)
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
/// of course a primitive rust integer would still work in these examples
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("-8..=360"))]
struct TestTypeA(pub Integer)

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("..=360"))]
struct TestTypeB(pub Integer)

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("42..="))]
struct TestTypeC(pub Integer)
```

</td>
</tr>
<td>Extensible value constraint</td>
<td>

```asn
Test-type-a ::= INTEGER (42,...)
Test-type-b ::= INTEGER (1..360,...)
```

</td>
<td>

```rust
/// of course a primitive rust integer would still work in these examples
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("42..=42", extensible))]
struct TestTypeA(pub Integer)

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("1..=360", extensible))]
struct TestTypeB(pub Integer)
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
#[derive(AsnType, Decode, Encode)]
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
#[derive(AsnType, Decode, Encode)]
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
/// The tagging encironment has to be declared for every rasn-annotated struct or enum
/// There is no implicit extensibility
#[derive(AsnType, Decode, Encode)]
#[rasn(enumerated, automatic_tags)]
enum TestTypeB {
    Juice,
    Wine,
    Grappa
}

#[derive(AsnType, Decode, Encode)]
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
/// The tagging encironment has to be declared for every rasn-annotated struct or enum
/// There is no implicit extensibility
#[derive(AsnType, Decode, Encode)]
#[rasn(enumerated, tag(explicit(application, 1))]
enum TestTypeB {
    #[rasn(explicit(0))]
    Juice,
    #[rasn(explicit(1))]
    Wine,
    #[rasn(explicit(2))]
    Grappa
}

#[derive(AsnType, Decode, Encode)]
#[rasn(enumerated, tag(explicit(application, 2)))]
enum TestTypeA {
    #[rasn(explicit(0))]
    Seed,
    #[rasn(explicit(1))]
    Grape,
    #[rasn(explicit(2))]
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
/// The tagging encironment has to be declared for every rasn-annotated struct or enum
/// There is no implicit extensibility
#[derive(AsnType, Decode, Encode)]
#[rasn(enumerated, tag(application, 1)]
enum TestTypeB {
    Juice = 0,
    Wine = 1,
    Grappa = 2
}

#[derive(AsnType, Decode, Encode)]
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
    seed Seed,
    grape Grape,
    raisin Raisin
}

Test-type-b ::= CHOICE { 
    juice INTEGER (0..3,...), 
    wine Zibibbo,
    ...,
    grappa [2] EXPLICIT INTEGER 
}
```

</td>
<td>

```rust
#[derive(AsnType, Decode, Encode)]
#[rasn(choice, automatic_tags)]
enum TestTypeA {
    Seed(Seed),
    Grape(Grape),
    Raisin(Raisin)
}

#[derive(AsnType, Decode, Encode)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
enum TestTypeB {
    #[rasn(value("0..3", extensible))]
    Juice(Integer),
    Wine(Zibibbo),
    #[rasn(extension_addition, tag(explicit(2)))]
    Grappa(Grappa)
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
    wine Zibibbo,
    ...,
    grappa [2] EXPLICIT INTEGER 
}
```

</td>
<td>

```rust
#[derive(AsnType, Decode, Encode)]
#[rasn(automatic_tags)]
#[non_exhaustive]
struct TestTypeA {
    #[rasn(value("0..3", extensible))]
    juice: Integer,
    wine: Zibibbo,
    #[rasn(extension_addition, tag(explicit(2)))]
    grappa: Grappa
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
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub SequenceOf<bool>)

/// Constrained inner primitive types need to be wrapped in a helper newtype
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, value("1..=1", extensible))]
struct InnerTestTypeB(pub Integer)

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeB(pub SequenceOf<InnerTestTypeB>)
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
/// the other charater types supported by rasn behave exactly the same:
/// NumericString, VisibleString, Ia5String, TeletexString, GeneralString, BmpString, PrintableString
/// (and also for BIT STRING and OCTET STRING)
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub Utf8String)
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
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub BitString)
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
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate)]
struct TestTypeA(pub OctetString)
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
/// The size constraint definition behaves similar to the value definition (see above)
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, size("42..=42", extensible))]
struct TestTypeA(pub Utf8String)

#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, size("1..=8"))]
struct TestTypeB(pub SequenceOf<bool>)
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
#[derive(AsnType, Decode, Encode)]
#[rasn(delegate, from("\u{0041}..\u{005A}"))]
struct TestTypeA(pub Utf8String)
```

</td>
</tr>
</table>