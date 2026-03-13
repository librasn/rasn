//! Intermediate representation for AVN values.

use core::fmt;

/// Intermediate representation of an ASN.1 Value Notation value.
#[derive(Debug, Clone)]
pub enum AvnValue {
    /// BOOLEAN value: `TRUE` or `FALSE`
    Boolean(bool),
    /// INTEGER value as decimal string, e.g. `42`
    Integer(alloc::string::String),
    /// OCTET STRING value as raw bytes; displayed as `'HEX'H`
    OctetString(alloc::vec::Vec<u8>),
    /// BIT STRING value with explicit bit length for non-byte-aligned strings
    BitString {
        /// Raw byte storage (last byte may be partially used)
        bytes: alloc::vec::Vec<u8>,
        /// Number of meaningful bits
        bit_length: usize,
    },
    /// NULL value: `NULL`
    Null,
    /// OBJECT IDENTIFIER as a vec of arcs; displayed as `{ 1 2 3 }`
    Oid(alloc::vec::Vec<u32>),
    /// REAL value as a decimal string or special keyword
    Real(alloc::string::String),
    /// ENUMERATED value as an identifier string
    Enumerated(alloc::string::String),
    /// SEQUENCE / SET as an ordered list of (field-name, value) pairs.
    /// `None` entries represent absent OPTIONAL fields and are omitted from output.
    Sequence(alloc::vec::Vec<(alloc::string::String, Option<AvnValue>)>),
    /// SEQUENCE OF / SET OF as an ordered list of values
    SequenceOf(alloc::vec::Vec<AvnValue>),
    /// CHOICE value: `identifier : value`
    Choice {
        /// The chosen alternative's identifier
        identifier: alloc::string::String,
        /// The value of the chosen alternative
        value: alloc::boxed::Box<AvnValue>,
    },
    /// Any character-string type: `"quoted string"` (X.680 double-quote escaping)
    CharString(alloc::string::String),
}

// ---------------------------------------------------------------------------
// Display helpers
// ---------------------------------------------------------------------------

/// Helper that carries a depth counter for indentation.
struct AvnFmt<'a> {
    value: &'a AvnValue,
    depth: usize,
}

impl<'a> AvnFmt<'a> {
    fn new(value: &'a AvnValue, depth: usize) -> Self {
        Self { value, depth }
    }
}

impl fmt::Display for AvnFmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let depth = self.depth;
        // Indentation strings: items at depth+1, closing brace at depth
        let inner_indent = alloc::format!("{:width$}", "", width = (depth + 1) * 2);
        let outer_indent = alloc::format!("{:width$}", "", width = depth * 2);

        match self.value {
            AvnValue::Boolean(b) => {
                if *b {
                    f.write_str("TRUE")
                } else {
                    f.write_str("FALSE")
                }
            }

            AvnValue::Integer(s) => f.write_str(s),

            AvnValue::OctetString(bytes) => {
                f.write_str("'")?;
                for b in bytes {
                    write!(f, "{b:02X}")?;
                }
                f.write_str("'H")
            }

            AvnValue::BitString { bytes, bit_length } => {
                if *bit_length == 0 {
                    return f.write_str("''H");
                }
                if bit_length % 8 == 0 {
                    // Byte-aligned: hex notation
                    f.write_str("'")?;
                    for b in bytes {
                        write!(f, "{b:02X}")?;
                    }
                    f.write_str("'H")
                } else {
                    // Non-byte-aligned: binary notation — emit exactly bit_length bits from MSB
                    f.write_str("'")?;
                    let mut remaining = *bit_length;
                    for b in bytes {
                        let bits_in_this_byte = remaining.min(8);
                        // Emit bits from MSB downward
                        for shift in (8 - bits_in_this_byte..8).rev() {
                            write!(f, "{}", (b >> shift) & 1)?;
                        }
                        remaining = remaining.saturating_sub(8);
                        if remaining == 0 {
                            break;
                        }
                    }
                    f.write_str("'B")
                }
            }

            AvnValue::Null => f.write_str("NULL"),

            AvnValue::Oid(arcs) => {
                f.write_str("{")?;
                for arc in arcs {
                    write!(f, " {arc}")?;
                }
                f.write_str(" }")
            }

            AvnValue::Real(s) => f.write_str(s),

            AvnValue::Enumerated(id) => f.write_str(id),

            AvnValue::Sequence(fields) => {
                let present: alloc::vec::Vec<_> = fields
                    .iter()
                    .filter_map(|(name, v)| v.as_ref().map(|v| (name, v)))
                    .collect();

                if present.is_empty() {
                    return f.write_str("{}");
                }

                writeln!(f, "{{")?;
                for (idx, (name, val)) in present.iter().enumerate() {
                    let comma = if idx + 1 < present.len() { "," } else { "" };
                    writeln!(
                        f,
                        "{inner_indent}{name} {val}{comma}",
                        val = AvnFmt::new(val, depth + 1)
                    )?;
                }
                write!(f, "{outer_indent}}}")
            }

            AvnValue::SequenceOf(items) => {
                if items.is_empty() {
                    return f.write_str("{}");
                }
                writeln!(f, "{{")?;
                for (idx, item) in items.iter().enumerate() {
                    let comma = if idx + 1 < items.len() { "," } else { "" };
                    writeln!(
                        f,
                        "{inner_indent}{val}{comma}",
                        val = AvnFmt::new(item, depth + 1)
                    )?;
                }
                write!(f, "{outer_indent}}}")
            }

            AvnValue::Choice { identifier, value } => {
                write!(f, "{identifier} : {}", AvnFmt::new(value, depth))
            }

            AvnValue::CharString(s) => {
                // X.680: embed a literal `"` as `""`
                f.write_str("\"")?;
                for ch in s.chars() {
                    if ch == '"' {
                        f.write_str("\"\"")?;
                    } else {
                        write!(f, "{ch}")?;
                    }
                }
                f.write_str("\"")
            }
        }
    }
}

impl fmt::Display for AvnValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        AvnFmt::new(self, 0).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_display() {
        assert_eq!(AvnValue::Boolean(true).to_string(), "TRUE");
        assert_eq!(AvnValue::Boolean(false).to_string(), "FALSE");
    }

    #[test]
    fn integer_display() {
        assert_eq!(AvnValue::Integer("42".into()).to_string(), "42");
        assert_eq!(AvnValue::Integer("-1".into()).to_string(), "-1");
    }

    #[test]
    fn null_display() {
        assert_eq!(AvnValue::Null.to_string(), "NULL");
    }

    #[test]
    fn octet_string_display() {
        assert_eq!(AvnValue::OctetString(alloc::vec![]).to_string(), "''H");
        assert_eq!(
            AvnValue::OctetString(alloc::vec![0x01, 0xFF, 0xAB]).to_string(),
            "'01FFAB'H"
        );
    }

    #[test]
    fn bit_string_hex_display() {
        // Byte-aligned (8 bits)
        let v = AvnValue::BitString {
            bytes: alloc::vec![0xA0],
            bit_length: 8,
        };
        assert_eq!(v.to_string(), "'A0'H");
    }

    #[test]
    fn bit_string_empty_display() {
        let v = AvnValue::BitString {
            bytes: alloc::vec![],
            bit_length: 0,
        };
        assert_eq!(v.to_string(), "''H");
    }

    #[test]
    fn oid_display() {
        let v = AvnValue::Oid(alloc::vec![1, 2, 840, 113549]);
        assert_eq!(v.to_string(), "{ 1 2 840 113549 }");
    }

    #[test]
    fn enumerated_display() {
        assert_eq!(
            AvnValue::Enumerated("someIdentifier".into()).to_string(),
            "someIdentifier"
        );
    }

    #[test]
    fn char_string_display() {
        assert_eq!(
            AvnValue::CharString("hello".into()).to_string(),
            "\"hello\""
        );
        // embedded quote must be doubled
        assert_eq!(
            AvnValue::CharString("say \"hi\"".into()).to_string(),
            "\"say \"\"hi\"\"\""
        );
    }

    #[test]
    fn choice_display() {
        let v = AvnValue::Choice {
            identifier: "myField".into(),
            value: alloc::boxed::Box::new(AvnValue::Boolean(true)),
        };
        assert_eq!(v.to_string(), "myField : TRUE");
    }

    #[test]
    fn sequence_display() {
        let v = AvnValue::Sequence(alloc::vec![
            ("field1".into(), Some(AvnValue::Boolean(true))),
            ("field2".into(), Some(AvnValue::Integer("42".into()))),
        ]);
        let expected = "{\n  field1 TRUE,\n  field2 42\n}";
        assert_eq!(v.to_string(), expected);
    }

    #[test]
    fn sequence_display_skips_absent() {
        let v = AvnValue::Sequence(alloc::vec![
            ("field1".into(), Some(AvnValue::Boolean(true))),
            ("field2".into(), None),
        ]);
        let expected = "{\n  field1 TRUE\n}";
        assert_eq!(v.to_string(), expected);
    }

    #[test]
    fn sequence_display_with_null_field() {
        let v = AvnValue::Sequence(alloc::vec![
            ("field1".into(), Some(AvnValue::Boolean(true))),
            ("flag".into(), Some(AvnValue::Null)),
        ]);
        let expected = "{\n  field1 TRUE,\n  flag NULL\n}";
        assert_eq!(v.to_string(), expected);
    }

    #[test]
    fn sequence_of_display() {
        let v = AvnValue::SequenceOf(alloc::vec![
            AvnValue::Integer("1".into()),
            AvnValue::Integer("2".into()),
        ]);
        let expected = "{\n  1,\n  2\n}";
        assert_eq!(v.to_string(), expected);
    }
}
