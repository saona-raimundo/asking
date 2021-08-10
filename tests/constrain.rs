//! Tests the matching example `constrain`.

use assert_cmd::cmd::Command;

#[test]
fn eof() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//constrain")?
        .write_stdin("\n2")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "\
            Please input a number between 2 and 4, that is not 3.\n\
            Please, try again: \
            You chose 2!\
        "
        .to_string()
    );

    Ok(())
}

#[test]
fn input_2() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//constrain")?
        .write_stdin("2\n")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "\
            Please input a number between 2 and 4, that is not 3.\n\
            You chose 2!\
        "
        .to_string()
    );

    Ok(())
}

#[test]
fn input_3() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//constrain")?
        .write_stdin("3\n2")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "\
            Please input a number between 2 and 4, that is not 3.\n\
            This value is not allowed.\n\
            Please, try again: \
            You chose 2!\
        "
        .to_string()
    );

    Ok(())
}
