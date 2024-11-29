
# IEEE 1609.2-2022 Standard for Wireless Access in Vehicular Environments--Security Services for Application and Management Messages

The standard can be found [here](https://standards.ieee.org/ieee/1609.2/10258/).

The crate contains `rasn` ASN.1 data structures for the IEEE 1609.2-2022 standard.
The data structures have been partially constructed with compiler, but they have been manually modified and verified to make them more usable. They also haven been modified to be more idiomatic Rust.

Extra features:
 * Most of the types implement `Deref` and to the inner type.
 * `From` and `Into` implementations for converting between types.
 * Structs use `builder` pattern for construction with [bon](https://github.com/elastio/bon) crate.
