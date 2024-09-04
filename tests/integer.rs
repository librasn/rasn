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
                    }
                    else {
                        assert!(matches!(min.into(), Integer::Variable(_)));
                    }
                    if max <= isize::MAX as u128 {
                        assert!(matches!(max.into(), Integer::Primitive(_)));
                    }else   {
                        assert!(matches!(max.into(), Integer::Variable(_)));
                    }

                    let value = 5 as $t;
                    let (unsigned_unsigned_bytes, needed) = value.to_unsigned_bytes_be();
                    assert_eq!(&unsigned_unsigned_bytes.as_ref()[..needed], &[5]);
                    let (signed_unsigned_bytes, needed) = value.to_signed_bytes_be();
                    assert_eq!(&signed_unsigned_bytes.as_ref()[..needed], &[5]);
                    let value = $t::try_from(128);
                    #[allow(irrefutable_let_patterns)]
                    if let Ok(number) = value {
                        let (unsigned_bytes, needed) = number.to_unsigned_bytes_be();
                        assert_eq!(&unsigned_bytes.as_ref()[..needed], &[128]);
                        let (signed_bytes, needed) = number.to_signed_bytes_be();
                        assert_eq!(&signed_bytes.as_ref()[..needed], &[0, 128]);
                    }
                    let value = $t::try_from(-5);
                    #[allow(irrefutable_let_patterns)]
                    if let Ok(number) = value {
                        // Signed bytes and unsigned bytes are the same for negative numbers
                        // However, when converting the actual integer, they mean different things
                        // 2's complement representation is ignored when converting unsigned bytes to whole number
                        // So "needed" includes all leading ones
                        let (unsigned_bytes, needed) = number.to_unsigned_bytes_be();
                        assert_eq!(unsigned_bytes.as_ref()[..needed].last().unwrap(), &251);
                        let (signed_bytes, needed) = number.to_signed_bytes_be();
                        assert_eq!(&signed_bytes.as_ref()[..needed], &[251]);
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
