use async_std::{
    io,
    io::{Stdin, Stdout},
};
use core::str::FromStr;
use std::error::Error;

use crate::QuestionBuilder;

/// Question in the standard input/output of the current process.
pub type StdQuestionBuilder<T> = QuestionBuilder<T, Stdin, Stdout>;

/// # Constructors
impl<T, F, E> From<F> for StdQuestionBuilder<T>
where
    F: Fn(&str) -> Result<T, E> + 'static,
    E: Error + Send + Sync + 'static,
{
    fn from(parser: F) -> Self {
        QuestionBuilder::new(io::stdin(), io::stdout(), parser)
    }
}

impl<T> Default for StdQuestionBuilder<T>
where
    T: FromStr,
    <T as FromStr>::Err: Send + Sync + Error + 'static,
{
    fn default() -> Self {
        StdQuestionBuilder::new_fromstr(io::stdin(), io::stdout())
    }
}
