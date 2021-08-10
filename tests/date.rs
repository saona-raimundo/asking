//! Tests the matching example `date`.

use assert_cmd::cmd::Command;

#[test]
fn eof() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//date")?
        .write_stdin("\n2200-01-01")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    let mut messages = std::str::from_utf8(&output.stdout)?.to_string();
    messages.truncate(82);
    let mut expected = "\
            Please input your awaited date: \
            Use a %Y-%m-%d format please.\n\
            Thank you!\
            There are {} weeks, and {} days left!\
        "
    .to_string();
    expected.truncate(82);
    assert_eq!(messages, expected);

    Ok(())
}

#[test]
fn input() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//date")?
        .write_stdin("2200-01-01\n")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    let mut messages = std::str::from_utf8(&output.stdout)?.to_string();
    messages.truncate(42);

    assert_eq!(
        messages,
        "\
            Please input your awaited date: \
            Thank you!\
        "
        .to_string()
    );

    Ok(())
}
