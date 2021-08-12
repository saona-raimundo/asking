//! Tests the matching example `select`.

use assert_cmd::cmd::Command;

#[test]
fn input() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//select")?
        .write_stdin("\nA")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "\
            Which option should I go for? \
            Options available [\"A\", \"B\"]\n\
            Try again: \
            A it is!\n\
        "
        .to_string()
    );

    Ok(())
}
