use async_std::{
    io,
    io::{BufReader, BufWriter, Stdin, Stdout},
    sync::Arc,
};
use core::str::FromStr;
use eyre::Report;
use std::error::Error;

use crate::QuestionBuilder;

/// Question in the standard input/output of the current process.
pub type StdQuestion<T> = QuestionBuilder<T, Stdin, Stdout>;

impl<T> Default for StdQuestion<T>
where
    T: FromStr,
    <T as FromStr>::Err: Send + Sync + Error + 'static,
{
    fn default() -> Self {
        StdQuestion {
            reader: BufReader::new(io::stdin()),
            writer: BufWriter::new(io::stdout()),
            message: (String::default(), bool::default()),
            help: (String::default(), bool::default()),
            default: None,
            feedback: Arc::new(|_| String::default()),
            parser: Arc::new(|s| s.trim_end().parse().map_err(|e| Report::new(e))),
            tests: Vec::default(),
            timeout: None,
            attempts: None,
        }
    }
}
