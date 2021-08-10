//! Tests the matching example `yn`.

use assert_cmd::cmd::Command;

#[test]
fn default_value() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//yn")?
        .write_stdin("\n")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "Shall I continue? (you have 5 seconds to answer) Super!\n".to_string()
    );

    Ok(())
}

#[test]
fn input_false() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//yn")?
        .write_stdin("false\n")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "Shall I continue? (you have 5 seconds to answer) Okay, shutting down...\n".to_string()
    );

    Ok(())
}
