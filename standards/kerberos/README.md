# Kerberos Version 5
This is an implementation of the data types from [RFC 4120] also known as
"Kerberos V5". Kerberos is an authentication framework for verifying
identities of "principals" (e.g. a user or network server) on an open
unprotected network.

This is accomplished without relying on assertions by the host operating
system, without basing trust on host addresses, without requiring physical
security of all the hosts on the network, and under the assumption that
packets traveling along the network can be read, modified, and inserted at
will. Kerberos performs authentication under these conditions as a trusted
third-party authentication service by using conventional (shared secret
key) cryptography.

Like other `rasn` core crates this crate does not provide the ability to
authenticate on its own, but provides shared data types to create your own
Kerberos clients and servers.

[RFC 4120]: https://datatracker.ietf.org/doc/html/rfc4120

