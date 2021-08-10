use crate::StdQuestionBuilder;
use chrono::naive::NaiveDate;
use std::{error::Error, str::FromStr};
///
pub fn question<T: FromStr>() -> StdQuestionBuilder<T>
where
    T: FromStr + Send + Sync,
    <T as FromStr>::Err: Send + Sync + Error + 'static,
{
    StdQuestionBuilder::default()
}

/// Yes/No questions.
///
/// The default parser reads, after making lowercase, the following:
/// - `true`: "true" or "t" or "yes" or "y"
/// - `false`: "false" or "f" or "no" or "n"
pub fn yn() -> StdQuestionBuilder<bool> {
    StdQuestionBuilder::from(|s: &str| match s.to_lowercase().as_str() {
        "true" | "t" | "yes" | "y" => Ok(true),
        "false" | "f" | "no" | "n" => Ok(false),
        _ => s.parse(),
    })
}

///
pub fn date() -> StdQuestionBuilder<NaiveDate> {
    StdQuestionBuilder::default()
}

///
pub fn text() -> StdQuestionBuilder<String> {
    StdQuestionBuilder::default()
}
