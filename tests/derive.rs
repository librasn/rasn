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

    assert_eq!(default, ber::decode(&raw).unwrap());
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
    #[derive(AsnType, Debug, Default, Decode, Encode, PartialEq)]
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
    assert_eq!(default, ber::decode(&raw).unwrap());
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
