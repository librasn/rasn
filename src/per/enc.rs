mod error;

use alloc::vec::Vec;

use snafu::*;

use super::{FOURTY_EIGHT_K, SIXTEEN_K, SIXTY_FOUR_K, THIRTY_TWO_K};
use crate::{
    types::{self, constraints, BitString, Constraints, Tag, strings::{DynConstrainedCharacterString, ConstrainedCharacterString}},
    Encode, enc::Error as _,
};

pub use error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Clone, Copy, Default)]
pub struct EncoderOptions {
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
    field_bitfield: alloc::collections::BTreeMap<Tag, bool>,
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

    fn new_set_encoder<C: crate::types::Constructed>(&self) -> Self {
        let mut options = self.options;
        options.set_encoding = true;
        let mut encoder = Self::new(options);
        encoder.field_bitfield = C::FIELDS.canonised().optional_and_default_fields().map(|field| (field.tag_tree.smallest_tag(), false)).collect();
        encoder
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

    pub fn set_bit(&mut self, tag: Tag, bit: bool) -> Result<()> {
        self.require_fields()?;
        self.field_bitfield.entry(tag).and_modify(|b| { *b = bit });
        Ok(())
    }

    fn pad_to_alignment(&self, buffer: &mut BitString) {
        const BYTE_WIDTH: usize = 8;
        if self.options.aligned && buffer.len() % BYTE_WIDTH != 0 {
            buffer.extend(BitString::repeat(false, BYTE_WIDTH - (buffer.len() % 8)));
            debug_assert_eq!(0, buffer.len() % 8);
        }
    }

    fn encode_extensible_bit(&mut self, constraints: &Constraints, buffer: &mut BitString, extensible_condition: impl FnOnce() -> bool) -> bool {
        constraints.extensible().then(|| {
            let is_in_constraints = !(extensible_condition)();
            buffer.push(is_in_constraints);
            is_in_constraints
        }).unwrap_or_default()
    }

    fn encode_known_multipler_string<const WIDTH: usize>(&mut self, tag: Tag, constraints: &Constraints, value: &ConstrainedCharacterString<WIDTH>) -> Result<()> {
        let mut buffer = BitString::default();
        let characters = match constraints.permitted_alphabet() {
            Some(alphabet)  => {
                let alphabet_width = crate::per::log2(alphabet.len() as i128);
                if WIDTH as u32 > self.options.aligned.then(|| alphabet_width.next_power_of_two()).unwrap_or(alphabet_width) {
                    Some(DynConstrainedCharacterString::from_known_multiplier_string(value, alphabet).map_err(Error::custom)?)
                } else {
                    None
                }
            }
            None => None,
        };

        let is_aligned = self.options.aligned;
        let ref characters = characters;

        self.encode_length(&mut buffer, value.len(), constraints.size(), |range| {
            Ok(if is_aligned {
                match characters {
                    Some(characters) => characters.to_octet_aligned()[range].to_bitvec(),
                    None => value.to_octet_aligned()[range].to_bitvec(),
                }
            } else {
                match characters {
                    Some(characters) => characters[range].to_bitvec(),
                    None => value[range].to_bitvec(),
                }
            })
        })?;

        self.extend(tag, &buffer);
        Ok(())
    }

    fn require_fields(&self) -> Result<()> {
        self.field_bitfield.is_empty()
            .then(|| Err(Error::custom("encode_some_* can only be called inside a sequence or set type")))
            .unwrap_or(Ok(()))
    }

    fn encode_constructed(&mut self, tag: Tag, constraints: &Constraints, encoder: Self) {
        let mut buffer = BitString::default();

        self.encode_extensible_bit(constraints, &mut buffer, || todo!());

        for bit in encoder.field_bitfield.values().copied() {
            buffer.push(bit);
        }

        self.pad_to_alignment(&mut buffer);
        buffer.extend(encoder.bitstring_output());
        self.extend(tag, &buffer);
    }

    fn encode_normally_small_integer(&mut self, value: usize, buffer: &mut BitString)
        -> Result<()>
    {
        let is_large = value >= 64;
        buffer.push(is_large);

        let size_constraints = if is_large {
            constraints::Value::new(constraints::Range::start_from(0)).into()
        } else {
            constraints::Value::new(constraints::Range::new(0, 63)).into()
        };

        self.encode_integer_into_buffer(
            Constraints::new(&[size_constraints]),
            &value.into(),
            buffer,
        )
    }

    fn encode_length(
        &mut self,
        buffer: &mut BitString,
        length: usize,
        constraints: constraints::Range<usize>,
        encode_fn: impl Fn(core::ops::Range<usize>) -> Result<BitString>,
    ) -> Result<()> {
        Error::check_length(length, constraints)?;

        let range = constraints.range().unwrap_or(usize::MAX);
        let effective_length = constraints.effective_value(length).into_inner();
        let encode_min = |buffer: &mut BitString| -> Result<()> {
            if let Some(min) = constraints.as_start() {
                buffer.extend((encode_fn)(0..*min)?);
            }

            Ok(())
        };

        if range == 0 {
            (encode_min)(buffer)?;
        } else if effective_length <= 127 {
            buffer.extend((effective_length as u8).to_be_bytes());
            self.pad_to_alignment(buffer);
            buffer.extend((encode_fn)(0..length)?);
        } else if effective_length < SIXTEEN_K.into() {
            const SIXTEENTH_BIT: u16 = 0x8000;
            buffer.extend((SIXTEENTH_BIT | effective_length as u16).to_be_bytes());
            buffer.extend((encode_fn)(0..length)?);
        } else {
            let mut effective_length = effective_length;
            let mut min = constraints.start();
            (encode_min)(buffer)?;

            loop {
                // Hack to get around no exclusive syntax.
                const K64: usize = SIXTY_FOUR_K as usize;
                const K48: usize = FOURTY_EIGHT_K as usize;
                const K32: usize = THIRTY_TWO_K as usize;
                const K16: usize = SIXTEEN_K as usize;
                const K64_MAX: usize = K64 - 1;
                const K48_MAX: usize = K48 - 1;
                const K32_MAX: usize = K32 - 1;
                let (fragment_index, amount) = match effective_length {
                    K64..=usize::MAX => (4, K64),
                    K48..=K64_MAX => (3, K48),
                    K32..=K48_MAX => (2, K32),
                    K16..=K32_MAX => (1, K16),
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

    fn encode_octet_string_into_buffer(
        &mut self,
        constraints: Constraints,
        value: &[u8],
        buffer: &mut BitString,
    ) -> Result<()> {
        let extensible_is_present = self.encode_extensible_bit(&constraints, buffer, || todo!());

        if constraints.size().effective_value(value.len()).into_inner() == 0 {
            // NO-OP
        } else if extensible_is_present {
            self.encode_length(buffer, value.len(), <_>::default(), |range| {
                Ok(BitString::from_slice(&value[range]))
            })?;
        } else {
            self.encode_length(buffer, value.len(), constraints.size(), |range| {
                Ok(BitString::from_slice(&value[range]))
            })?;
        }

        Ok(())
    }

    fn encode_integer_into_buffer(
        &mut self,
        constraints: Constraints,
        value: &num_bigint::BigInt,
        buffer: &mut BitString,
    ) -> Result<()> {
        let value_range = constraints.value();
        let bytes = match value_range.effective_bigint_value(value.clone()) {
            either::Left(offset) => offset.to_biguint().unwrap().to_bytes_be(),
            either::Right(value) => value.to_signed_bytes_be(),
        };

        self.encode_extensible_bit(&constraints, buffer, || todo!());

        if let Some(range) = value_range
            .range()
            .filter(|range| *range < SIXTY_FOUR_K.into())
        {
            let total_bits = super::log2(range) as usize;
            let mut bits = BitString::from_vec(bytes);
            bits.resize(total_bits, false);
            if range >= super::TWO_FIFTY_SIX.into() {
                self.pad_to_alignment(buffer);
            }
            buffer.extend(bits);
        } else {
            self.encode_length(buffer, bytes.len(), constraints.size(), |range| {
                Ok(BitString::from_slice(&bytes[range]))
            })?;
        }

        Ok(())
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

    fn encode_any(&mut self, tag: Tag, value: &types::Any) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, <_>::default(), &value.contents)
    }

    fn encode_bit_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &BitString,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::default();
        let extensible_is_present = self.encode_extensible_bit(&constraints, &mut buffer, || todo!());

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
            Constraints::new(&[
                constraints::Size::new(constraints::Range::up_to(variance)).into(),
            ]),
            &value.into(),
        )
    }

    fn encode_integer(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &num_bigint::BigInt,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::new();
        self.encode_integer_into_buffer(constraints, value, &mut buffer)?;
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
        self.encode_octet_string_into_buffer(constraints, value, &mut buffer)?;
        self.extend(tag, &buffer);
        Ok(())
    }

    fn encode_visible_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::VisibleString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multipler_string(tag, &constraints, value)
    }

    fn encode_ia5_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::Ia5String,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multipler_string(tag, &constraints, value)
    }

    fn encode_printable_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::PrintableString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multipler_string(tag, &constraints, value)
    }

    fn encode_numeric_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::NumericString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multipler_string(tag, &constraints, value)
    }

    fn encode_teletex_string(
        &mut self,
        tag: Tag,
        _: Constraints,
        value: &types::TeletexString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, <_>::default(), value)
    }

    fn encode_bmp_string(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &types::BmpString,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_known_multipler_string(tag, &constraints, value)
    }

    fn encode_utf8_string(&mut self, tag: Tag, _: Constraints, value: &str) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, <_>::default(), value.as_bytes())
    }

    fn encode_utc_time(
        &mut self,
        tag: Tag,
        value: &types::UtcTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, <_>::default(), &crate::der::encode(value).context(error::DerSnafu)?)
    }

    fn encode_generalized_time(
        &mut self,
        tag: Tag,
        value: &types::GeneralizedTime,
    ) -> Result<Self::Ok, Self::Error> {
        self.encode_octet_string(tag, <_>::default(), &crate::der::encode(value).context(error::DerSnafu)?)
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
        self.set_bit(E::TAG, true)?;
        value.encode(self)
    }

    fn encode_some_with_tag<E: Encode>(
        &mut self,
        tag: Tag,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        value.encode_with_tag(self, tag)
    }

    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        value: &E,
    ) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, true)?;
        value.encode_with_tag_and_constraints(self, tag, constraints)
    }

    fn encode_none<E: Encode>(&mut self) -> Result<Self::Ok, Self::Error> {
        self.set_bit(E::TAG, false)?;
        Ok(())
    }

    fn encode_none_with_tag(&mut self, tag: Tag) -> Result<Self::Ok, Self::Error> {
        self.set_bit(tag, false)?;
        Ok(())
    }

    fn encode_sequence<C, F>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        let mut sequence = Self::new(self.options.without_set_encoding());
        (encoder_scope)(&mut sequence)?;
        self.encode_constructed(tag, &constraints, sequence);
        Ok(())
    }

    fn encode_set<C, F>(
        &mut self,
        tag: Tag,
        constraints: Constraints,
        encoder_scope: F,
    ) -> Result<Self::Ok, Self::Error>
    where
        C: crate::types::Constructed,
        F: FnOnce(&mut Self) -> Result<Self::Ok, Self::Error>,
    {
        let mut set = self.new_set_encoder::<C>();

        (encoder_scope)(&mut set)?;

        self.encode_constructed(tag, &constraints, set);

        Ok(())
    }

    fn encode_choice<E: Encode + crate::types::Choice>(
        &mut self,
        constraints: Constraints,
        encode_fn: impl FnOnce(&mut Self)  -> Result<Tag, Self::Error>,
    ) -> Result<Self::Ok, Self::Error> {
        let mut buffer = BitString::new();
        let mut choice_encoder = Self::new(self.options.without_set_encoding());
        let tag = (encode_fn)(&mut choice_encoder)?;
        let is_root_extension = crate::TagTree::tag_contains(&tag, E::VARIANTS);

        self.encode_extensible_bit(&constraints, &mut buffer, || is_root_extension);
        let variants = crate::types::variants::Variants::from_static(
            is_root_extension.then(|| E::VARIANTS).unwrap_or(E::EXTENDED_VARIANTS)
        );

        let index = variants
            .iter()
            .enumerate()
            .find_map(|(i, &variant_tag)| (tag == variant_tag).then(|| i))
            .ok_or_else(|| Error::custom("variant not found in choice"))?;

        let bounds = if is_root_extension {
            let variance = variants.len();

            if variance != 1 {
                Some(Some(variance))
            } else {
                None
            }
        } else {
            Some(None)
        };

        match (index, bounds) {
            (index, Some(Some(variance))) => {
                self.encode_integer_into_buffer(
                    Constraints::new(&[
                        constraints::Value::new(constraints::Range::new(0, variance as i128)).into()
                    ]),
                    &index.into(),
                    &mut buffer,
                )?;

                buffer.extend(choice_encoder.output);
            }
            (index, Some(None)) => {
                self.encode_normally_small_integer(index, &mut buffer)?;
                self.pad_to_alignment(&mut buffer);
                self.encode_octet_string_into_buffer(<_>::default(), &choice_encoder.output(), &mut buffer)?;
            }
            (_, None) => {},
        }

        self.extend(tag, &buffer);
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
            const CONSTRAINTS: Constraints<'static> = Constraints::new(&[
                constraints::Constraint::Value(constraints::Value::new(constraints::Range::up_to(65535)))
            ]);
        }

        impl crate::Encode for CustomInt {
            fn encode_with_tag_and_constraints<E: crate::Encoder>(
                &self,
                encoder: &mut E,
                tag: Tag,
                constraints: Constraints,
            ) -> Result<(), E::Error> {
                encoder
                    .encode_integer(
                        tag,
                        constraints,
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
                    constraints::Value::from(constraints::Range::start_from(-1)).into()
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
                    &[constraints::Value::from(constraints::Range::start_from(1)).into()],
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
                    &[constraints::Value::from(constraints::Range::start_from(0)).into()],
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
            VisibleString::try_from("John").unwrap(),
            &[4, 0x95, 0xBF, 0x46, 0xE0],
        );
        assert_encode(
            EncoderOptions::aligned(),
            VisibleString::try_from("John").unwrap(),
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
        let buffer = (make_buffer)(THIRTY_TWO_K as usize);
        assert_eq!(THIRTY_TWO_K as usize + 2, buffer.len());
        assert!(buffer.starts_with(&[0b11000010]));
        assert!(buffer.ends_with(&[0]));
        let buffer = (make_buffer)(99000);
        assert_eq!(99000 + 4, buffer.len());
        assert!(buffer.starts_with(&[0b11000100]));
        assert!(buffer[1 + SIXTY_FOUR_K as usize..].starts_with(&[0b11000010]));
        assert!(buffer[SIXTY_FOUR_K as usize + THIRTY_TWO_K as usize + 2..].starts_with(&[0b10000010, 0b10111000]));
    }
}
