
# IEEE 1609.2-2022 Standard for Wireless Access in Vehicular Environments--Security Services for Application and Management Messages

The standard can be found [here](https://standards.ieee.org/ieee/1609.2/10258/).

The module contains `rasn` ASN.1 data structures for the IEEE 1609.2-2022 standard.
The data structures have been partially constructed with compiler, but they have been manually modified and verified to make them more usable. They also haven been modified to be more idiomatic Rust.

Comments have been edited and shortened to be more suitable for Rust documentation.
Claude 3.5 has been used on this process. The output has been reviewed.
You should always rely on the original standard for the most accurate information.

Basic features:
 * All `newtype`'s implement `Deref` and `DerefMut` the inner type.
 * `From` and `Into` implementations for converting between similar types.
 * Most structs use `builder` pattern for construction with [bon](https://github.com/elastio/bon) crate.
 * For types with inner subtype constraints, there is `InnerSubtypeConstraint` marker trait and relevant validation methods.
