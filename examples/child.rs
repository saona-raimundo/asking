//! Test a command-line application that uses user input!
//!
//! This examples shows the use of
//! - Testing

use assert_cmd::cmd::Command;

fn main() -> eyre::Result<()> {
    // Setting up a run
    let mut cmd = Command::cargo_bin("child_app")?;

    // Write all input
    // Note: the run has to end, so write all answers here!
    cmd.write_stdin("no");

    // Test the run
    let output = cmd.unwrap();
    assert_eq!(
        output.stdout,
        Vec::<u8>::from("Do you like testing? Oh no!")
    );

    Ok(())
}
