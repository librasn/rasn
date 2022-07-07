mod error;

use alloc::vec::Vec;

use bitvec::prelude::*;
use snafu::*;

use super::{FOURTY_EIGHT_K, SIXTEEN_K, SIXTY_FOUR_K, THIRTY_TWO_K};
use crate::{
    types::{self, constraints, BitString, Constraints, Tag},
    Encode, Encoder as _,
};

pub use error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Clone, Copy, Default)]
pub struct EncoderOptions {
    #[allow(unused)]
    aligned: bool,
    set_encoding: bool,
}

impl EncoderOptions {
    pub fn aligned() -> Self {
        Self {
            aligned: true,
            ..<_>::default()
        }
    }

    pub fn unaligned() -> Self {
        Self {
            aligned: false,
            ..<_>::default()
        }
    }

    fn without_set_encoding(mut self) -> Self {
        self.set_encoding = false;
        self
    }
}

pub struct Encoder {
    options: EncoderOptions,
    output: BitString,
    set_output: alloc::collections::BTreeMap<Tag, BitString>,
    field_bitfield: BitString,
}

impl Encoder {
    pub fn new(options: EncoderOptions) -> Self {
        Self {
            options,
            output: <_>::default(),
            set_output: <_>::default(),
            field_bitfield: <_>::default(),
        }
    }

    fn new_set_encoder(&self) -> Self {
        let mut options = self.options;
        options.set_encoding = true;
        Self::new(options)
    }

    pub fn output(self) -> Vec<u8> {
        self.bitstring_output().into_vec()
    }

    pub fn bitstring_output(self) -> BitString {
        let mut output = self
            .options
            .set_encoding
            .then(|| self.set_output.values().flatten().collect::<BitString>())
            .unwrap_or(self.output);

        (output.len() < 8).then(|| output.resize(8, false));

        output
    }

    fn pad_to_alignment(&self, buffer: &mut BitString) {
        const BYTE_WIDTH: usize = 8;
        if self.options.aligned && buffer.len() % BYTE_WIDTH != 0 {
            buffer.extend(BitString::repeat(false, BYTE_WIDTH - (buffer.len() % 8)));
            debug_assert_eq!(0, buffer.len() % 8);
        }
    }

    fn encode_constructed(&mut self, tag: Tag, extensible: Option<bool>, encoder: Self) {
        let mut buffer = BitString::default();

        extensible.map(|extended_value| buffer.push(extended_value));
        buffer.extend(&encoder.field_bitfield);
        self.pad_to_alignment(&mut buffer);
        buffer.extend(encoder.bitstring_output());
        self.extend(tag, &buffer);
    }

    fn encode_length(
        &mut self,
        buffer: &mut BitString,
        length: usize,
        constraints: constraints::Range<usize>,
        encode_fn: impl Fn(core::ops::Range<usize>) -> Result<BitString>,
    ) -> Result<()> {
        Error::check_length(length, constraints)?;

        let range = constraints.range().map_or(usize::MAX, |range| range + 1);
        let effective_length = constraints.effective_value(length).into_inner();
        let encode_min = |buffer: &mut BitString| -> Result<()> {
            if let Some(min) = constraints.as_start() {
                buffer.extend((encode_fn)(0..*min)?);
            }

            Ok(())
        };

        if range == 0 {
            (encode_min)(buffer)?;
        } else if range < SIXTY_FOUR_K {
            self.encode_integer(
                Tag::INTEGER,
                (&[constraints::Value::from(constraints).into()]).into(),
                &effective_length.into(),
            )?;
            buffer.extend((encode_fn)(0..length)?);
        } else if effective_length <= 127 {
            buffer.extend((effective_length as u8).to_be_bytes());
            self.pad_to_alignment(buffer);
            buffer.extend((encode_fn)(0..length)?);
        } else if effective_length < SIXTEEN_K {
            const SIXTEENTH_BIT: u16 = 0x8000;
            buffer.extend((SIXTEENTH_BIT | effective_length as u16).to_be_bytes());
            buffer.extend((encode_fn)(0..length)?);
        } else {
            let mut effective_length = effective_length;
            let mut min = constraints.start();
            (encode_min)(buffer)?;

            loop {
                // Hack to get around no exclusive syntax.
                const K64: usize = SIXTY_FOUR_K - 1;
                const K48: usize = FOURTY_EIGHT_K - 1;
                const K32: usize = THIRTY_TWO_K - 1;
                let (fragment_index, amount) = match effective_length {
                    SIXTY_FOUR_K..=usize::MAX => (4, SIXTY_FOUR_K),
                    FOURTY_EIGHT_K..=K64 => (3, FOURTY_EIGHT_K),
                    THIRTY_TWO_K..=K48 => (2, THIRTY_TWO_K),
                    SIXTEEN_K..=K32 => (1, SIXTEEN_K),
                    _ => {
                        self.encode_length(buffer, effective_length, <_>::default(), encode_fn)?;
                        break;
                    }
                };

                const FRAGMENT_MARKER: u8 = 0xC0;
                buffer.extend(&[FRAGMENT_MARKER | fragment_index]);

                buffer.extend((encode_fn)(min..min + amount)?);
                min += amount;
                effective_length = effective_length.saturating_sub(amount);
            }
        }

        Ok(())
    }

    fn extend<'input>(&mut self, tag: Tag, input: impl Into<Input<'input>>) {
        use bitvec::field::BitField;
        let mut set_buffer = <_>::default();
        let buffer = if self.options.set_encoding {
            &mut set_buffer
        } else {
            &mut self.output
        };

        match input.into() {
            Input::Bits(bits) => {
                buffer.extend_from_bitslice(bits);
            }
            Input::Bit(bit) => {
                buffer.push(bit);
            }
            Input::Byte(byte) => {
                buffer.store_be(byte);
            }
            Input::Bytes(bytes) => {
                buffer.extend(bytes);
            }
        }

        if self.options.set_encoding {
            self.set_output.insert(tag, set_buffer);
        }
    }
}

pub enum Input<'input> {
    Bit(bool),
    Byte(u8),
    Bits(&'input BitString),
    Bytes(&'input [u8]),
}

impl<'input> From<&'input BitString> for Input<'input> {
    fn from(value: &'input BitString) -> Self {
        Self::Bits(value)
    }
}

impl<'input> From<&'input [u8]> for Input<'input> {
    fn from(value: &'input [u8]) -> Self {
        Self::Bytes(value)
    }
}

impl<'input> From<&'input Vec<u8>> for Input<'input> {
    fn from(value: &'input Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl<'input> From<bool> for Input<'input> {
    fn from(value: bool) -> Self {
        Self::Bit(value)
    }
}

impl<'input> From<u8> for Input<'input> {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

impl crate::Encoder for Encoder {
    type Ok = ();
    type Error = Error;

    fn encode_any(&mut self, value: &types::Any) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(Tag::EOC, <_>::default(), &value.contents)
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &BitString,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::default();

        let extensible_is_present = constraints
            .extensible()
            .map(|is_present| {
                buffer.push(is_present);
                is_present
            })
            .unwrap_or_default();

        if constraints.size().effective_value(value.len()).into_inner() == 0 {
            // NO-OP
        } else if extensible_is_present {
            self.encode_length(&mut buffer, value.len(), <_>::default(), |range| {
                Ok(BitString::from(&value[range]))
            })?;
        } else {
            self.encode_length(&mut buffer, value.len(), constraints.size(), |range| {
                Ok(BitString::from(&value[range]))
            })?;
        }

        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_bool(&mut self, tag: Tag, value: bool) -> Result<Self::Ok, Self::Error> {
        self.extend(tag, value);
        Ok(())
    }

    fn encode_enumerated(
        &mut self,
        tag: Tag,
        variance: usize,
        value: isize,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_integer(
            tag,
            Constraints::from(&[constraints::Range::up_to(variance).into()]),
            &value.into(),
        )
    }

    fn encode_integer(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::Integer,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::default();
        let value_range = constraints.value();
        let mut bytes = match value_range.effective_value(value.clone()) {
            either::Left(offset) => offset.to_biguint().unwrap().to_bytes_be(),
            either::Right(value) => value.to_signed_bytes_be(),
        };

        constraints
            .extensible()
            .map(|is_present| buffer.push(is_present));

        if let Some(range) = value_range
            .range()
            .filter(|range| *range < SIXTY_FOUR_K.into())
        {
            let total_bits = value_range.end().unwrap().bits();
            let current_bits = (bytes.len() * 8) as u64;
            if total_bits > current_bits {
                let diff = total_bits - current_bits;
                let padding = (diff / 8) + ((diff % 8) != 0) as u64;
                let mut padding_vec = vec![0; padding as usize];

                padding_vec.append(&mut bytes);
                bytes = padding_vec;
            }

            let mut field = BitString::repeat(false, range.bits() as usize);
            field |= BitString::from_slice(&bytes);

            buffer.extend(field);
        } else {
            self.encode_length(&mut buffer, bytes.len(), constraints.size(), |range| {
                Ok(BitString::from_slice(&bytes[range]))
            })?;
        }

        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_null(&mut self, _: Tag) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn encode_object_identifier(&mut self, tag: Tag, oid: &[u32]) -> Result<Self::Ok, Self::Error> {
        let der = crate::der::encode_scope(|encoder| encoder.encode_object_identifier(tag, oid))
            .context(error::DerSnafu)?;
        self.encode_octet_string(tag, <_>::default(), &der)
    }

    fn encode_octet_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &[u8],
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::default();

        let extensible_is_present = constraints
            .extensible()
            .map(|is_present| {
                buffer.push(is_present);
                is_present
            })
            .unwrap_or_default();

        if constraints.size().effective_value(value.len()).into_inner() == 0 {
            // NO-OP
        } else if extensible_is_present {
            self.encode_length(&mut buffer, value.len(), <_>::default(), |range| {
                Ok(BitString::from_slice(&value[range]))
            })?;
        } else {
            self.encode_length(&mut buffer, value.len(), constraints.size(), |range| {
                Ok(BitString::from_slice(&value[range]))
            })?;
        }

        self.extend(tag, &buffer);

        Ok(())
    }

    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::VisibleString,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::default();
        let characters = self.options.aligned.then(|| value.to_iso646_bytes());
        let characters = characters.as_deref();

        self.encode_length(&mut buffer, value.len(), constraints.size(), |range| {
            Ok(match characters {
                Some(characters) => characters[range].view_bits::<Msb0>().to_bitvec(),
                None => value[range].to_bitvec(),
            })
        })?;

        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_utf8_string(&mut self, tag: Tag, value: &str) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, <_>::default(), value.as_bytes())
    }

    fn encode_utc_time(
        &mut self,
        _tag: Tag,
        _value: &types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_generalized_time(
        &mut self,
        _tag: Tag,
        _value: &types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &[E],
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::default();
        let options = self.options.clone();

        self.encode_length(&mut buffer, values.len(), constraints.size(), |range| {
            let mut buffer = BitString::default();
            for value in &values[range] {
                let mut encoder = Self::new(options);
                value.encode(&mut encoder)?;
                buffer.extend(encoder.bitstring_output());
            }
            Ok(buffer)
        })?;

        self.extend(tag, &buffer);

        Ok(())
    }

    fn encode_set_of<E: Encode>(
        &mut self,
        tag: Tag,
        values: &types::SetOf<E>,
        constraints: Constraints,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_sequence_of(tag, &values.iter().collect::<Vec<_>>(), constraints)
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        tag: Tag,
        value: &V,
    ) -> Result<Self::Ok, Self::Error> {
        value.encode_with_tag(self, tag)
    }

    fn encode_some<E: Encode>(&mut self, value: &E) -> Result<Self::Ok, Self::Error> {
        self.field_bitfield.push(true);
        value.encode(self)
    }

    fn encode_some_with_tag<E: Encode>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.field_bitfield.push(true);
        value.encode_with_tag(self, tag)
    }

    fn encode_none<E: Encode>(&mut self) -> Result<Self::Ok, Self::Error> {
        self.field_bitfield.push(false);
        Ok(())
    }

    fn encode_sequence<F>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        let mut sequence = Self::new(self.options.without_set_encoding());
        (encoder_scope)(&mut sequence)?;
        self.encode_constructed(tag, constraints.extensible(), sequence);
        Ok(())
    }

    fn encode_set<F>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        let mut set = self.new_set_encoder();

        (encoder_scope)(&mut set)?;

        self.encode_constructed(tag, constraints.extensible(), set);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Encoder as _;

    #[derive(crate::AsnType, Default, crate::Encode, Clone, Copy)]
    #[rasn(crate_root = "crate")]
    struct Byte {
        one: bool,
        two: bool,
        three: bool,
        four: bool,
        five: bool,
        six: bool,
        seven: bool,
        eight: bool,
    }

    impl Byte {
        const MAX: Self = Self {
            one: true,
            two: true,
            three: true,
            four: true,
            five: true,
            six: true,
            seven: true,
            eight: true,
        };
    }

    #[test]
    fn sequence() {
        assert_eq!(&[0xff], &*crate::uper::encode(&Byte::MAX).unwrap());
    }

    #[test]
    fn constrained_integer() {
        assert_eq!(&[0xff], &*crate::uper::encode(&0xffu8).unwrap());
    }
    #[test]
    fn unconstrained_integer() {
        assert_eq!(
            &[0b00000010, 0b00010000, 0],
            &*crate::uper::encode(&types::Integer::from(4096)).unwrap()
        );
        struct CustomInt(i32);

        impl crate::AsnType for CustomInt {
            const TAG: Tag = Tag::INTEGER;
        }

        impl crate::Encode for CustomInt {
            fn encode_with_tag<E: crate::Encoder>(
                &self,
                encoder: &mut E,
                tag: Tag,
            ) -> Result<(), E::Error> {
                encoder
                    .encode_integer(
                        tag,
                        Constraints::from(&[constraints::Value::from(constraints::Range::up_to(types::Integer::from(
                            65535,
                        ))
                        ).into()]),
                        &self.0.into(),
                    )
                    .map(drop)
            }
        }

        assert_eq!(
            &[0b00000001, 0b01111111],
            &*crate::uper::encode(&CustomInt(127)).unwrap()
        );
        assert_eq!(
            &[0b00000001, 0b10000000],
            &*crate::uper::encode(&CustomInt(-128)).unwrap()
        );
        assert_eq!(
            &[0b00000010, 0b00000000, 0b10000000],
            &*crate::uper::encode(&CustomInt(128)).unwrap()
        );
    }

    #[test]
    fn semi_constrained_integer() {
        let mut encoder = Encoder::new(EncoderOptions::unaligned());
        encoder
            .encode_integer(
                Tag::INTEGER,
                Constraints::from(&[
                    constraints::Value::from(constraints::Range::start_from(types::Integer::from(-1))).into()
                ]),
                &4096.into(),
            )
            .unwrap();

        assert_eq!(&[2, 0b00010000, 1], &*encoder.output.clone().into_vec());
        encoder.output.clear();
        encoder
            .encode_integer(
                Tag::INTEGER,
                Constraints::from(
                    &[constraints::Value::from(constraints::Range::start_from(types::Integer::from(1))).into()],
                ),
                &127.into(),
            )
            .unwrap();
        assert_eq!(&[1, 0b01111110], &*encoder.output.clone().into_vec());
        encoder.output.clear();
        encoder
            .encode_integer(
                Tag::INTEGER,
                Constraints::from(
                    &[constraints::Value::from(constraints::Range::start_from(types::Integer::from(0))).into()],
                ),
                &128.into(),
            )
            .unwrap();
        assert_eq!(&[1, 0b10000000], &*encoder.output.into_vec());
    }

    fn assert_encode<T: Encode>(options: EncoderOptions, value: T, expected: &[u8]) {
        let mut encoder = Encoder::new(options);
        T::encode(&value, &mut encoder).unwrap();
        let output = encoder.output.clone().into_vec();
        assert_eq!(
            expected
                .iter()
                .map(|ch| format!("{ch:08b}"))
                .collect::<Vec<_>>(),
            output
                .iter()
                .map(|ch| format!("{ch:08b}"))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn visible_string() {
        use crate::types::VisibleString;

        assert_encode(
            EncoderOptions::unaligned(),
            VisibleString::from("John"),
            &[4, 0x95, 0xBF, 0x46, 0xE0],
        );
        assert_encode(
            EncoderOptions::aligned(),
            VisibleString::from("John"),
            &[4, 0x4A, 0x6F, 0x68, 0x6E],
        );
    }

    #[test]
    fn sequence_of() {
        let make_buffer =
            |length| crate::uper::encode(&alloc::vec![Byte::default(); length]).unwrap();
        assert_eq!(&[5, 0, 0, 0, 0, 0], &*(make_buffer)(5));
        assert!((make_buffer)(130).starts_with(&[0b10000000u8, 0b10000010]));
        assert!((make_buffer)(16000).starts_with(&[0b10111110u8, 0b10000000]));
        let buffer = (make_buffer)(THIRTY_TWO_K);
        assert_eq!(THIRTY_TWO_K + 2, buffer.len());
        assert!(buffer.starts_with(&[0b11000010]));
        assert!(buffer.ends_with(&[0]));
        let buffer = (make_buffer)(99000);
        assert_eq!(99000 + 4, buffer.len());
        assert!(buffer.starts_with(&[0b11000100]));
        assert!(buffer[1 + SIXTY_FOUR_K..].starts_with(&[0b11000010]));
        assert!(buffer[SIXTY_FOUR_K + THIRTY_TWO_K + 2..].starts_with(&[0b10000010, 0b10111000]));
    }
}
