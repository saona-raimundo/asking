//! We can use files as input and output!
//!
//! This example shows how to use:
//! - `reader`
//! - `writer`
//! - `File`s instead of the command line

use async_std::fs::OpenOptions;
use std::{fs::File, io::Write};

#[async_std::main]
async fn main() -> eyre::Result<()> {
    // Create files with answers
    {
        let mut file = File::create("examples\\file\\in.txt")?;
        write!(file, "{}", "false")?;
        File::create("examples\\file\\out.txt")?;
    }

    let file_in = OpenOptions::new()
        .read(true)
        .open("examples\\file\\in.txt")
        .await?;

    let file_out = OpenOptions::new()
        .write(true)
        .open("examples\\file\\out.txt")
        .await?;

    let question = asking::yn()
        .reader(file_in)
        .writer(file_out)
        .message("Shall I continue?\n")
        .repeat_help("Please use y/n format.\n")
        .str_test_with_msg(|s| s.len() > 0, "You can do it!")
        .ask();

    match question.await {
        Ok(true) => println!("Super!"),
        Ok(false) => println!("Okay, shutting down..."), // We know this is the answer!
        _ => eprintln!("Error with questionnaire, try again later"),
    }

    Ok(())
}
