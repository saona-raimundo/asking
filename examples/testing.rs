//! Test a command-line application that uses user input!
//!
//! Test a run of the example `app`.
//!
//! This examples shows the use of
//! - Testing

use assert_cmd::cmd::Command;

fn main() -> eyre::Result<()> {
    // Setting up a run
    let mut cmd = Command::cargo_bin("app")?; // Example `app` needs to be built before running this example

    // Write all input
    // Note: the run has to end, so write all answers here!
    cmd.write_stdin("maybe...\nno");

    // Make sure not to fall in a loop.
    cmd.timeout(std::time::Duration::from_secs(1));

    // Test the run
    let output = cmd.unwrap();
    assert_eq!(
        std::str::from_utf8(&output.stdout)?,
        "Do you like testing? Please use format y/n. Try again: Oh no!".to_string()
    );

    Ok(())
}
