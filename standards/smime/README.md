# Secure/Multipurpose Internet Mail Extensions
An implementation of [RFC 8551] also known as Secure/Multipurpose Internet
Mail Extensions (S/MIME). S/MIME provides a consistent way to send and
receive secure MIME data.  Based on the popular Internet MIME standard,
S/MIME provides the following cryptographic security services for electronic
messaging applications: authentication, message integrity, and
non-repudiation of origin (using digital signatures), and data
confidentiality (using encryption).

Like other `rasn` core crates. This crate does not provide the ability to
do authentication or encryption on its own, but instead provides shared data
types for creating your own S/MIME clients and servers.

[rfc 8551]: https://datatracker.ietf.org/doc/html/rfc8551

