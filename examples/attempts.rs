//! Give only some attempts to answer!
//!
//! This examples shows how to use:
//! - `attempts`
//! - Error handling

use asking::error::ProcessingError;

fn main() {
    let result = async_std::task::block_on(
        asking::yn()
            .message("Do you like me? (y/n)\n")
            .attempts_with_feedback(3, |num| match num {
                3 => "".to_string(),
                2 => "Let's try again, do you like me?\n".to_string(),
                1 => "Okay, last chance! do you like me... a bit?\n".to_string(),
                _ => unreachable!(),
            })
            .feedback(|value| match value {
                true => "I like you too!\n".to_string(),
                false => "Don't worry, we can get know each other with time.\n".to_string(),
            })
            .ask(),
    );

    match result {
        Ok(_) => println!("We are done here! Go and play!"),
        Err(ProcessingError::NoMoreAttempts) => {
            eprintln!("You did not manage to answer the question.")
        }
        Err(ProcessingError::Io { .. }) => eprintln!("Failed to read line."),
        _ => unreachable!(),
    }
}
