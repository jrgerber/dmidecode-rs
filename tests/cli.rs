use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;


static CLI_COMMAND: &str = "dmidecode";

#[test]
fn test_command_run() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(CLI_COMMAND)?;
    Ok(())
}

#[test]
fn test_dump_bin_no_value() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CLI_COMMAND)?;

    cmd.arg("--dump-bin");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("requires a value"));
    Ok(())
}

#[test]
fn test_read_bin_no_value() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CLI_COMMAND)?;

    cmd.arg("--from-dump");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("requires a value"));
    Ok(())
}

#[test]
fn test_dump_bin_read_bin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd_dump = Command::cargo_bin(CLI_COMMAND)?;
    let dir = tempdir()?;

    let filename = dir.path().join("raw.bin");
    let filename_str = filename.to_str().unwrap();
    cmd_dump.arg("--dump-bin").arg(&filename_str);
    cmd_dump.assert().success();

    let mut cmd_read = Command::cargo_bin(CLI_COMMAND)?;
    cmd_read.arg("--from-dump").arg(&filename_str);
    cmd_read.assert().success();

    drop(filename);
    dir.close()?;
    Ok(())
}
