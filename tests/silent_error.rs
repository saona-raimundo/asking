//! Tests the matching example `silent_error`.

use assert_cmd::cmd::Command;

#[test]
fn input() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//silent_error")?
        .write_stdin("\n\n5")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "Give me a number bigger than 3, please. I will not tell you anything else.\n".to_string()
    );

    Ok(())
}
