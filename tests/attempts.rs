//! Tests the matching example `attempts`.

use assert_cmd::cmd::Command;

#[test]
fn eof() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//attempts")?
        .write_stdin("\n\n\n\n")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "Do you like me? (y/n)\nLet's try again, do you like me?\nOkay, last chance! do you like me... a bit?\n".to_string()
    );
    assert_eq!(
        std::str::from_utf8(&output.stderr)?,
        "You did not manage to answer the question.\n".to_string()
    );

    Ok(())
}

#[test]
fn input_false() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//attempts")?
        .write_stdin("false\n")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "Do you like me? (y/n)\nDon't worry, we can get know each other with time.\nWe are done here! Go and play!\n".to_string()
    );

    Ok(())
}

#[test]
fn input_true() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//attempts")?
        .write_stdin("yes\n")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "Do you like me? (y/n)\nI like you too!\nWe are done here! Go and play!\n".to_string()
    );

    Ok(())
}
