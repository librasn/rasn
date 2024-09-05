macro_rules! assert_integer_round_trip {
    ($t:ty, $value:expr, $expected_unsigned:expr, $expected_signed:expr) => {{
        let value = <$t>::try_from($value).unwrap();

        // Test unsigned bytes
        let (unsigned_bytes, unsigned_needed) = value.to_unsigned_bytes_be();
        assert_eq!(
            &unsigned_bytes.as_ref()[..unsigned_needed],
            $expected_unsigned,
            "Unsigned bytes mismatch for {}",
            stringify!($value)
        );

        // Only test unsigned round-trip for non-negative values or unsigned types
        #[allow(unused_comparisons)]
        if $value >= 0 || stringify!($t).starts_with('u') {
            assert_eq!(
                <$t>::try_from_unsigned_bytes(
                    &unsigned_bytes.as_ref()[..unsigned_needed],
                    rasn::Codec::Oer
                )
                .ok(),
                Some(value),
                "Round-trip failed for unsigned bytes of {}",
                stringify!($value)
            );
        }

        // Test signed bytes
        let (signed_bytes, signed_needed) = value.to_signed_bytes_be();
        assert_eq!(
            &signed_bytes.as_ref()[..signed_needed],
            $expected_signed,
            "Signed bytes mismatch for {}",
            stringify!($value)
        );

        // Always test signed round-trip
        assert_eq!(
            <$t>::try_from_signed_bytes(&signed_bytes.as_ref()[..signed_needed], rasn::Codec::Oer)
                .ok(),
            Some(value),
            "Round-trip failed for signed bytes of {}",
            stringify!($value)
        );
        // Round trip with Integer type should work for any type with all values
        let integer = Integer::from($value);
        let (bytes, needed) = integer.to_signed_bytes_be();
        assert_eq!(
            Integer::try_from_signed_bytes(&bytes.as_ref()[..needed], rasn::Codec::Oer).ok(),
            Some($value.into()),
            "Round-trip failed for Integer({})",
            stringify!($value)
        );
        // Check that encoding matches the signed expected bytes for Integer type
        assert_eq!(
            &bytes.as_ref()[..needed],
            $expected_signed,
            "Signed bytes mismatch for Integer({})",
            stringify!($value)
        );
    }};
}

macro_rules! test_integer_conversions_and_operations {
    ($($t:ident),*) => {
        #[cfg(test)]
        mod tests {
            use rasn::prelude::*;

            $(
                #[test]
                fn $t() {
                    let min = <$t>::MIN as i128;
                    let max = <$t>::MAX as u128;

                    if min >= isize::MIN as i128 {
                        assert!(matches!(min.into(), Integer::Primitive(_)));
                    } else {
                        assert!(matches!(min.into(), Integer::Variable(_)));
                    }
                    if max <= isize::MAX as u128 {
                        assert!(matches!(max.into(), Integer::Primitive(_)));
                    } else {
                        assert!(matches!(max.into(), Integer::Variable(_)));
                    }

                    // Test positive values
                    assert_integer_round_trip!($t, 1, &[1], &[1]);

                    // Test some signed maximum values, should be identical to unsigned
                    if <$t>::MAX as u128 >= i8::MAX as u128 {
                        assert_integer_round_trip!($t, i8::MAX as $t, &[127], &[127]);
                    }
                    // Even if the type is wider than 2 bytes, the value remains the same
                    if <$t>::MAX as u128 >= i16::MAX as u128 {
                        assert_integer_round_trip!($t, i16::MAX as $t, &[127, 255], &[127, 255]);
                        // 127 + 1 results to leading zero for signed types
                        assert_integer_round_trip!($t, 128, &[128], &[0, 128]);
                    }
                    if <$t>::MAX as u128 >= i32::MAX as u128 {
                        assert_integer_round_trip!($t, i32::MAX as $t, &[127, 255, 255, 255], &[127, 255, 255, 255]);
                        // 32_767 + 1 results to leading zero for signed types
                        assert_integer_round_trip!($t, (i16::MAX as $t + 1), &[128, 0], &[0, 128, 0]);
                    }
                    if <$t>::MAX as u128 >= i64::MAX as u128 {
                        assert_integer_round_trip!($t, i64::MAX as $t, &[127, 255, 255, 255, 255, 255, 255, 255], &[127, 255, 255, 255, 255, 255, 255, 255]);
                    }


                    // Test negative values for signed types
                    match stringify!($t) {
                        "i8" => {
                            assert_integer_round_trip!($t, -1, &[255], &[255]);
                        },
                        "i16" => {
                            assert_integer_round_trip!($t, -1, &[255, 255], &[255]);
                        },
                        "i32" => {
                            assert_integer_round_trip!($t, -1, &[255, 255, 255, 255], &[255]);
                        },
                        "i64" => {
                            assert_integer_round_trip!($t, -1, &[255, 255, 255, 255, 255, 255, 255, 255], &[255]);
                        },
                        _ => {},
                    }
                }
            )*

            #[test]
            fn test_overflow_addition() {
                let max = Integer::Primitive(isize::MAX);
                let one = Integer::Primitive(1);
                let result = max + one;
                assert!(matches!(result, Integer::Variable(_)));
            }

            #[test]
            fn test_overflow_subtraction() {
                let min = Integer::Primitive(isize::MIN);
                let one = Integer::Primitive(1);
                let result = min - one;
                assert!(matches!(result, Integer::Variable(_)));
            }
        }
    };
}

test_integer_conversions_and_operations!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
