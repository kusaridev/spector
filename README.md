# Spector
Spector is both tooling and a library for generation, validation and verification of supply chain metadata documents and frameworks.  Many tools generate non-compliant sboms or attestations.  It currently supports
* [SLSA 1.0 Provenance](https://slsa.dev/provenance/v1)
* [in-toto 1.0 Statement](https://github.com/in-toto/attestation/blob/v1.0/spec/v1.0/statement.md)

## Library
You can include spector as a library when writing your own generation.  It can provide the serialization & deserialization for SLSA attestations, providng the assurance that they are properly to spec before you go further in the process.

## Tooling
TBD

## Developing
Spector is written in Rust, and built with [cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
Check out the code and run `cargo build`