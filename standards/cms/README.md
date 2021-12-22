# Cryptographic Message Syntax

`rasn-cms` is an implementation of the data types defined in IETF
[RFC 4108], [RFC 5083], [RFC 5084], and [RFC 5652]. Also known as
Cryptographic Message Syntax (CMS) or PKCS#7.

This does not provide an implementation of a CMS generator or validator, instead
`rasn-cms` provides an implementation of the underlying data types used to 
decode and encode the CMS structures from/to DER or BER.

[RFC 4108]: https://datatracker.ietf.org/doc/html/rfc4108
[RFC 5083]: https://datatracker.ietf.org/doc/html/rfc5083
[RFC 5084]: https://datatracker.ietf.org/doc/html/rfc5084
[RFC 5652]: https://datatracker.ietf.org/doc/html/rfc5652

