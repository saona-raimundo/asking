use async_std::{
    io::{
        prelude::{BufReadExt, WriteExt},
        BufReader, BufWriter, Read, Write,
    },
    sync::Arc,
};
use core::{fmt::Debug, str::FromStr};
use eyre::Report;
use std::{error::Error, marker::Unpin, string::ToString, time::Duration};

mod executor;
mod standard;
pub use executor::Executor;
pub use standard::StdQuestionBuilder;

/// Async I/O handler (in builder form).
///
/// # Contents
///
/// + [Processing](#processing)
/// + [Constructor](#constructor)
/// + [Message](#message)
/// + [Testing value](#testing-value)
/// + [Testing value extended](#testing-value-extended)
/// + [Useful Settings](#useful-settings)
/// + [Prompt functionalities](#prompt-functionalities)
/// + [Processing Text Input](#processing-text-input)
/// + [Advanced Methods](#advanced-methods)
///
/// # Processing
///
/// There are many steps in handling input and giving feedback, and therefore many options.
/// As a preview, these are the main steps:
/// 1. Write message
/// 2. Take input
/// 3. Test the input as a `String`
/// 4. Parse input
/// 5. Test the value
/// 6. Give feedback
/// 7. Return the value
///
/// For more details, checkout the [`ask`] method.
///
/// [`ask`]: #method.ask
pub struct QuestionBuilder<T, R, W> {
    reader: BufReader<R>,
    writer: BufWriter<W>,
    message: (String, bool),
    help: (String, bool),
    default: Option<T>,
    feedback: Arc<dyn Fn(&T) -> String + Send + Sync>,
    preparser: Arc<dyn Fn(String) -> String + Send + Sync>,
    str_tests: Vec<(Arc<dyn Fn(&str) -> eyre::Result<()> + Send + Sync>, bool)>,
    parser: (Arc<dyn Fn(&str) -> eyre::Result<T> + Send + Sync>, bool),
    tests: Vec<(Arc<dyn Fn(&T) -> eyre::Result<()> + Send + Sync>, bool)>,
    error_formatter: Arc<dyn Fn(String) -> String + Send + Sync>,
    executor: Executor,
    attempts: Option<(usize, Arc<dyn Fn(usize) -> String + Send + Sync>)>,
    required: (String, bool),
}

/// # Constructor
impl<T, R, W> QuestionBuilder<T, R, W>
where
    R: Read,
    W: Write,
{
    /// Constructs a new `QuestionBuilder<T, R, W>`.
    ///
    /// # Remarks
    ///
    /// There are two default behaviours while handling strings.
    ///
    /// 1. The [`preparser`] trims the end of the input.
    /// 2. The [`error_formatter`] adds a new line before display.
    ///
    /// [`preparser`]: #method.preparser
    /// [`error_formatter`]: #method.error_formatter
    pub fn new<F, E>(reader: R, writer: W, parser: F) -> Self
    where
        F: Fn(&str) -> Result<T, E> + Send + Sync + 'static,
        E: Error + Send + Sync + 'static,
    {
        Self {
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
            message: (String::default(), bool::default()),
            help: (String::default(), bool::default()),
            default: None,
            feedback: Arc::new(|_| String::default()),
            preparser: Arc::new(|s| s.trim_end().to_string()),
            str_tests: Vec::default(),
            parser: (
                Arc::new(move |s| parser(s).map_err(|e| Report::new(e))),
                bool::default(),
            ),
            tests: Vec::default(),
            error_formatter: Arc::new(|s| s + "\n"),
            executor: Executor::None,
            attempts: None,
            required: (String::default(), bool::default()),
        }
    }
}

impl<T, R, W> QuestionBuilder<T, R, W>
where
    T: FromStr,
    <T as FromStr>::Err: Error + Send + Sync + 'static,
    R: Read,
    W: Write,
{
    /// Constructs a new `QuestionBuilder<T, R, W>` where
    /// the [`parser`] is given by the implementation of `FromStr`.
    ///
    /// [`parser`]: #method.parser
    pub fn new_fromstr(reader: R, writer: W) -> Self {
        Self::new(reader, writer, |s| s.parse())
    }
}

/// # Message
///
/// Main messages that will be displayed.
impl<T, R, W> QuestionBuilder<T, R, W> {
    /// Message to be displayed.
    pub fn message(mut self, message: impl ToString) -> Self {
        self.message = (message.to_string(), false);
        self
    }
    /// Message to be displayed repeatedly before each attempt.
    pub fn repeat_message(mut self, message: impl ToString) -> Self {
        self.message = (message.to_string(), true);
        self
    }
    /// Help message to be displayed after the first failed attempt.
    pub fn help(mut self, help: impl ToString) -> Self {
        self.help = (help.to_string(), false);
        self
    }
    /// Help message to be displayed every time an attempt failed.
    pub fn repeat_help(mut self, help: impl ToString) -> Self {
        self.help = (help.to_string(), true);
        self
    }
    /// Feedback message to be displayed after the input has been succesfully processed.
    pub fn feedback<F>(mut self, feedback: F) -> Self
    where
        F: Fn(&T) -> String + Send + Sync + 'static,
    {
        self.feedback = Arc::new(feedback);
        self
    }
}

/// # Testing value
///
/// We encourage to write tests whose error should be displayed to the user,
/// and add them with the `test_with_msg` method.
impl<T, R, W> QuestionBuilder<T, R, W> {
    /// Forgets all tests.
    pub fn clear(mut self) -> Self {
        self.tests = vec![];
        self
    }
    /// Add a new test for the value.
    ///
    /// # Remarks
    ///
    /// There is a default message you might want to change.
    pub fn test<F>(self, test: F) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.test_with_msg(test, "The value failed a test.")
    }
    /// Add a new test for the value, displaying a message upon failure.
    pub fn test_with_msg<F, M>(self, test: F, message: M) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
        M: ToString + Send + Sync + 'static,
    {
        let test = move |value: &T| match test(value) {
            true => Ok(()),
            false => Err(Report::msg(message.to_string())),
        };
        self.test_with_feedback(test)
    }
}

/// # Testing value extended
///
/// ## Remarks
///
/// Not all bounds are used in all methods. Open an issue in GitHub if this is a problem for you.
impl<T, R, W> QuestionBuilder<T, R, W>
where
    T: PartialEq + PartialOrd + Send + Sync + 'static,
{
    /// Test if the value is inside an iterator.
    ///
    /// # Remarks
    ///
    /// To prevent infinite loops, make sure `iterator` is finite.
    /// Also, there is a default message you might want to change.
    pub fn inside<I>(self, iterator: I) -> Self
    where
        I: IntoIterator<Item = T> + 'static,
    {
        self.inside_with_msg(iterator, "Value is not one of the options.")
    }

    /// Test if the value is inside an iterator, displaying a message upon failure.
    ///
    /// # Remarks
    ///
    /// To prevent infinite loops, make sure `iterator` is finite.
    pub fn inside_with_msg<I, M>(self, iterator: I, message: M) -> Self
    where
        I: IntoIterator<Item = T> + 'static,
        M: ToString + Send + Sync + 'static,
    {
        let options: Vec<T> = iterator.into_iter().collect();
        self.test_with_msg(
            move |value: &T| options.iter().any(|option| option == value),
            message,
        )
    }

    /// Test if the value is at most `upper_bound`.
    ///
    /// # Remarks
    ///
    /// There is a default message you might want to change.
    pub fn max(self, upper_bound: T) -> Self {
        self.max_with_msg(upper_bound, "The value can not be so big.")
    }

    /// Test if the value is at most `upper_bound`, displaying a message upon failure.
    pub fn max_with_msg<M>(self, upper_bound: T, message: M) -> Self
    where
        M: ToString + Send + Sync + 'static,
    {
        self.test_with_msg(move |value: &T| *value <= upper_bound, message)
    }

    /// Test if the value is between `lower_bound` and `upper_bound`, including borders.
    ///
    /// # Remarks
    ///
    /// There is a default message you might want to change.
    pub fn min_max(self, lower_bound: T, upper_bound: T) -> Self {
        self.min_with_msg(lower_bound, "The value can not be so small.")
            .max_with_msg(upper_bound, "The value can not be so big.")
    }

    /// Test if the value is between `lower_bound` and `upper_bound`, including borders,
    /// displaying a message upon failure.
    pub fn min_max_with_msg<M>(self, lower_bound: T, upper_bound: T, message: M) -> Self
    where
        M: ToString + Send + Sync + 'static,
    {
        self.test_with_msg(
            move |value: &T| (lower_bound <= *value) && (*value <= upper_bound),
            message,
        )
    }

    /// Test if the value is at least `lower_bound`.
    ///
    /// # Remarks
    ///
    /// There is a default message you might want to change.
    pub fn min(self, lower_bound: T) -> Self {
        self.min_with_msg(lower_bound, "The value can not be so small.")
    }

    /// Test if the value is at least `lower_bound`, displaying a message upon failure.
    pub fn min_with_msg<M>(self, lower_bound: T, message: M) -> Self
    where
        M: ToString + Send + Sync + 'static,
    {
        self.test_with_msg(move |value: &T| *value >= lower_bound, message)
    }

    /// Test if the value is not `other`.
    ///
    /// # Remarks
    ///
    /// There is a default message you might want to change.
    pub fn not(self, other: T) -> Self {
        self.not_with_msg(other, "This value is not allowed.")
    }

    /// Test if the value is not `other`, displaying a message upon failure.
    pub fn not_with_msg<M>(self, other: T, message: M) -> Self
    where
        M: ToString + Send + Sync + 'static,
    {
        self.test_with_msg(move |value: &T| *value != other, message)
    }
}

/// # Useful settings
impl<T, R, W> QuestionBuilder<T, R, W> {
    /// Bound the number of possible attempts.
    ///
    /// The default value is `None`, which gives infinite attempts to the user.
    pub fn attempts<O>(mut self, attempts: O) -> Self
    where
        O: Into<Option<usize>>,
    {
        match attempts.into() {
            Some(attempts) => self.attempts_with_feedback(attempts, |_| "".to_string()),
            None => {
                self.attempts = None;
                self
            }
        }
    }

    /// Bound the number of possible attempts, displaying a message before any input is read.
    ///
    /// The default value is `None`, which gives infinite attempts to the user.
    ///
    /// # Remarks
    ///
    /// Before the first attempt, a message will be displayed, so make sure to handle that case.
    pub fn attempts_with_feedback<F>(mut self, attempts: usize, feedback: F) -> Self
    where
        F: Fn(usize) -> String + Send + Sync + 'static,
    {
        self.attempts = Some((attempts, Arc::new(feedback)));
        self
    }

    /// Give a default value in case the input is not required and empty.
    ///
    /// # Remarks
    ///
    /// Default values are NOT tested, so make sure that it is a value that passes your tests!
    pub fn default_value<S>(mut self, value: S) -> Self
    where
        S: Into<Option<T>>,
    {
        self.default = value.into();
        self
    }

    /// Requires that the input is not empty to continue.
    ///
    /// # Remarks
    ///
    /// There is a default message you might want to change.
    pub fn required(mut self) -> Self {
        self.required.1 = true;
        self
    }

    /// Requires that the input is not empty to continue, displaying a message upon failure.
    pub fn required_with_msg(mut self, message: impl ToString) -> Self {
        self.required = (message.to_string(), true);
        self
    }

    /// Toggles between requiring and not requiring input.
    pub fn required_toogle(mut self) -> Self {
        self.required.1 = !self.required.1;
        self
    }
}

/// # Executors
///
/// For ease of use, there are some built-in executors.
impl<T, R, W> QuestionBuilder<T, R, W> {
    /// set a maximum time for the user to finish answering the question.
    ///
    /// # Remarks
    ///
    /// This time corresponds to the whole execution, including displaying feedback.
    /// So the user might enter valid input before the time runs out,
    /// but the whole process might still timeout.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.executor = Executor::Timeout(duration);
        self
    }
}

/// # Prompt functionalities
impl<T, R, W> QuestionBuilder<T, R, W>
where
    W: Write + Unpin,
    R: Read + Unpin,
{
    /// Asynchronously gets input from the user.
    ///
    /// The detailed process is as follows.
    ///
    /// 0. Check there are [`attempts`] left
    /// 1. Write [`message`]
    /// 2. Write [`feedback from attempts`]
    /// 3. Read input
    /// 4. Apply the [`preparser`] to the input
    /// 5. Return [`default_value`] if it corresponds
    /// 6. Apply all [`str_test`]s
    /// 4. Convert the input with [`parser`]
    /// 5. Apply all [`test`]s
    /// 6. Write [`feedback`]
    /// 7. Return the value
    ///
    /// [`attempts`]: #method.attempts
    /// [`message`]: #method.message
    /// [`feedback from attempts`]: #method.attempts_with_feedback
    /// [`preparser`]: #method.preparser
    /// [`default_value`]: #method.default_value
    /// [`str_test`]: #method.str_test
    /// [`parser`]: #method.parser
    /// [`test`]: #method.test
    /// [`feedback`]: #method.feedback
    ///
    /// # Remarks
    ///
    /// There are two types of messages that can be displayed during the process:
    /// feedback and error messages.
    /// Feedback is displayed as given by the corresponding method.
    /// Error messages are displayed after applying [`error_formatter`].
    ///
    /// [`error_formatter`]: #method.error_formatter
    pub async fn ask(self) -> Result<T, crate::error::Processing> {
        match self.executor {
            Executor::None => self.ask_loop().await,
            Executor::Timeout(duration) => {
                async_std::future::timeout(duration, self.ask_loop()).await?
            }
        }
    }

    async fn ask_loop(mut self) -> Result<T, crate::error::Processing> {
        loop {
            self.check_attempts()?;
            self.write_message().await?;
            self.write_attempts_feedback().await?;
            let preinput = self.take_input().await?;
            self.decrease_attempts()?;
            let input = (self.preparser)(preinput);
            if input.is_empty() && !self.required.1 && self.default.is_some() {
                return Ok(self.default.unwrap());
            }
            if self.test_string(&input).await.is_err() {
                continue;
            };
            let proposal = match self.parse_input(&input).await {
                Ok(value) => value,
                Err(_) => continue,
            };
            if self.test_proposal(&proposal).await.is_err() {
                continue;
            }
            self.give_feedback(&proposal).await?;

            return Ok(proposal);
        }
    }

    fn check_attempts(&mut self) -> Result<(), crate::error::Processing> {
        match self.attempts {
            Some((0, _)) => Err(crate::error::Processing::NoMoreAttempts),
            _ => Ok(()),
        }
    }

    async fn write_message(&mut self) -> Result<(), std::io::Error> {
        self.writer.write(self.message.0.as_bytes()).await?;
        self.writer.flush().await?;
        if !self.message.1 {
            self.message = ("".to_string(), false);
        }
        Ok(())
    }

    async fn write_attempts_feedback(&mut self) -> Result<(), std::io::Error> {
        if let Some((left_attempts, feedback)) = &self.attempts {
            self.writer
                .write((feedback)(*left_attempts).as_bytes())
                .await?;
            self.writer.flush().await?;
        };
        Ok(())
    }

    async fn take_input(&mut self) -> Result<String, crate::error::Processing> {
        let mut input = String::new();
        if self.reader.read_line(&mut input).await? == 0 {
            return Err(crate::error::Processing::Eof);
        }
        Ok(input)
    }

    fn decrease_attempts(&mut self) -> Result<(), crate::error::Processing> {
        if let Some((left_attempts, _)) = &mut self.attempts {
            *left_attempts -= 1;
        }
        Ok(())
    }

    async fn test_string(&mut self, str_proposal: &str) -> eyre::Result<()> {
        for str_test in &self.str_tests {
            let result = (str_test.0)(str_proposal);
            if let Err(ref e) = result {
                if str_test.1 {
                    self.writer
                        .write((self.error_formatter)(e.to_string()).as_bytes())
                        .await?;
                    self.writer.flush().await?;
                }
                self.display_help().await?;
                return result;
            }
        }
        Ok(())
    }

    async fn parse_input(&mut self, input: &str) -> eyre::Result<T> {
        if input == "" && self.required.1 {
            self.display_help().await?;
            self.writer
                .write((self.error_formatter)(self.required.0.clone()).as_bytes())
                .await?;
            self.writer.flush().await?;
        }
        let result = (self.parser.0)(input);
        if let Err(ref e) = result {
            self.display_help().await?;
            if self.parser.1 {
                self.writer
                    .write((self.error_formatter)(e.to_string()).as_bytes())
                    .await?;
                self.writer.flush().await?;
            }
        }
        result
    }

    async fn test_proposal(&mut self, proposal: &T) -> eyre::Result<()> {
        for test in &self.tests {
            let result = (test.0)(proposal);
            if let Err(e) = result {
                if test.1 {
                    self.writer
                        .write((self.error_formatter)(e.to_string()).as_bytes())
                        .await?;
                    self.writer.flush().await?;
                }
                self.display_help().await?;
                return Err(e);
            }
        }
        Ok(())
    }

    async fn display_help(&mut self) -> Result<(), std::io::Error> {
        self.writer.write(self.help.0.as_bytes()).await?;
        self.writer.flush().await?;

        if !self.help.1 {
            self.help = ("".to_string(), false);
        }
        Ok(())
    }

    async fn give_feedback(&mut self, value: &T) -> Result<(), std::io::Error> {
        self.writer.write((self.feedback)(value).as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }
}

/// # Processing text input
///
/// ## Remarks
///
/// This is done after applying the preparser and before parsing the input.
/// Therefore, the changes the preparser process (like triming trailing space)
/// do not count for the length of the input.
impl<T, R, W> QuestionBuilder<T, R, W> {
    /// Set the preparser for the input.
    ///
    /// This is applied to the raw input before being parsed to `T`.
    /// In CLI applications, it is useful to clean the leading new line that comes with the input.
    pub fn preparser<F>(mut self, preparser: F) -> Self
    where
        F: Fn(String) -> String + Send + Sync + 'static,
    {
        self.preparser = Arc::new(preparser);
        self
    }

    /// Add a test over the unparsed input.
    ///
    /// # Remarks
    ///
    /// There is a default message that you might want to change.
    pub fn str_test<F>(self, str_test: F) -> Self
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        self.str_test_with_msg(str_test, "The input failed a test before being parsed.")
    }

    /// Add a test over the unparsed input, displaying a message upon failure.
    pub fn str_test_with_msg<F, M>(self, str_test: F, message: M) -> Self
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
        M: ToString + Send + Sync + 'static,
    {
        let str_test = move |s: &str| match str_test(s) {
            true => Ok(()),
            false => Err(Report::msg(message.to_string())),
        };
        self.str_test_with_feedback(str_test)
    }

    /// Tests that the input length is equal to `exact_length`.
    ///
    /// # Remarks
    ///
    /// There is a default message that you might want to change.
    pub fn length(self, exact_length: usize) -> Self {
        self.length_with_msg(
            exact_length,
            format!("The input needs to have length exactly {}.", exact_length),
        )
    }

    /// Tests that the input length is equal to `exact_length`, displaying a message upon failure.
    ///
    pub fn length_with_msg<M>(self, exact_length: usize, message: M) -> Self
    where
        M: ToString + Send + Sync + 'static,
    {
        self.str_test_with_msg(move |s: &str| s.len() == exact_length, message)
    }

    /// Tests that the input length is less or equal to `max_length`.
    ///
    /// # Remarks
    ///
    /// There is a default message that you might want to change.
    pub fn max_length(self, max_length: usize) -> Self {
        self.length_with_msg(
            max_length,
            format!("The input needs to have length at most {}.", max_length),
        )
    }

    /// Tests that the input length is less or equal to `max_length`, displaying a message upon failure.
    pub fn max_length_with_msg<M>(self, max_length: usize, message: M) -> Self
    where
        M: ToString + Send + Sync + 'static,
    {
        self.str_test_with_msg(move |s: &str| s.len() <= max_length, message)
    }

    /// Tests that the input length is greater or equal to `min_length`.
    ///
    /// # Remarks
    ///
    /// There is a default message that you might want to change.
    pub fn min_length(self, min_length: usize) -> Self {
        self.length_with_msg(
            min_length,
            format!("The input needs to have length at least {}.", min_length),
        )
    }

    /// Tests that the input length is greater or equal to `min_length`, displaying a message upon failure.
    pub fn min_length_with_msg<M>(self, min_length: usize, message: M) -> Self
    where
        M: ToString + Send + Sync + 'static,
    {
        self.str_test_with_msg(move |s: &str| s.len() >= min_length, message)
    }

    /// Set the parser for the input.
    ///
    /// # Remarks
    ///
    /// Errors will NOT be displayed if they occur.
    /// Check out [`parser_with_feedback`] if you want to display a message upon failure.
    ///
    /// [`parser_with_feedback`]: #method.parser_with_feedback
    pub fn parser<F>(mut self, parser: F) -> Self
    where
        F: Fn(&str) -> eyre::Result<T> + Send + Sync + 'static,
    {
        self.parser = (Arc::new(parser), false);
        self
    }
}

/// # Advanced methods
impl<T, R, W> QuestionBuilder<T, R, W> {
    /// Change the reader.
    pub fn reader<R2: Read>(self, other_reader: R2) -> QuestionBuilder<T, R2, W> {
        QuestionBuilder {
            reader: BufReader::new(other_reader),
            writer: self.writer,
            message: self.message,
            help: self.help,
            default: self.default,
            feedback: self.feedback,
            preparser: self.preparser,
            str_tests: self.str_tests,
            parser: self.parser,
            tests: self.tests,
            error_formatter: self.error_formatter,
            executor: self.executor,
            attempts: self.attempts,
            required: self.required,
        }
    }
    /// Change the writer.
    pub fn writer<W2: Write>(self, other_writer: W2) -> QuestionBuilder<T, R, W2> {
        QuestionBuilder {
            reader: self.reader,
            writer: BufWriter::new(other_writer),
            message: self.message,
            help: self.help,
            default: self.default,
            feedback: self.feedback,
            preparser: self.preparser,
            str_tests: self.str_tests,
            parser: self.parser,
            tests: self.tests,
            error_formatter: self.error_formatter,
            executor: self.executor,
            attempts: self.attempts,
            required: self.required,
        }
    }

    /// Change the way errors are displayed.
    ///
    /// # Remarks
    ///
    /// This can be useful if to:
    /// - Change the default behaviour of appending a newline.
    /// - Not display a message upon an error.
    ///
    /// # Examples
    ///
    /// Silent errors. Now, only messages will be displayed.
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> eyre::Result<()> {
    ///     let _num = asking::question()
    ///         .message("Give me a number bigger than 3, please. I will not tell you anything else.\n")
    ///         .test(|i: &u32| *i > 3)
    ///         .error_formatter(|_| "".to_string())
    ///         .ask()
    ///         .await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn error_formatter<F>(mut self, error_formatter: F) -> Self
    where
        F: Fn(String) -> String + Send + Sync + 'static,
    {
        self.error_formatter = Arc::new(error_formatter);
        self
    }

    /// Toggle the feedback from the parser.
    ///
    /// If activated, errors from parsing will be displayed.
    pub fn parser_feedback_toggle(mut self) -> Self {
        self.parser.1 = !self.parser.1;
        self
    }

    /// Set the parser for the input.
    ///
    /// Errors will be displayed if they occur.
    pub fn parser_with_feedback<F>(mut self, parser: F) -> Self
    where
        F: Fn(&str) -> eyre::Result<T> + Send + Sync + 'static,
    {
        self.parser = (Arc::new(parser), true);
        self
    }

    /// Add a test over the unparsed input.
    ///
    /// Errors will be displayed if they occur.
    pub fn str_test_with_feedback<F>(mut self, str_test: F) -> Self
    where
        F: Fn(&str) -> eyre::Result<()> + Send + Sync + 'static,
    {
        self.str_tests.push((Arc::new(str_test), true));
        self
    }

    /// Add a test over the parsed input
    ///
    /// Errors will be displayed if they occur.
    pub fn test_with_feedback<F>(mut self, test: F) -> Self
    where
        F: Fn(&T) -> eyre::Result<()> + Send + Sync + 'static,
    {
        self.tests.push((Arc::new(test), true));
        self
    }
}

impl<T, R, W> Debug for QuestionBuilder<T, R, W>
where
    T: Debug,
    R: Read + Debug,
    W: Write + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuestionBuilder")
            .field("reader", &self.reader)
            .field("writer", &self.writer)
            .field("message", &self.message)
            .field("help", &self.help)
            .field("default", &self.default)
            .field("executor", &self.executor)
            .field("required", &self.required)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::io;

    #[test]
    fn is_send() {
        if false {
            let question = QuestionBuilder::new_fromstr(io::stdin(), io::stdout()).ask();
            fn is_send<T: Send>(_: &T) {}
            is_send(&question);
            let _answer: bool = async_std::task::block_on(question).unwrap();
        }
    }
}
