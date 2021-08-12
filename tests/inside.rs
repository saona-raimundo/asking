//! Tests the matching example `inside`.

use assert_cmd::cmd::Command;

#[test]
fn eof() -> eyre::Result<()> {
    let output = Command::cargo_bin("examples//inside")?
        .write_stdin("\n1\n5")
        .timeout(std::time::Duration::from_secs(1))
        .unwrap();

    let messages = std::str::from_utf8(&output.stdout)?;
    let expected = "\
            What level is your Pokemon?\
            Sorry, I need this. Give me an answer please.\n\
            It should be at least level 5 and most 100, right? Try again.\n\
            Level 5! That is awesome!\n\
        ";
    assert_eq!(messages, expected);

    Ok(())
}
