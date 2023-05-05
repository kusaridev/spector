//! A CLI tool for validating supply chain metadata documents.
//!
//! This tool currently supports validating In-Toto v1 documents with
//! SLSA Provenance v1 predicates.
//! TODO(mlieberman85): The CLI commands and args could probably be generalized better to minimize duplication.

use std::{path::PathBuf, process};

use anyhow::Result;
use clap::{Parser, ValueEnum};
use serde::de::DeserializeOwned;
use serde_json::Value;
use spector::{
    models::intoto::{
        predicate::Predicate, provenance::SLSAProvenanceV1Predicate, statement::InTotoStatementV1,
    },
    validate::{self, Validator},
};
use typify::{TypeSpace, TypeSpaceSettings};

#[derive(Parser)]
#[clap(
    version = "0.0.1",
    about = "A tool for validating supply chain metadata documents"
)]
struct Spector {
    #[clap(subcommand)]
    command: Command,
}

// The available subcommands
#[derive(Parser)]
enum Command {
    Validate(Validate),
    SchemaGenerate(SchemaGenerate),
    CodeGenerate(CodeGenerate),
    SchemaValidate(SchemaValidate),
}

// The `code-generate` subcommand
#[derive(Parser)]
struct CodeGenerate {
    #[clap(subcommand)]
    codegen: CodeGenerateSubCommand,
}

// The supported code generation types
#[derive(Parser)]
enum CodeGenerateSubCommand {
    JsonSchema(JsonSchema),
}

#[derive(Parser)]
struct JsonSchema {
    /// Path to the file to generate a schema for
    /// TODO(mlieberman85): Make this optional once we support stdin
    /// TODO(mlieberman85): Figure out how to generalize this to all applicable subcommands
    #[clap(value_parser)]
    #[clap(long, short, required = true)]
    file: PathBuf,
}

// The `validate` subcommand
#[derive(Parser)]
struct Validate {
    #[clap(subcommand)]
    document: ValidateDocumentSubCommand,
}

// The `generate` subcommand
#[derive(Parser)]
struct SchemaGenerate {
    #[clap(subcommand)]
    document: GenerateDocumentSubCommand,
}

// The `schema-validate` subcommand
#[derive(Parser)]
struct SchemaValidate {
    /// Path to the schema file
    #[clap(value_parser)]
    schema: PathBuf,

    /// Path to the file to validate
    // TODO(mlieberman85): Make this optional once we support stdin
    #[clap(value_parser)]
    #[clap(long, short, required = true)]
    file: PathBuf,
}

// The supported validate document types
#[derive(Parser)]
enum ValidateDocumentSubCommand {
    InTotoV1(ValidateInTotoV1),
}

// The supported schema generate document types
#[derive(Parser)]
enum GenerateDocumentSubCommand {
    InTotoV1(GenerateInTotoV1),
}

// The In-Toto v1 validate document subcommand
// TODO(mlieberman85): Add support for stdin
// TODO(mlieberman85): Figure out a way to ensure file is added onto all document subcommands
#[derive(Parser)]
struct ValidateInTotoV1 {
    /// Predicate type for In-Toto v1 documents
    #[arg(value_enum)]
    #[clap(long, short)]
    predicate: Option<PredicateOption>,

    /// Path to the file to validate
    #[clap(value_parser)]
    #[clap(long, short, required = true)]
    file: PathBuf,
}

// The In-Toto v1 generate schema subcommand
#[derive(Parser)]
struct GenerateInTotoV1 {
    /// Predicate type for In-Toto v1 documents
    #[arg(value_enum)]
    #[clap(long, short)]
    predicate: Option<PredicateOption>,
}

#[derive(Copy, Clone, ValueEnum)]
enum PredicateOption {
    SLSAProvenanceV1,
}

#[derive(Parser)]
struct SLSAProvenanceV1 {}

/// Validates the specified document.
fn validate_cmd(validate: Validate) -> Result<()> {
    //let file_str = std::fs::read_to_string(&validate.file)?;
    match validate.document {
        ValidateDocumentSubCommand::InTotoV1(in_toto) => validate_intoto_v1(in_toto),
    }
}

/// Generates a schema for the specified type.
fn generate_cmd(generate: SchemaGenerate) -> Result<()> {
    match generate.document {
        GenerateDocumentSubCommand::InTotoV1(in_toto) => generate_intoto_v1(in_toto),
    }
}

/// Handles validation for In-Toto v1 documents.
fn validate_intoto_v1(in_toto: ValidateInTotoV1) -> Result<()> {
    let file_str = std::fs::read_to_string(&in_toto.file)?;
    let result = serde_json::from_str::<InTotoStatementV1>(&file_str);

    match result {
        Ok(statement) => {
            let pretty_json = serde_json::to_string_pretty(&statement)?;
            match statement.predicate {
                Predicate::SLSAProvenanceV1(_) => match in_toto.predicate {
                    Some(PredicateOption::SLSAProvenanceV1) => {
                        println!("Valid InTotoV1 SLSAProvenanceV1 document");
                        println!("Document: {}", &pretty_json);
                        Ok(())
                    }
                    // TODO(mlieberman85): Uncomment below once additional predicate types are supported.
                    /*Some(_) => {
                        eprintln!("Invalid InTotoV1 SLSAProvenanceV1 document");
                        eprintln!("Document: {}", &pretty_json);
                        Err(anyhow::anyhow!(
                            "Invalid InTotoV1 SLSAProvenanceV1 document"
                        ))
                    }*/
                    None => {
                        println!("Valid InTotoV1 SLSAProvenanceV1 document");
                        println!("Document: {}", &pretty_json);
                        Ok(())
                    }
                },
                _ => {
                    if let Some(PredicateOption::SLSAProvenanceV1) = in_toto.predicate {
                        eprintln!("Invalid InTotoV1 SLSAProvenanceV1 document");
                        eprintln!("Document: {}", &pretty_json);
                        Err(anyhow::anyhow!(
                            "Unexpected predicateType: {:?}",
                            statement.predicate_type.as_str()
                        ))
                    } else {
                        println!(
                            "Unknown predicateType: {:?}",
                            statement.predicate_type.as_str()
                        );
                        println!("Document: {}", &pretty_json);
                        Ok(())
                    }
                }
            }
        }
        Err(err) => {
            // TODO(mlieberman85): Figure out how to add all the fields that are incorrect between a valid SLSA statement and the one that is being validated.
            // Right now it only prints the first error.
            eprintln!("Error parsing JSON: {}", err);
            Err(err.into())
        }
    }
}

/// Handles generation of schemas for In-Toto v1 documents.
fn generate_intoto_v1(in_toto: GenerateInTotoV1) -> Result<()> {
    match in_toto.predicate {
        Some(PredicateOption::SLSAProvenanceV1) => print_schema::<SLSAProvenanceV1Predicate>(),
        None => print_schema::<InTotoStatementV1>(),
    }
}

/// Generates Rust code from a JSON schema file.
fn code_generate_cmd(cg: CodeGenerate) -> Result<()> {
    match cg.codegen {
        CodeGenerateSubCommand::JsonSchema(json_schema) => {
            let schema_str = std::fs::read_to_string(&json_schema.file)?;
            generate_rust_code(schema_str)
        }
    }
}

/// Generates Rust code from a JSON schema.
fn generate_rust_code(schema_str: String) -> Result<()> {
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&schema_str)?;
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema)?;

    let contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?)
    );
    println!("{}", contents);

    Ok(())
}

/// Prints a JSON schema for the given type T.
fn print_schema<T: serde::Serialize + schemars::JsonSchema>() -> Result<()> {
    let schema = schemars::schema_for!(T);
    println!("{}", serde_json::to_string_pretty(&schema)?);
    Ok(())
}

/// Handles validation of documents to JSON schemas.
///
/// Prints the document if valid, otherwise prints an error message
fn schema_validate_cmd<T: DeserializeOwned>(sv: SchemaValidate) -> Result<()> {
    let file_str = std::fs::read_to_string(&sv.file)?;
    let schema_str = std::fs::read_to_string(&sv.schema)?;
    let schema = serde_json::from_str::<serde_json::Value>(&schema_str)?;
    let validator = validate::JSONSchemaValidator::<Value>::new(&schema);
    let document = serde_json::from_str::<serde_json::Value>(&file_str)?;
    let result: std::result::Result<Value, anyhow::Error> = validator.validate(&document);

    match result {
        Ok(_) => {
            println!("Valid document based on JSON schema");
            match serde_json::from_value::<T>(document) {
                Ok(_) => {
                    println!("Document: {}", &file_str);
                    Ok(())
                }
                Err(err) => {
                    eprintln!("Error validating document against Serde structs: {}", err);
                    Err(err.into())
                }
            }
        }
        Err(err) => {
            eprintln!("Error validating document against JSON schema: {}", err);
            Err(err.into())
        }
    }
}

fn main() {
    let opts: Spector = Spector::parse();
    match opts.command {
        Command::Validate(validate) => {
            if let Err(e) = validate_cmd(validate) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Command::SchemaGenerate(generate) => {
            if let Err(e) = generate_cmd(generate) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Command::SchemaValidate(sv) => {
            // TODO(mlieberman85): Update this once we support validating against the JSON schema AND the
            // Serde structs at the same time.
            if let Err(e) = schema_validate_cmd::<Value>(sv) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Command::CodeGenerate(cg) => {
            if let Err(e) = code_generate_cmd(cg) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}
