//! Tests the matching example `app`.

use assert_cmd::cmd::Command;

#[test]
fn eof() -> eyre::Result<()> {
    Command::cargo_bin("examples//app")?
        .write_stdin("\n")
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .failure();

    Ok(())
}

#[test]
fn input_false() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//app")?
        .write_stdin("false\n")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "Do you like testing? Oh no!".to_string()
    );

    Ok(())
}

#[test]
fn input_twice() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//app")?
        .write_stdin("m...\nyes")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "Do you like testing? Please use format y/n. Try again: Great!".to_string()
    );

    Ok(())
}
