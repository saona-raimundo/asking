//! Build async prompts for non-blocking user input!

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod error;
mod question;

pub use question::{QuestionBuilder, StdQuestion};

///
pub fn question<T: std::str::FromStr>() -> StdQuestion<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Send + Sync + std::error::Error + 'static,
{
    StdQuestion::default()
}

///
///
/// The default parser reads, after making lowercase, the following:
/// - `true`: "true" or "t" or "yes" or "y"
/// - `false`: "false" or "f" or "no" or "n"
pub fn yn() -> StdQuestion<bool> {
    StdQuestion::default().parser(|s: &str| match s.to_lowercase().as_str() {
        "true" | "yes" | "y" | "t" => Ok(true),
        "false" | "f" | "no" | "n" => Ok(false),
        _ => match s.parse() {
            Ok(v) => Ok(v),
            Err(e) => Err(eyre::Report::new(e)),
        },
    })
}

use chrono::naive::NaiveDate;
///
pub fn date() -> StdQuestion<NaiveDate> {
    StdQuestion::default()
}

///
pub fn select<T>(options: &[T]) -> StdQuestion<T> {
    todo!()
}

///
pub fn multiple_select<T>(options: &[T]) -> StdQuestion<T> {
    todo!()
}

///
pub fn password() -> StdQuestion<String> {
    StdQuestion::default().feedback(|_| "".to_string())
}

///
pub fn text() -> StdQuestion<String> {
    StdQuestion::default()
}
