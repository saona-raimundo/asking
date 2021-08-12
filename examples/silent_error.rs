//! Silent errors, for those who do not want to give feedback.
//!
//! This examples shows the use of
//! - `error_formatter`

#[async_std::main]
async fn main() -> eyre::Result<()> {
    asking::question()
        .message("Give me a number bigger than 3, please. I will not tell you anything else.\n")
        .test(|i: &u32| *i > 3)
        .error_formatter(|_| "".to_string())
        .ask()
        .await?;
    Ok(())
}
