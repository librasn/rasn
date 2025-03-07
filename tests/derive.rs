#![allow(clippy::disallowed_names)]

use rasn::{types::*, *};

#[test]
fn enumerated() {
    #[derive(AsnType, Clone, Copy, Debug, Encode, Decode, PartialEq)]
    #[rasn(enumerated)]
    enum Foo {
        Ein,
        Zwei,
        Drei,
    }

    let ein = Foo::Ein;
    let zwei = Foo::Zwei;
    let drei = Foo::Drei;

    assert_eq!(ein, ber::decode(&ber::encode(&ein).unwrap()).unwrap());
    assert_eq!(zwei, ber::decode(&ber::encode(&zwei).unwrap()).unwrap());
    assert_eq!(drei, ber::decode(&ber::encode(&drei).unwrap()).unwrap());
}

#[test]
fn choice() {
    #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
    #[rasn(choice)]
    enum Choice {
        Bar(bool),
        #[rasn(tag(1))]
        Baz(OctetString),
        #[rasn(tag(2))]
        Foo(OctetString),
    }

    let bar = Choice::Bar(true);
    let baz = Choice::Baz(OctetString::from(vec![1, 2, 3, 4, 5]));
    let foo = Choice::Foo(OctetString::from(vec![1, 2, 3, 4, 5]));

    assert_eq!(foo, ber::decode(&ber::encode(&foo).unwrap()).unwrap());
    assert_eq!(bar, ber::decode(&ber::encode(&bar).unwrap()).unwrap());
    assert_eq!(baz, ber::decode(&ber::encode(&baz).unwrap()).unwrap());

    #[derive(AsnType, Clone, Debug, Encode, Decode, PartialEq)]
    struct ChoiceField {
        choice: VecChoice,
    }

    #[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
    #[rasn(choice)]
    enum VecChoice {
        #[rasn(tag(1))]
        Bar(Vec<bool>),
        #[rasn(tag(2))]
        Foo(Vec<OctetString>),
    }
    let bar = ChoiceField {
        choice: VecChoice::Bar(vec![true]),
    };
    let foo = ChoiceField {
        choice: VecChoice::Foo(vec![OctetString::from(vec![1, 2, 3, 4, 5])]),
    };
    assert_eq!(foo, ber::decode(&ber::encode(&foo).unwrap()).unwrap());
    assert_eq!(bar, ber::decode(&ber::encode(&bar).unwrap()).unwrap());
}

#[test]
fn sequence() {
    #[derive(AsnType, Debug, Default, Decode, Encode, PartialEq)]
    struct Bools {
        a: bool,
        #[rasn(tag(0))]
        b: bool,
    }

    let raw = &[
        0x30, // Sequence tag
        0x6,  // Length
        1, 1, 0xff, // A
        0x80, 1, 0, // B
    ][..];

    let default = Bools { a: true, b: false };

    assert_eq!(default, ber::decode(raw).unwrap());
    assert_eq!(raw, &*ber::encode(&default).unwrap());
}

#[derive(AsnType, Debug, Decode, Encode, PartialEq)]
#[rasn(choice)]
enum NestedAnonChoiceStruct {
    Foo {
        x: bool,
        #[rasn(tag(0))]
        y: bool,
    },

    #[rasn(tag(0))]
    Bar { x: bool, y: Integer },
}

#[test]
fn automatic_tags() {
    #[derive(AsnType, Debug, Decode, Default, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    struct Bools {
        #[rasn(default)]
        a: bool,
        #[rasn(default)]
        b: bool,
    }

    let raw = &[
        0x30, // Sequence tag
        0x6,  // Length
        0x80, 1, 0xFF, // A
        0x81, 1, 0, // B
    ][..];

    let default = Bools { a: true, b: false };
    assert_eq!(default, ber::decode(raw).unwrap());
}

#[test]
fn list_in_single_attr() {
    #[derive(AsnType, Debug, Default, Decode, Encode, PartialEq)]
    #[rasn(delegate, tag(context, 0))]
    struct Foo(u8);

    #[derive(AsnType, Debug, Default, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct Bar(pub u8);

    assert_eq!(Foo::TAG, Tag::new(Class::Context, 0));
    assert_eq!(Bar::TAG, Integer::TAG);
}

#[derive(AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum ExplicitChoice {
    #[rasn(tag(explicit(1)))]
    ByName,
    #[rasn(tag(explicit(2)))]
    ByKey,
}

#[derive(AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum GenericEnum<T: Clone, const N: usize> {
    FixedString(FixedOctetString<N>),
    Sequence(Vec<T>),
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BasicConstraints {
    #[rasn(default)]
    pub ca: bool,
    pub path_len_constraint: Option<Integer>,
}

// This test makes sure that Newtype3(Newtype2(Newtype1(T))) results in serializing
// as T when using the #[rasn(delegate)] attribute when T is a non-universal type.
#[test]
fn delegated_newtype_wrapping() {
    #[derive(AsnType, Debug, Decode, Encode, PartialEq)]
    #[rasn(choice)]
    enum Hash {
        #[rasn(tag(explicit(0)))]
        Sha256(String),
    }

    #[derive(AsnType, Debug, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    struct TransactionID(Hash);

    #[derive(AsnType, Debug, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    struct PolicyID(TransactionID);

    let policy_id1 = PolicyID(TransactionID(Hash::Sha256("abcdef".into())));

    let ser = rasn::der::encode(&policy_id1).unwrap();
    assert_eq!(&ser[..], &[160, 8, 12, 6, 97, 98, 99, 100, 101, 102]);
    let policy_id2: PolicyID = rasn::der::decode(&ser[..]).unwrap();
    assert_eq!(policy_id1, policy_id2);
}

// This test will fail to compile if `Result` is used in the derive/proc macros instead of
// `core::result::Result`
#[test]
fn result_scoping() {
    enum Error {}
    type Result<T> = core::result::Result<T, Error>;

    #[derive(rasn::AsnType, rasn::Encode, rasn::Decode)]
    #[rasn(choice)]
    enum Choose {
        Single(String),
    }

    let _: Result<()> = Ok(());
}

// This makes sure enum fields can have a field named `encoder` or `tag`
#[test]
fn enum_with_encoder_tag_name_variants() {
    #[derive(AsnType, Encode, Decode)]
    #[rasn(choice)]
    enum MyEnum {
        #[rasn(tag(explicit(0)))]
        HasConflictingFields {
            #[rasn(tag(explicit(0)))]
            encoder: String,
            #[rasn(tag(explicit(1)))]
            tag: String,
        },
    }
}

#[test]
fn explicit_identifiers() {
    #[derive(AsnType, Encode, Decode)]
    #[rasn(choice, identifier = "my-choice")]
    enum MyChoice {
        #[rasn(identifier = "has-alt-ident")]
        HasAltIdent(()),
    }

    #[derive(AsnType, Encode, Decode, Debug, PartialEq, Clone, Copy)]
    #[rasn(enumerated, identifier = "my-enum")]
    enum MyEnum {
        #[rasn(identifier = "has-alt-ident")]
        HasAltIdent,
    }

    #[derive(AsnType, Encode, Decode)]
    #[rasn(identifier = "my-struct")]
    struct MyStruct {
        #[rasn(identifier = "has-alt-ident")]
        has_alt_ident: (),
    }

    #[derive(AsnType, Encode, Decode)]
    #[rasn(identifier = "my-delegate")]
    struct MyDelegate(());

    assert_eq!(MyEnum::IDENTIFIER, Identifier(Some("my-enum")));
    assert_eq!(MyEnum::IDENTIFIERS, ["has-alt-ident"]);
    assert_eq!(MyChoice::IDENTIFIER, Identifier(Some("my-choice")));
    assert_eq!(MyChoice::IDENTIFIERS, ["has-alt-ident"]);
    assert_eq!(MyStruct::IDENTIFIER, Identifier(Some("my-struct")));
    assert_eq!(
        MyStruct::FIELDS.identifiers().collect::<Vec<_>>(),
        vec!["has-alt-ident"]
    );
    assert_eq!(MyDelegate::IDENTIFIER, Identifier(Some("my-delegate")));
}
