use assert_cmd::Command;
use predicates::prelude::*; // Used for writing assertions
use tempfile::tempdir;

#[test]
fn missing_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn db_path_only() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db /some/path");
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("USAGE"));

    assert_eq!(std::path::Path::new("/some/path").exists(), false);
    Ok(())
}

#[test]
fn db_path_with_create_if_missing_does_not_create() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db /some/path").arg("--create_if_missing");
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("USAGE"));

    assert_eq!(std::path::Path::new("/some/path").exists(), false);
    Ok(())
}

#[test]
fn basic_put_and_get() -> Result<(), Box<dyn std::error::Error>> {
    let path = tempdir()?;
    let key = "hello";
    let value = "world";
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("put")
        .arg(key)
        .arg(value);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("get").arg(key);
    cmd.assert()
        .success()
        .stdout(format!("{}\n", value))
        .code(0);
    Ok(())
}

#[test]
fn hex_put_and_get() -> Result<(), Box<dyn std::error::Error>> {
    let path = tempdir()?;
    let key = "68656c6c6f";
    let value = "776f726c64";
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("put")
        .arg("--hex")
        .arg(key)
        .arg(value);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db")
        .arg(path.path())
        .arg("get")
        .arg("--hex")
        .arg(key);
    cmd.assert()
        .success()
        .stdout(format!("{}\n", value))
        .code(0);
    Ok(())
}

#[test]
fn key_hex_put_and_get() -> Result<(), Box<dyn std::error::Error>> {
    let path = tempdir()?;
    let key = "68656c6c6f";
    let value = "world";
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("put")
        .arg("--key_hex")
        .arg(key)
        .arg(value);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db")
        .arg(path.path())
        .arg("get")
        .arg("--key_hex")
        .arg(key);
    cmd.assert()
        .success()
        .stdout(format!("{}\n", value))
        .code(0);
    Ok(())
}

#[test]
fn value_hex_put_and_get() -> Result<(), Box<dyn std::error::Error>> {
    let path = tempdir()?;
    let key = "hello";
    let value = "776f726c64";
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("put")
        .arg("--value_hex")
        .arg(key)
        .arg(value);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db")
        .arg(path.path())
        .arg("get")
        .arg("--value_hex")
        .arg(key);
    cmd.assert()
        .success()
        .stdout(format!("{}\n", value))
        .code(0);
    Ok(())
}

#[test]
fn basic_delete() -> Result<(), Box<dyn std::error::Error>> {
    let path = tempdir()?;
    let key = "hello";
    let value = "world";
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("put")
        .arg(key)
        .arg(value);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("get").arg(key);
    cmd.assert()
        .success()
        .stdout(format!("{}\n", value))
        .code(0);

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("delete").arg(key);
    cmd.assert().success().code(0);

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("get").arg(key);
    cmd.assert().success().stdout("Not Found???\n").code(0);
    Ok(())
}
