use thiserror::Error;

/// All possible errors of the [`ask`] method.
///
/// [`ask`]: ../struct.QuestionBuilder.html#method.ask
#[derive(Error, Debug)]
pub enum ProcessingError {
    /// The user has no more attempts to answer a question.
    ///
    /// Related to the method [`attempts`].
    ///
    /// [`attempts`]: ../struct.QuestionBuilder.html#method.attempts
    #[error("No more attempts to answer this question.")]
    NoMoreAttempts,
    /// There was an I/O error while asking.
    #[error("Problems with displaying messages or reading input.")]
    Io {
        #[from]
        source: std::io::Error,
    },
    /// The reader got to the end of file (EOF) character. In other words, there was no input.
    ///
    /// # Remarks
    ///
    /// This is particularly useful when reading from a file.
    #[error("EOF reached while asking for input.")]
    Eof,
    /// The time to answer a question has passed.
    ///
    /// Related to the method [`timeout`].
    ///
    /// [`timeout`]: ../struct.QuestionBuilder.html#method.timeout
    #[error("User could not answer the question in the given time.")]
    Timeout {
        #[from]
        source: async_std::future::TimeoutError,
    },
}
