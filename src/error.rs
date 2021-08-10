//!

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Processing {
    #[error("No more attempts to answer this question.")]
    NoMoreAttempts,
    #[error("Problems with displaying messages or reading input.")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("EOF reached while asking for input.")]
    Eof,
    #[error("User could not answer the question in the given time.")]
    Timeout {
        #[from]
        source: async_std::future::TimeoutError,
    },
}

#[derive(Error, Debug)]
pub enum Looping {
    #[error("{0}")]
    Required(String),
}
