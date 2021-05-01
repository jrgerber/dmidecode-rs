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

#[test]
fn test_dmi_str_no_value() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(CLI_COMMAND)?;
    cmd.arg("-s");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("requires a value"));
    Ok(())
}

#[test]
fn test_dmi_str_known_unknown_keyword() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin(CLI_COMMAND)?;
    cmd1.arg("-s").arg("bios-version");
    cmd1.assert().success();

    let mut cmd2 = Command::cargo_bin(CLI_COMMAND)?;
    cmd2.arg("-s").arg("invalid");
    cmd2.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid value"));

    Ok(())
}

#[test]
fn test_oem_string_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin(CLI_COMMAND)?;
    cmd1.arg("--oem-string").arg("0");
    cmd1.assert()
        .failure()
        .stderr(predicate::str::contains("string number 0"));

    let mut cmd2 = Command::cargo_bin(CLI_COMMAND)?;
    cmd2.arg("--oem-string").arg("foo");
    cmd2.assert()
        .failure()
        .stderr(predicate::str::contains("string number foo"));

    Ok(())
}

#[test]
fn test_oem_string_valid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin(CLI_COMMAND)?;
    cmd1.arg("--oem-string").arg("1");
    cmd1.assert().success();

    let mut cmd2 = Command::cargo_bin(CLI_COMMAND)?;
    cmd2.arg("--oem-string").arg("count");
    cmd2.assert().success();

    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
#[test]
fn test_no_sysfs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin(CLI_COMMAND)?;
    cmd1.arg("--no-sysfs");
    cmd1.assert().success();

    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
#[test]
fn test_dev_mem() -> Result<(), Box<dyn std::error::Error>> {
    // test good path to /dev/mem
    let mut cmd = Command::cargo_bin(CLI_COMMAND)?;
    cmd.arg("--no-sysfs").arg("--dev-mem").arg("/dev/mem");
    cmd.assert().success();

    // test bad path to /dev/memx
    cmd = Command::cargo_bin(CLI_COMMAND)?;
    cmd.arg("--no-sysfs").arg("--dev-mem").arg("/dev/memx");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}
