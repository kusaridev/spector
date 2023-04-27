use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn test_valid_slsa_provenance_v1_document() {
    let mut cmd = Command::cargo_bin("spector").unwrap();
    let fixture = fixture_path("slsa_provenance_v1.json");

    cmd.args(&[
        "validate",
        "in-toto-v1",
        "slsa-provenance-v1",
        "--file",
        fixture.to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains(
        "Valid InTotoV1 SLSAProvenanceV1 document",
    ));
}

#[test]
fn test_invalid_slsa_provenance_v1_document() {
    let mut cmd = Command::cargo_bin("spector").unwrap();
    let fixture = fixture_path("slsa_provenance_v1_invalid.json");

    cmd.args(&[
        "validate",
        "in-toto-v1",
        "slsa-provenance-v1",
        "--file",
        fixture.to_str().unwrap(),
    ])
    .assert()
    .failure()
    .stderr(predicate::str::contains(
        "Error parsing JSON: missing field `buildType`",
    ));
}

#[test]
fn test_invalid_predicate_slsa_provenance_v1_document() {
    let mut cmd = Command::cargo_bin("spector").unwrap();
    let fixture = fixture_path("slsa_provenance_v1_invalid_predicate.json");

    cmd.args(&[
        "validate",
        "in-toto-v1",
        "slsa-provenance-v1",
        "--file",
        fixture.to_str().unwrap(),
    ])
    .assert()
    .failure()
    .stderr(predicate::str::contains(
        "Unexpected predicateType: \"https://slsa.dev/provenance/v12\"",
    ));
}
