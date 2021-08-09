use crate::StdQuestionBuilder;
use chrono::naive::NaiveDate;

///
pub fn question<T: std::str::FromStr>() -> StdQuestionBuilder<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Send + Sync + std::error::Error + 'static,
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
