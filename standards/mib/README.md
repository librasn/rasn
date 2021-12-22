# Management Information Base
This crate represents an implementation of MIB objects defined in IETF RFCs,
Nearly all of these types are newtype wrappers around their network
protocol type, and as such don't they add any additional overhead in terms
of size, any `OBJECT-TYPE` information is available statically through the
[`smi::ObjectType`] trait.

