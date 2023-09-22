// use crate::types::Tag;
// use snafu::Snafu;
//
// /// An error that occurred when decoding BER or any of its variants.
// #[derive(Snafu)]
// #[snafu(visibility(pub(crate)))]
// #[derive(Debug)]
// pub enum Error {
//     /// More bytes needed to parse the complete value.
//     // #[snafu(display("Need more bytes to continue ({:?}).", needed))]
//     // Incomplete {
//     //     /// Amount of bytes needed.
//     //     needed: nom::Needed,
//     // },
//     // /// Constructed encoding encountered but not allowed.
//     // ConstructedEncodingNotAllowed,
//     /// Indefinite length encountered but not allowed.
//     // IndefiniteLengthNotAllowed,
//     // /// BOOL value is not `0` or `0xFF`.
//     // InvalidBool,
//     // /// OBJECT IDENTIFIER with missing or corrupt root nodes.
//     // InvalidObjectIdentifier,
//     // /// Invalid UTF-8 data.
//     // InvalidUtf8,
//     // /// Invalid date.
//     // InvalidDate,
//     /// Custom error in the parser.
//     // #[snafu(display("Error in Parser: {}", msg))]
//     // Parser {
//     //     /// The error's message.
//     //     msg: alloc::string::String,
//     // },

//
//
//     /// The length exceeds the maximum length allowed.
//     // #[snafu(display("Expected maximum of {} items", length))]
//     // ExceedsMaxLength {
//     //     /// The maximum length.
//     //     length: num_bigint::BigUint,
//     // },
//
//     /// Expected a certain field, which was not present.
//     // #[snafu(display("Expected required field `{}`", name))]
//     // MissingField {
//     //     /// The name of the field.
//     //     name: &'static str,
//     // },
//     /// No valid choice variant found.
//     // #[snafu(display("No valid `CHOICE` variant for `{}`", name))]
//     // NoValidChoice {
//     //     /// The name of the `CHOICE`.
//     //     name: &'static str,
//     // },
//     /// An error occurred while decoding a field.
//     // #[snafu(display("Field `{}`: {}", name, error))]
//     // FieldError {
//     //     /// The field's name.
//     //     name: &'static str,
//     //     /// The error that occurred.
//     //     error: alloc::string::String,
//     // },
//     /// A custom error.
//     // #[snafu(display("{}", msg))]
//     // Custom {
//     //     /// the error's message.
//     //     msg: alloc::string::String,
//     // },
// }
//
//
//
// impl crate::de::Error for Error {
//     fn custom<D: core::fmt::Display>(msg: D) -> Self {
//         Self::Custom {
//             msg: alloc::string::ToString::to_string(&msg),
//         }
//     }
//
//     fn incomplete(needed: nom::Needed) -> Self {
//         Self::Incomplete { needed }
//     }
//
//     fn exceeds_max_length(length: num_bigint::BigUint) -> Self {
//         Self::ExceedsMaxLength { length }
//     }
//
//     fn missing_field(name: &'static str) -> Self {
//         Self::MissingField { name }
//     }
//
//     fn field_error<D: core::fmt::Display>(name: &'static str, error: D) -> Self {
//         Self::FieldError {
//             name,
//             error: alloc::string::ToString::to_string(&error),
//         }
//     }
//
//     fn duplicate_field(name: &'static str) -> Self {
//         Self::FieldError {
//             name,
//             error: alloc::string::ToString::to_string("Duplicate field found"),
//         }
//     }
//
//     fn no_valid_choice(name: &'static str) -> Self {
//         Self::NoValidChoice { name }
//     }
// }
