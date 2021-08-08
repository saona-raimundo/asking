use crate::StdQuestionBuilder;
use core::str::FromStr;
// use eyre::Report;
use std::error::Error;

///
pub fn question<T: std::str::FromStr>() -> StdQuestionBuilder<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Send + Sync + std::error::Error + 'static,
{
    StdQuestionBuilder::default()
}

///
///
/// The default parser reads, after making lowercase, the following:
/// - `true`: "true" or "t" or "yes" or "y"
/// - `false`: "false" or "f" or "no" or "n"
pub fn yn() -> StdQuestionBuilder<bool> {
    StdQuestionBuilder::default().parser(|s: &str| match s.to_lowercase().as_str() {
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
pub fn date() -> StdQuestionBuilder<NaiveDate> {
    StdQuestionBuilder::default()
}

///
pub fn select<T>(options: Vec<T>) -> StdQuestionBuilder<T>
where
    T: FromStr + PartialOrd + 'static,
    <T as FromStr>::Err: Send + Sync + Error + 'static,
{
    StdQuestionBuilder::default().inside(options)
}

///
pub fn password() -> StdQuestionBuilder<String> {
    StdQuestionBuilder::default().feedback(|_| "".to_string())
}

///
pub fn text() -> StdQuestionBuilder<String> {
    StdQuestionBuilder::default()
}
