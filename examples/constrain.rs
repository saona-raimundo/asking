//! Constrain the input.
//!
//! This examples shows how to use:
//! - `max`
//! - `min`
//! - `not`

fn main() {
    async_std::task::block_on(
        asking::question()
            .message("Please input a number between 2 and 4, that is not 3.\n")
            .repeat_help("Please, try again: ")
            .min_with_msg(2, "Remember, from 2 up!")
            .not(3)
            .max(4)
            .feedback(|value| match value {
                2 => "You chose 2!".to_string(),
                4 => "You chose 4!".to_string(),
                _ => unreachable!(),
            })
            .ask(),
    )
    .expect("Failed to read line");
}
