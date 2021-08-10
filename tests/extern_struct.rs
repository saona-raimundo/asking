//! Tests the matching example `extern_struct`.

use assert_cmd::cmd::Command;

#[test]
fn eof() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//extern_struct")?
        .write_stdin("\n1,1")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    let messages = std::str::from_utf8(&output.stdout)?;
    let expected = "\
            Please input a point in 2D space.\n\
            Use a `x,y` format please :)\n\
            Parsing failed.\n\
            Thank you! Your point is (1.0, 1.0)\
        ";
    assert_eq!(messages, expected);

    Ok(())
}
