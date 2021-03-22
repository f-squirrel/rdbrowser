use assert_cmd::Command;
use predicates::prelude::*; // Used for writing assertions
use tempfile::tempdir;
extern crate glob;
use glob::glob;
use std::io::Read;
use std::io::Write;

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
    cmd.assert().success().stdout("OK\n");

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
    let key = "0x68656c6c6f";
    let value = "776f726c64";
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("put")
        .arg("--hex")
        .arg(key)
        .arg(value);
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db")
        .arg(path.path())
        .arg("get")
        .arg("--hex")
        .arg(key);
    cmd.assert()
        .success()
        .stdout(format!("0x{}\n", value))
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
    cmd.assert().success().stdout("OK\n");

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
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db")
        .arg(path.path())
        .arg("get")
        .arg("--value_hex")
        .arg(key);
    cmd.assert()
        .success()
        .stdout(format!("0x{}\n", value))
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
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("get").arg(key);
    cmd.assert()
        .success()
        .stdout(format!("{}\n", value))
        .code(0);

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("delete").arg(key);
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("get").arg(key);
    cmd.assert().success().stderr("Not Found\n").code(0);
    Ok(())
}

#[test]
fn multiple_word_put_get_delete() -> Result<(), Box<dyn std::error::Error>> {
    let path = tempdir()?;
    let key = "aaaa bbbb cccc";
    let value = "dddd eeee fffff";
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("put")
        .arg(key)
        .arg(value);
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("get").arg(key);
    cmd.assert()
        .success()
        .stdout(format!("{}\n", value))
        .code(0);

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("delete").arg(key);
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("get").arg(key);
    cmd.assert().success().stderr("Not Found\n").code(0);
    Ok(())
}

#[test]
fn batchput_and_get() -> Result<(), Box<dyn std::error::Error>> {
    let path = tempdir()?;
    let kv = [
        "1111", "1111", "2222", "2222", "3333", "3333", "4444", "4444",
    ];
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("batchput")
        .args(&kv);
    cmd.assert().success().stdout("OK\n");

    for i in (0..kv.len()).step_by(2) {
        let mut cmd = Command::cargo_bin("rdbrowser")?;
        cmd.arg("--db").arg(path.path()).arg("get").arg(kv[i]);
        cmd.assert()
            .success()
            .stdout(format!("{}\n", kv[i + 1]))
            .code(0);
    }
    Ok(())
}

#[test]
fn batchput_multi_word_and_get() -> Result<(), Box<dyn std::error::Error>> {
    let path = tempdir()?;
    let kv = ["hey hey", "hoy hoy", "bla bla", "tra tra"];
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("batchput")
        .args(&kv);
    cmd.assert().success().stdout("OK\n");

    for i in (0..kv.len()).step_by(2) {
        let mut cmd = Command::cargo_bin("rdbrowser")?;
        cmd.arg("--db").arg(path.path()).arg("get").arg(kv[i]);
        cmd.assert()
            .success()
            .stdout(format!("{}\n", kv[i + 1]))
            .code(0);
    }
    Ok(())
}

#[test]
fn batchput_and_get_hex() -> Result<(), Box<dyn std::error::Error>> {
    {
        let path = tempdir()?;
        let kv = [
            "31313131", "1111", "32323232", "2222", "33333333", "3333", "34343434", "4444",
        ];
        let mut cmd = Command::cargo_bin("rdbrowser")?;
        cmd.arg("--create_if_missing")
            .arg("--db")
            .arg(path.path())
            .arg("batchput")
            .arg("--key_hex")
            .args(&kv);
        cmd.assert().success().stdout("OK\n");

        for i in (0..kv.len()).step_by(2) {
            let mut cmd = Command::cargo_bin("rdbrowser")?;
            cmd.arg("--db")
                .arg(path.path())
                .arg("get")
                .arg("--key_hex")
                .arg(kv[i]);
            cmd.assert()
                .success()
                .stdout(format!("{}\n", kv[i + 1]))
                .code(0);
        }
    }
    {
        let path = tempdir()?;
        let kv = [
            "1111", "31313131", "2222", "32323232", "3333", "33333333", "4444", "34343434",
        ];
        let mut cmd = Command::cargo_bin("rdbrowser")?;
        cmd.arg("--create_if_missing")
            .arg("--db")
            .arg(path.path())
            .arg("batchput")
            .arg("--value_hex")
            .args(&kv);
        cmd.assert().success().stdout("OK\n");

        for i in (0..kv.len()).step_by(2) {
            let mut cmd = Command::cargo_bin("rdbrowser")?;
            cmd.arg("--db")
                .arg(path.path())
                .arg("get")
                .arg("--value_hex")
                .arg(kv[i]);
            cmd.assert()
                .success()
                .stdout(format!("0x{}\n", kv[i + 1]))
                .code(0);
        }
    }
    {
        let path = tempdir()?;
        let kv = [
            "31313131", "31313131", "32323232", "32323232", "33333333", "33333333", "34343434",
            "34343434",
        ];
        let mut cmd = Command::cargo_bin("rdbrowser")?;
        cmd.arg("--create_if_missing")
            .arg("--db")
            .arg(path.path())
            .arg("batchput")
            .arg("--hex")
            .args(&kv);
        cmd.assert().success().stdout("OK\n");

        for i in (0..kv.len()).step_by(2) {
            let mut cmd = Command::cargo_bin("rdbrowser")?;
            cmd.arg("--db")
                .arg(path.path())
                .arg("get")
                .arg("--hex")
                .arg(kv[i]);
            cmd.assert()
                .success()
                .stdout(format!("0x{}\n", kv[i + 1]))
                .code(0);
        }
    }
    Ok(())
}

#[test]
fn batchput_wrong_input() -> Result<(), Box<dyn std::error::Error>> {
    let path = tempdir()?;
    let kv = ["1111", "1111", "2222"];
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("batchput")
        .args(&kv);
    cmd.assert().failure().stderr(format!(
        "Failed: Keys and values bnumber has to be even, given {}\n",
        kv.len()
    ));

    Ok(())
}

#[test]
fn basic_scan() -> Result<(), Box<dyn std::error::Error>> {
    let kv = [
        "1111", "1111", "2222", "2222", "3333", "3333", "4444", "4444",
    ];
    let path = tempdir()?;
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("batchput")
        .args(&kv);
    cmd.assert().success().stdout("OK\n");

    let mut expected_output = String::new();
    for i in (0..kv.len()).step_by(2) {
        expected_output.push_str(format!("{} : {}\n", kv[i], kv[i + 1]).as_str());
    }

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan");
    cmd.assert().success().stdout(expected_output);

    cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan")
        .arg("--value_hex");
    cmd.assert()
        .success()
        .stdout("1111 : 0x31313131\n2222 : 0x32323232\n3333 : 0x33333333\n4444 : 0x34343434\n");

    cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan")
        .arg("--key_hex");
    cmd.assert()
        .success()
        .stdout("0x31313131 : 1111\n0x32323232 : 2222\n0x33333333 : 3333\n0x34343434 : 4444\n");

    cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan")
        .arg("--hex");
    cmd.assert()
            .success()
            .stdout("0x31313131 : 0x31313131\n0x32323232 : 0x32323232\n0x33333333 : 0x33333333\n0x34343434 : 0x34343434\n");
    Ok(())
}

#[test]
fn scan_from_to() -> Result<(), Box<dyn std::error::Error>> {
    let kv = [
        "1111", "1111", "2222", "2222", "3333", "3333", "4444", "4444",
    ];
    let path = tempdir()?;
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("batchput")
        .args(&kv);
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan")
        .arg("--from")
        .arg("2222")
        .arg("--to")
        .arg("4444");
    cmd.assert().success().stdout("2222 : 2222\n3333 : 3333\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan")
        .arg("--from")
        .arg("2")
        .arg("--to")
        .arg("4");
    cmd.assert().success().stdout("2222 : 2222\n3333 : 3333\n");

    cmd.assert().success().stdout("2222 : 2222\n3333 : 3333\n");
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan")
        .arg("--to")
        .arg("4444");
    cmd.assert()
        .success()
        .stdout("1111 : 1111\n2222 : 2222\n3333 : 3333\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan")
        .arg("--key_hex")
        .arg("--from")
        .arg("32323232")
        .arg("--to")
        .arg("34343434");
    cmd.assert()
        .success()
        .stdout("0x32323232 : 2222\n0x33333333 : 3333\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan")
        .arg("--value_hex")
        .arg("--from")
        .arg("2222")
        .arg("--to")
        .arg("4444");
    cmd.assert()
        .success()
        .stdout("2222 : 0x32323232\n3333 : 0x33333333\n");

    Ok(())
}

#[test]
fn delete_range() -> Result<(), Box<dyn std::error::Error>> {
    let kv = [
        "1111", "1111", "2222", "2222", "3333", "3333", "4444", "4444",
    ];
    let path = tempdir()?;
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("batchput")
        .args(&kv);
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("deleterange")
        .arg("2222")
        .arg("4444");
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan");
    cmd.assert().success().stdout("1111 : 1111\n4444 : 4444\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("deleterange")
        .arg("1111")
        .arg("4");
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("scan");
    cmd.assert().success().stdout("4444 : 4444\n");
    Ok(())
}

#[test]
fn checkconsistency() -> Result<(), Box<dyn std::error::Error>> {
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
    cmd.assert().success().stdout("OK\n");

    cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("put")
        .arg("hello1")
        .arg("value1");
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("get").arg(key);
    cmd.assert()
        .success()
        .stdout(format!("{}\n", value))
        .code(0);

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("checkconsistency");
    cmd.assert().success().stdout("OK\n").code(0);

    let paths: Vec<_> = glob(path.path().join("*.sst").to_str().unwrap())
        .unwrap()
        .collect();
    assert_ne!(paths.len(), 0);

    let sst_file_path = paths[0].as_ref().unwrap().to_str().unwrap();
    std::fs::remove_file(sst_file_path)?;
    let mut sst_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(sst_file_path)
        .unwrap();
    sst_file.write_all(b"EVIL")?;
    sst_file.flush()?;
    sst_file.sync_all()?;
    drop(sst_file);

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("checkconsistency");
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Failed: Corruption:"));

    std::fs::remove_file(sst_file_path)?;
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db").arg(path.path()).arg("checkconsistency");
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Failed: IO error:"));
    Ok(())
}

#[test]
fn dump() -> Result<(), Box<dyn std::error::Error>> {
    let path = tempdir()?;
    let kv = [
        "1111", "1111", "2222", "2222", "3333", "3333", "4444", "4444",
    ];
    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--create_if_missing")
        .arg("--db")
        .arg(path.path())
        .arg("batchput")
        .args(&kv);
    cmd.assert().success().stdout("OK\n");

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db")
        .arg(path.path())
        .arg("dump")
        .arg("--output")
        .arg("./dump.txt");
    cmd.assert().success();

    let mut f = std::fs::File::open("./dump.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    assert_eq!(
        contents,
        r"1111 ==> 1111
2222 ==> 2222
3333 ==> 3333
4444 ==> 4444
"
    );

    let mut cmd = Command::cargo_bin("rdbrowser")?;
    cmd.arg("--db")
        .arg(path.path())
        .arg("dump")
        .arg("--max_keys")
        .arg("2")
        .arg("--output")
        .arg("./dump.txt");
    cmd.assert().success();

    let mut f = std::fs::File::open("./dump.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    assert_eq!(
        contents,
        r"1111 ==> 1111
2222 ==> 2222
"
    );
    Ok(())
}
