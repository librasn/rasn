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
