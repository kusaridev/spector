//! A CLI tool for validating supply chain metadata documents.
//!
//! This tool currently supports validating In-Toto v1 documents with
//! SLSA Provenance v1 predicates.

use std::process;

use anyhow::anyhow;
use clap::Parser;
use spector::models::intoto::{predicate::Predicate, statement::InTotoStatementV1};

/// The top-level CLI parser.
#[derive(Parser)]
#[clap(
    version = "0.0.1",
    about = "A tool for validating supply chain metadata documents"
)]
struct Spector {
    #[clap(subcommand)]
    command: Command,
}

/// The available subcommands.
#[derive(Parser)]
enum Command {
    Validate(Validate),
}

/// The `validate` subcommand.
/// Currently only supports validating documents.
#[derive(Parser)]
struct Validate {
    #[clap(subcommand)]
    document: DocumentSubCommand,
}

/// The supported document types.
/// Currently only supports In-Toto v1 attestation statement documents.
#[derive(Parser)]
enum DocumentSubCommand {
    InTotoV1(InTotoV1),
}

/// The In-Toto v1 document subcommand.
#[derive(Parser)]
struct InTotoV1 {
    #[clap(subcommand)]
    predicate_subcommand: PredicateSubcommand,
}

/// The supported predicate types for In-Toto v1 documents.
/// Currently supports only SLSA Provenance v1 predicates.
#[derive(Parser)]
enum PredicateSubcommand {
    SLSAProvenanceV1(SLSAProvenanceV1),
}

/// The SLSA Provenance v1 predicate subcommand.
/// Currently only supports validating files.
#[derive(Parser)]
struct SLSAProvenanceV1 {
    #[clap(short, long, required = true)]
    file: String,
}

/// Validates the specified document.
fn validate_cmd(validate: Validate) -> Result<(), Box<dyn std::error::Error>> {
    match validate.document {
        DocumentSubCommand::InTotoV1(in_toto) => handle_intoto_v1(in_toto),
    }
}

/// Handles In-Toto v1 documents.
fn handle_intoto_v1(in_toto: InTotoV1) -> Result<(), Box<dyn std::error::Error>> {
    match in_toto.predicate_subcommand {
        PredicateSubcommand::SLSAProvenanceV1(sl) => handle_slsa_provenance_v1(sl),
    }
}

/// Handles SLSA Provenance v1 predicates in In-Toto v1 documents.
///
/// Prints the document if valid, otherwise prints an error message
/// with the unexpected predicate type or JSON parsing error.
fn handle_slsa_provenance_v1(sl: SLSAProvenanceV1) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = std::fs::read_to_string(sl.file)?;
    let result = serde_json::from_str::<InTotoStatementV1>(&json_str);

    match result {
        Ok(statement) => {
            let pretty_json = serde_json::to_string_pretty(&statement)?;

            match statement.predicate {
                Predicate::SLSAProvenanceV1(_) => {
                    println!("Valid InTotoV1 SLSAProvenanceV1 document");
                    println!("Document: {}", &pretty_json);
                    Ok(())
                }
                _ => {
                    eprintln!("Unexpected predicateType: {:?}", statement.predicate_type.as_str());
                    eprintln!("Document: {}", &pretty_json);
                    Err(anyhow!("Unexpected predicateType: {:?}", statement.predicate_type.as_str()).into())
                }
            }
        }
        Err(err) => {
            // TODO(mlieberman85): Figure out how to add all the fields that are incorrect between a valid SLSA statement and the one that is being validated.
            // Right now it only prints the first error.
            eprintln!("Error parsing JSON: {}", err);
            Err(Box::new(err))
        }
    }
}

fn main() {
    let opts: Spector = Spector::parse();
    match opts.command {
        Command::Validate(validate) => {
            if let Err(_) = validate_cmd(validate) {
                process::exit(1);
            }
        }
    }
}
