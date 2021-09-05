//! Give only 10 seconds to answer!
//!
//! This examples shows how to use:
//! - `timeout`
//! - Error handling

use asking::error::ProcessingError;
use std::time::Duration;

fn main() {
    let result = async_std::task::block_on(
        asking::yn()
            .message("Shall I continue? (you have 10 seconds to answer) ")
            .feedback(|b| {
                if *b {
                    "Great!\n".to_string()
                } else {
                    "Too bad\n".to_string()
                }
            })
            .help("You can do it!\n")
            .required()
            .timeout(Duration::from_secs(10_u64))
            .ask(),
    );

    match result {
        Ok(true) => println!("Super!"),
        Ok(false) => println!("Okay, shutting down..."),
        Err(ProcessingError::Timeout { .. }) => {
            println!("\nI think you are not here, I will continue :)")
        }
        Err(ProcessingError::Io { .. }) => eprintln!("\nFailed to read line."),
        _ => unreachable!(),
    }
}
