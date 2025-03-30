use rasn::prelude::*;
pub trait LeetTrait {
    type Leet: Encode + Decode + core::fmt::Debug + Clone;

    fn leet(&self) -> Self::Leet;
}

// https://github.com/librasn/rasn/issues/193
#[test]
fn test_sequence_with_generics_issue_193() {
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

// This test is just for checking that generics will compile
#[test]
fn test_sequence_with_generic_and_constraints() {
    #[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(automatic_tags)]
    pub struct ConstrainedBlock<T>
    where
        T: LeetTrait,
    {
        id: Integer,
        #[rasn(size("1.."))]
        extn: SequenceOf<T>,
    }
    #[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(automatic_tags)]
    #[rasn(delegate)]
    #[rasn(size("4"))]
    pub struct ConstrainedDelegateBlock<T: LeetTrait>(T);

    #[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(automatic_tags)]
    #[rasn(choice)]
    enum ConstrainedBlockEnum<T: LeetTrait> {
        First(Integer),
        #[rasn(size("1.."))]
        Second(T),
        #[rasn(value("5"))]
        Third(T),
    }
}
#[test]
fn test_multi_field_tuple_structs_with_phantom_data() {
    use core::marker::PhantomData;
    // without phantom data
    #[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(automatic_tags)]
    #[rasn(delegate)]
    pub struct TupleStruct(u8);

    let encoded = rasn::oer::encode::<TupleStruct>(&TupleStruct(1)).unwrap();

    #[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, automatic_tags)]
    pub struct TupleStructWithPhantomData<T: Ord>(u8, PhantomData<T>);
    let encoded2 = rasn::oer::encode::<TupleStructWithPhantomData<u32>>(
        &TupleStructWithPhantomData(1, PhantomData),
    )
    .unwrap();

    #[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(automatic_tags)]
    #[rasn(delegate)]
    pub struct TupleStructWithTwoPhantomData<T: Ord>(u8, PhantomData<T>, PhantomData<T>);
    let encoded3 = rasn::oer::encode::<TupleStructWithTwoPhantomData<u32>>(
        &TupleStructWithTwoPhantomData(1, PhantomData, PhantomData),
    )
    .unwrap();

    #[derive(AsnType, Debug, Encode, Decode, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(automatic_tags)]
    #[rasn(delegate)]
    pub struct TupleStructWithThreePhantomData<T: Ord>(
        u8,
        PhantomData<T>,
        PhantomData<T>,
        PhantomData<T>,
    );
    let encoded4 = rasn::oer::encode::<TupleStructWithThreePhantomData<u32>>(
        &TupleStructWithThreePhantomData(1, PhantomData, PhantomData, PhantomData),
    )
    .unwrap();
    assert_eq!(&encoded, &encoded2);
    assert_eq!(&encoded, &encoded3);
    assert_eq!(&encoded, &encoded4);
    let decoded = rasn::oer::decode::<TupleStruct>(&encoded).unwrap();
    assert_eq!(decoded, TupleStruct(1));
    let decoded2 = rasn::oer::decode::<TupleStructWithPhantomData<u32>>(&encoded2).unwrap();
    assert_eq!(decoded2, TupleStructWithPhantomData(1, PhantomData));
    let decoded3 = rasn::oer::decode::<TupleStructWithTwoPhantomData<u32>>(&encoded3).unwrap();
    assert_eq!(
        decoded3,
        TupleStructWithTwoPhantomData(1, PhantomData, PhantomData)
    );
    let decoded4 = rasn::oer::decode::<TupleStructWithThreePhantomData<u32>>(&encoded4).unwrap();
    assert_eq!(
        decoded4,
        TupleStructWithThreePhantomData(1, PhantomData, PhantomData, PhantomData)
    );
}
