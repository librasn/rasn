use rasn::prelude::*;

// https://github.com/librasn/rasn/issues/193
#[test]
fn test_sequence_with_generics_issue_193() {
    pub trait LeetTrait {
        type Leet: Encode + Decode + core::fmt::Debug + Clone;

        fn leet(&self) -> Self::Leet;
    }

    #[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum Messages<T: LeetTrait> {
        Content(T::Leet),
        All(()),
    }
    #[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, automatic_tags)]
    pub struct Message<T: LeetTrait>(T);

    #[derive(AsnType, Encode, Decode, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct Hello(String);

    impl LeetTrait for Hello {
        type Leet = u16;

        fn leet(&self) -> Self::Leet {
            1337
        }
    }

    let hello_message: Message<Hello> = Message(Hello("Hello".to_string()));
    let hello_selection: Messages<Hello> = Messages::Content(hello_message.0.leet());

    let encoded = rasn::oer::encode::<Messages<Hello>>(&hello_selection).unwrap();
    assert_eq!(encoded, vec![0x80, 0x05, 0x39]);
    assert_eq!(
        hello_selection,
        rasn::oer::decode::<Messages<Hello>>(&encoded).unwrap()
    )
}
