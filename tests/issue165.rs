use rasn::{types::*, *};

#[test]
fn decode_implicitly_tagged_vec_choice() {
    #[derive(Debug, Clone, AsnType, Encode, Decode, PartialEq)]
    #[rasn(choice)]
    pub enum MultisigPolicySignature {
        #[rasn(tag(0))]
        Key {
            #[rasn(tag(0))]
            key: String,
        },
    }

    let transactions2 = vec![MultisigPolicySignature::Key {
        key: String::from("key1"),
    }];

    let ser2 = rasn::der::encode(&transactions2).unwrap();
    let transactions2_2: Vec<MultisigPolicySignature> = rasn::der::decode(&ser2[..]).unwrap();
    assert_eq!(transactions2_2.len(), 1);
    assert_eq!(
        transactions2_2.first().unwrap(),
        &MultisigPolicySignature::Key {
            key: String::from("key1")
        }
    );
}
