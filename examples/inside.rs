//! Require the input to be an element of an iterator.
//!
//! This examples shows how to use:
//! - `select`

fn main() {
    let level: u32 = async_std::task::block_on(
        asking::question()
            .message("What level is your Pokemon?")
            .inside_with_msg(
                5..=100,
                "It should be at least level 5 and most 100, right? Try again.",
            )
            .required_with_msg("Sorry, I need this. Give me an answer please.")
            .feedback(|level| format!("Level {}! That is awesome!\n", level))
            .ask(),
    )
    .expect("Failed to read line");

    assert!(level <= 100 && level >= 5);
}
