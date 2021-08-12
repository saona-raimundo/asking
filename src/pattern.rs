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

/// Test if the value is inside an iterator
///
/// # Remarks
///
/// To prevent infinite loops, make sure `iterator` is finite.
pub fn select<T, I>(iterator: I) -> StdQuestionBuilder<T>
where
    T: PartialEq + FromStr + Send + Sync + 'static,
    <T as FromStr>::Err: Send + Sync + Error + 'static,
    I: IntoIterator<Item = T> + 'static,
{
    select_with_msg(iterator, "Value is not one of the options.")
}

/// Test if the value is inside an iterator
///
/// # Remarks
///
/// To prevent infinite loops, make sure `iterator` is finite.
pub fn select_with_msg<T, I, M>(iterator: I, message: M) -> StdQuestionBuilder<T>
where
    T: PartialEq + FromStr + Send + Sync + 'static,
    <T as FromStr>::Err: Send + Sync + Error + 'static,
    I: IntoIterator<Item = T> + 'static,
    M: ToString + Send + Sync + 'static,
{
    let question = StdQuestionBuilder::default();
    let options: Vec<T> = iterator.into_iter().collect();
    question.test_with_msg(
        move |value: &T| options.iter().any(|option| option == value),
        message,
    )
}

///
pub fn date() -> StdQuestionBuilder<NaiveDate> {
    StdQuestionBuilder::default()
}

///
pub fn text() -> StdQuestionBuilder<String> {
    StdQuestionBuilder::default()
}
