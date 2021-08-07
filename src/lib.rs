//! Build async prompts for non-blocking user input!

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

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
pub fn yn() -> StdQuestion<bool> {
    StdQuestion::default()
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
    let mut question = StdQuestion::default();
    question.feedback(|_| "".to_string());
    question
}

///
pub fn text() -> StdQuestion<String> {
    StdQuestion::default()
}
