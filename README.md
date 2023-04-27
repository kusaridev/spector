# Spector
Spector is both tooling and a library for the generation, validation and verification of supply chain metadata documents and frameworks.  Many tools generate non-compliant SBOMs or attestations.  It currently supports
* [SLSA 1.0 Provenance](https://slsa.dev/provenance/v1)
* [in-toto 1.0 Statement](https://github.com/in-toto/attestation/blob/v1.0/spec/v1.0/statement.md)

## Library
You can include spector as a library when writing generators for SLSA or other supported document types.  It can provide the serialization & deserialization for SLSA attestations, assuring that they are properly to spec before you go further in the process.

## Tooling
Spector is still early on and doesn't have an official release yet.

You can run:
```shell
cargo run validate in-toto-v1 slsa-provenance-v1 --file tests/fixtures/slsa_provenance_v1.json
```

You can replace the `slsa_provenance_v1.json` with another in-toto statement and even an invalid one to verify the correctness of the document. 

## Developing and Building
Spector is written in Rust, and built with [cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
Check out the code and run `cargo build` or `cargo test`.
