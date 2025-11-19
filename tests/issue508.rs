use pretty_assertions::assert_eq;
use rasn::prelude::*;
use serde::{Deserialize, Serialize};

#[test]
fn enum_struct_variant() {
    #[derive(AsnType, Encode, Decode, Serialize, Deserialize)]
    #[rasn(choice)]
    enum StructVariantEnum {
        StructVariant {
            #[serde(rename = "numeric")]
            field1: u8,
            #[serde(rename = "text")]
            field2: String,
        },
    }
    let value = StructVariantEnum::StructVariant {
        field1: 1,
        field2: "foo bar baz".to_string(),
    };
    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        r#"{"StructVariant":{"numeric":1,"text":"foo bar baz"}}"#
    )
}

#[test]
fn asn_set() {
    #[derive(AsnType, Encode, Decode, Serialize, Deserialize)]
    #[rasn(set)]
    struct AsnSet {
        #[rasn(tag(0))]
        #[serde(rename = "host")]
        field1: String,
        #[rasn(tag(1))]
        #[serde(rename = "port")]
        field2: u16,
    }
    let value = AsnSet {
        field1: "localhost".to_string(),
        field2: 12345,
    };
    assert_eq!(
        serde_json::to_string(&value).unwrap(),
        r#"{"host":"localhost","port":12345}"#
    )
}
