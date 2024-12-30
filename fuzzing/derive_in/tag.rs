#[rasn(set, tag(application, 0))]
struct Foo {
    #[rasn(tag(explicit(444)))]
    a: Integer,
    #[rasn(tag(explicit(5)))]
    b: Integer,
    #[rasn(tag(application, 9))]
    c: Integer,
}
