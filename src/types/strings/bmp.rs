use super::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BmpString(Vec<u16>);

impl BmpString {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.iter().flat_map(|ch| ch.to_be_bytes()).collect()
    }
}

impl StaticPermittedAlphabet for BmpString {
    const CHARACTER_SET: &'static [u32] = &{
        let mut array = [0u32; 0xFFFE];
        let mut i = 0;
        while i < 0xFFFE {
            array[i as usize] = i;
            i += 1;
        }
        array
    };

    fn push_char(&mut self, ch: u32) {
        debug_assert!(
            Self::CHARACTER_SET.contains(&ch),
            "{} not in character set",
            ch
        );
        self.0.push(ch as u16);
    }

    fn chars(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::from(self.0.iter().map(|ch| *ch as u32))
    }
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[snafu(display("Invalid bmp character"))]
pub struct InvalidBmpString;

impl TryFrom<String> for BmpString {
    type Error = InvalidBmpString;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}

impl TryFrom<&'_ str> for BmpString {
    type Error = InvalidBmpString;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut vec = Vec::with_capacity(value.len());
        for ch in value.chars() {
            if !matches!(ch as u16, 0x0..=0xFFFF) {
                return Err(InvalidBmpString);
            } else {
                vec.push(ch as u16);
            }
        }

        Ok(Self(vec))
    }
}

impl AsnType for BmpString {
    const TAG: Tag = Tag::BMP_STRING;
}

impl Encode for BmpString {
    fn encode_with_tag_and_constraints<E: Encoder>(
        &self,
        encoder: &mut E,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<(), E::Error> {
        encoder.encode_bmp_string(tag, constraints, self).map(drop)
    }
}

impl Decode for BmpString {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        constraints: Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_bmp_string(tag, constraints)
    }
}
