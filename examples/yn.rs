//! Yes/no question.
//!
//! This examples shows the use of
//! - `yn`
//! - `timeout`
//! - Error handling

use asking::error::ProcessingError;
use std::time::Duration;

fn main() {
    let ans = asking::yn()
        .message("Shall I continue? (you have 5 seconds to answer) ")
        .help("Please use format y/n. Try again: ")
        .default_value(true)
        .timeout(Duration::from_secs(5_u64))
        .ask();

    match async_std::task::block_on(ans) {
        Ok(true) => println!("Super!"),
        Ok(false) => println!("Okay, shutting down..."),
        Err(ProcessingError::Timeout { .. }) => {
            println!("I think you are not here, I will continue :)")
        }
        Err(_) => eprintln!("Error with questionnaire, try again later"),
    }
}
