use std::error::Error;

use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("rust-grep")?;

    cmd.arg("Wojciech").arg("IdoNotExist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    return Ok(());
}

#[test]
fn pointed_at_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("rust-grep")?;

    cmd.arg("Wojciech").arg("./tests/file.txt");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1 Wojciech\n3 Wojciech"));

    return Ok(());
}

#[test]
fn pointed_at_directory() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("rust-grep")?;

    cmd.arg("Wojciech").arg("./tests/files");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Filename: \"tests/files/first.txt\"\n3 Wojciech",
        ))
        .stdout(predicate::str::contains(
            "Filename: \"tests/files/second/second.txt\"\n2 Wojciech",
        ));

    return Ok(());
}
