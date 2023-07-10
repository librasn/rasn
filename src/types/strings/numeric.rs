use super::*;

/// A string which can only contain numbers or `SPACE` characters.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumericString(Vec<u8>);

impl NumericString {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidNumericString> {
        if !bytes
            .iter()
            .all(|byte| Self::CHARACTER_SET.contains(&(*byte as u32)))
        {
            Err(InvalidNumericString)
        } else {
            Ok(Self(bytes.to_owned()))
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<BitString> for NumericString {
    type Error = FromPermittedAlphabetError;

    fn try_from(string: BitString) -> Result<Self, Self::Error> {
        Self::try_from_permitted_alphabet(&string, None)
    }
}

impl TryFrom<String> for NumericString {
    type Error = InvalidNumericString;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::from_bytes(string.as_bytes())
    }
}

impl TryFrom<&'_ str> for NumericString {
    type Error = InvalidNumericString;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Self::from_bytes(string.as_bytes())
    }
}

impl TryFrom<Vec<u8>> for NumericString {
    type Error = InvalidNumericString;

    fn try_from(string: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_bytes(&string)
    }
}

impl AsnType for NumericString {
    const TAG: Tag = Tag::NUMERIC_STRING;
}

impl Encode for NumericString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder
            .encode_numeric_string(tag, constraints, self)
            .map(drop)
    }
}

impl Decode for NumericString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_numeric_string(tag, constraints)
    }
}

impl StaticPermittedAlphabet for NumericString {
    const CHARACTER_SET: &'static [u32] = &bytes_to_chars([
        b' ', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    ]);

    fn chars(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::from(self.0.iter().map(|byte| *byte as u32))
    }

    fn push_char(&mut self, ch: u32) {
        debug_assert!(
            Self::CHARACTER_SET.contains(&ch),
            "{} not in character set",
            ch
        );
        self.0.push(ch as u8);
    }
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[snafu(display("Invalid numeric string"))]
pub struct InvalidNumericString;
