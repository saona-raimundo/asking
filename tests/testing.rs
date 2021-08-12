//! Tests the matching example `testing`.

use assert_cmd::cmd::Command;

#[test]
fn eof() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//testing")?
        // .write_stdin("\n1\n5")
        // .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    let messages = std::str::from_utf8(&output.stdout)?;
    let expected = "";
    assert_eq!(messages, expected);

    Ok(())
}
