//! Short application to be tested through the example `testing`.
//!
//! This examples shows the use of
//! - `yn`

#[async_std::main]
async fn main() {
    asking::yn()
        .message("Do you like testing? ")
        .help("Please use format y/n. Try again: ")
        .feedback(|b| match *b {
            true => "Great!".to_string(),
            false => "Oh no!".to_string(),
        })
        .ask()
        .await
        .expect("Failed to ask the question");
}
