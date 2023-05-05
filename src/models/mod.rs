mod helpers;
pub mod intoto;
pub mod sbom;

// NOTE: Throughout the models, several of the Options have a serde attribute of `skip_serializing_if = "Option::is_none"`.
// This is required to ensure that the JSON schema output is correct. Without this, it will default the value to "null" and
// other things like Rust codegen from the schema will not work correctly.
