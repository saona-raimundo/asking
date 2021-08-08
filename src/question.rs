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

mod standard;
pub use standard::StdQuestionBuilder;

/// ## Contents
///
/// + [Constructor](#constructor)
/// + [Message](#message)
/// + [Testing value](#testing-value)
/// + [Testing value extended](#testing-value-extended)
/// + [Useful Settings](#useful-settings)
/// + [Prompt functionalities](#prompt-functionalities)
/// + [Processing Text Input](#processing-text-input)
/// + [Advanced Methods](#advanced-methods)
pub struct QuestionBuilder<T, R, W> {
    reader: BufReader<R>,
    writer: BufWriter<W>,
    message: (String, bool),
    help: (String, bool),
    default: Option<T>,
    feedback: Arc<dyn Fn(&T) -> String>,
    preparser: Arc<dyn Fn(String) -> String>,
    str_tests: Vec<(Arc<dyn Fn(&str) -> eyre::Result<()>>, bool)>,
    parser: (Arc<dyn Fn(&str) -> eyre::Result<T>>, bool),
    tests: Vec<(Arc<dyn Fn(&T) -> eyre::Result<()>>, bool)>,
    timeout: Option<Duration>,
    attempts: Option<usize>,
    required: (String, bool),
}

/// # Constructor
impl<T, R, W> QuestionBuilder<T, R, W>
where
    T: FromStr,
    <T as FromStr>::Err: Send + Sync + Error + 'static,
    R: Read,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
            message: (String::default(), bool::default()),
            help: (String::default(), bool::default()),
            default: None,
            feedback: Arc::new(|_| String::default()),
            preparser: Arc::new(|s| s),
            str_tests: Vec::default(),
            parser: (
                Arc::new(|s| s.parse().map_err(|e| Report::new(e))),
                bool::default(),
            ),
            tests: Vec::default(),
            timeout: None,
            attempts: None,
            required: (String::default(), bool::default()),
        }
    }
}

/// # Message
impl<T, R, W> QuestionBuilder<T, R, W> {
    pub fn message(mut self, message: impl ToString) -> Self {
        self.message = (message.to_string(), false);
        self
    }
    pub fn repeat_message(mut self, message: impl ToString) -> Self {
        self.message = (message.to_string(), true);
        self
    }

    pub fn help(mut self, help: impl ToString) -> Self {
        self.help = (help.to_string(), false);
        self
    }
    pub fn repeat_help(mut self, help: impl ToString) -> Self {
        self.help = (help.to_string(), true);
        self
    }

    pub fn feedback(mut self, feedback: impl Fn(&T) -> String + 'static) -> Self {
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

    pub fn test(self, test: impl Fn(&T) -> bool + 'static) -> Self {
        self.test_with_msg(test, "The value failed a test.\n")
    }

    pub fn test_with_msg(
        self,
        test: impl Fn(&T) -> bool + 'static,
        message: impl ToString + 'static,
    ) -> Self {
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
    T: PartialEq + PartialOrd + 'static,
{
    /// Test if the value is inside an iterator
    ///
    /// # Remarks
    ///
    /// To prevent infinite loops, make sure `iterator` is finite.
    pub fn inside<I>(self, iterator: I) -> Self
    where
        I: IntoIterator<Item = T> + 'static,
    {
        self.inside_with_msg(iterator, "Value is not one of the options.")
    }

    /// Test if the value is inside an iterator
    ///
    /// # Remarks
    ///
    /// To prevent infinite loops, make sure `iterator` is finite.
    pub fn inside_with_msg<I>(self, iterator: I, message: impl ToString + 'static) -> Self
    where
        I: IntoIterator<Item = T> + 'static,
    {
        let options: Vec<T> = iterator.into_iter().collect();
        self.test_with_msg(
            move |value: &T| options.iter().any(|option| option == value),
            message,
        )
    }

    /// Test if the value is at most `upper_bound`.
    pub fn max(self, upper_bound: T) -> Self {
        self.max_with_msg(upper_bound, "The value can not be so big.")
    }

    /// Test if the value is at most `upper_bound`.
    pub fn max_with_msg(self, upper_bound: T, message: impl ToString + 'static) -> Self {
        self.test_with_msg(move |value: &T| *value <= upper_bound, message)
    }

    /// Test if the value is between `lower_bound` and `upper_bound`, including borders.
    pub fn min_max(self, lower_bound: T, upper_bound: T) -> Self {
        self.min_with_msg(lower_bound, "The value can not be so small.")
            .max_with_msg(upper_bound, "The value can not be so big.")
    }

    /// Test if the value is between `lower_bound` and `upper_bound`, including borders.
    pub fn min_max_with_msg(
        self,
        lower_bound: T,
        upper_bound: T,
        message: impl ToString + 'static,
    ) -> Self {
        self.test_with_msg(
            move |value: &T| (lower_bound <= *value) && (*value <= upper_bound),
            message,
        )
    }

    /// Test if the value is at least `lower_bound`.
    pub fn min(self, lower_bound: T) -> Self {
        self.min_with_msg(lower_bound, "The value can not be so small.")
    }

    /// Test if the value is at least `lower_bound`.
    pub fn min_with_msg(self, lower_bound: T, message: impl ToString + 'static) -> Self {
        self.test_with_msg(move |value: &T| *value >= lower_bound, message)
    }

    /// Test if the value is not `other`.
    pub fn not(self, other: T) -> Self {
        self.not_with_msg(other, "This value is not allowed.")
    }

    /// Test if the value is not `other`.
    pub fn not_with_msg(self, other: T, message: impl ToString + 'static) -> Self {
        self.test_with_msg(move |value: &T| *value != other, message)
    }
}

/// # Useful settings
impl<T, R, W> QuestionBuilder<T, R, W> {
    /// Bound the number of possible attempts.
    pub fn attempts<O: Into<Option<usize>>>(mut self, attempts: O) -> Self {
        self.attempts = attempts.into();
        self
    }

    /// Give a default value in case the input is empty.
    ///
    /// # Remarks
    ///
    /// Default values are NOT tested, so make sure that it is a value that passes your tests!
    pub fn default_value<S: Into<Option<T>>>(mut self, value: S) -> Self {
        self.default = value.into();
        self
    }

    /// Requires that the input is not empty to continue.
    ///
    /// # Remarks
    ///
    /// There is a default mesage that will be displayed, so you might want to overwrite it
    /// using `required_with_msg`.
    pub fn required(mut self) -> Self {
        self.required.1 = true;
        self
    }

    pub fn required_with_msg(mut self, message: impl ToString) -> Self {
        self.required = (message.to_string(), true);
        self
    }

    pub fn required_toogle(mut self) -> Self {
        self.required.1 = !self.required.1;
        self
    }

    pub fn timeout<O: Into<Option<Duration>>>(mut self, duration: O) -> Self {
        self.timeout = duration.into();
        self
    }
}

/// # Prompt functionalities
impl<T, R, W> QuestionBuilder<T, R, W>
where
    T: FromStr,
    <T as FromStr>::Err: Send + Sync + Error + 'static,
    W: Write + Unpin,
    R: Read + Unpin,
{
    /// Asynchronously gets input from the user.
    ///
    /// # Errors
    ///
    /// When there are problems with displaying messages or reading input.
    pub async fn ask(self) -> Result<T, crate::error::Processing> {
        if let Some(duration) = self.timeout {
            async_std::future::timeout(duration, self.ask_loop()).await?
        } else {
            self.ask_loop().await
        }
    }

    async fn ask_loop(mut self) -> Result<T, crate::error::Processing> {
        loop {
            self.write_message().await?;
            let preinput = self.take_input().await?;
            self.decrease_attempts()?;
            let input = (self.preparser)(preinput);
            if self.test_string(&input).await.is_err() {
                continue;
            };
            if input.is_empty() && !self.required.1 && self.default.is_some() {
                return Ok(self.default.unwrap());
            }
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

    async fn write_message(&mut self) -> Result<(), std::io::Error> {
        write!(self.writer, "{}", &self.message.0).await?;
        self.writer.flush().await?;
        if !self.message.1 {
            self.message = ("".to_string(), false);
        }
        Ok(())
    }

    async fn take_input(&mut self) -> Result<String, std::io::Error> {
        let mut input = String::new();
        self.reader.read_line(&mut input).await?;
        Ok(input)
    }

    fn decrease_attempts(&mut self) -> Result<(), crate::error::Processing> {
        if let Some(left_attempts) = self.attempts {
            if left_attempts == 0 {
                return Err(crate::error::Processing::NoMoreAttempts);
            } else {
                self.attempts = Some(left_attempts - 1);
            }
        };
        Ok(())
    }

    async fn test_string(&mut self, str_proposal: &str) -> eyre::Result<()> {
        for str_test in &self.str_tests {
            let result = (str_test.0)(str_proposal);
            if let Err(ref e) = result {
                if str_test.1 {
                    write!(self.writer, "{}", e).await?;
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
            write!(self.writer, "{}", self.required.0).await?;
            self.writer.flush().await?;
        }
        let result = (self.parser.0)(input);
        if let Err(ref e) = result {
            self.display_help().await?;
            if self.parser.1 {
                write!(self.writer, "{}", e).await?;
                self.writer.flush().await?;
            }
        }
        result
    }

    async fn test_proposal(&mut self, proposal: &T) -> eyre::Result<()> {
        for test in &self.tests {
            let result = (test.0)(proposal);
            if let Err(ref e) = result {
                if test.1 {
                    write!(self.writer, "{}", e).await?;
                    self.writer.flush().await?;
                }
                self.display_help().await?;
                return result;
            }
        }
        Ok(())
    }

    async fn display_help(&mut self) -> Result<(), std::io::Error> {
        write!(self.writer, "{}", self.help.0).await?;
        self.writer.flush().await?;

        if !self.help.1 {
            self.help = ("".to_string(), false);
        }
        Ok(())
    }

    async fn give_feedback(&mut self, value: &T) -> Result<(), std::io::Error> {
        write!(self.writer, "{}", (self.feedback)(value)).await?;
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
    pub fn preparser(mut self, preparser: impl Fn(String) -> String + 'static) -> Self {
        self.preparser = Arc::new(preparser);
        self
    }

    /// Add a test over the unparsed input.
    ///
    /// # Remarks
    ///
    /// There is a default message that you might want to overwrite with `str_test_with_msg`.
    pub fn str_test(self, str_test: impl Fn(&str) -> bool + 'static) -> Self {
        self.str_test_with_msg(str_test, "The input failed a test before being parsed.")
    }

    /// Add a test over the unparsed input and shows the message if an error occurs.
    pub fn str_test_with_msg(
        self,
        str_test: impl Fn(&str) -> bool + 'static,
        message: impl ToString + 'static,
    ) -> Self {
        let str_test = move |s: &str| match str_test(s) {
            true => Ok(()),
            false => Err(Report::msg(message.to_string())),
        };
        self.str_test_with_feedback(str_test)
    }

    /// Tests that the input length is equal to `exact_length`.
    ///
    pub fn length(self, exact_length: usize) -> Self {
        self.length_with_msg(
            exact_length,
            format!("The input needs to have length exactly {}.", exact_length),
        )
    }

    /// Tests that the input length is equal to `exact_length`.
    ///
    pub fn length_with_msg(self, exact_length: usize, message: impl ToString + 'static) -> Self {
        self.str_test_with_msg(move |s: &str| s.len() == exact_length, message)
    }

    /// Tests that the input length is less or equal to `max_length`.
    ///
    pub fn max_length(self, max_length: usize) -> Self {
        self.length_with_msg(
            max_length,
            format!("The input needs to have length at most {}.", max_length),
        )
    }

    /// Tests that the input length is less or equal to `max_length`.
    ///
    pub fn max_length_with_msg(self, max_length: usize, message: impl ToString + 'static) -> Self {
        self.str_test_with_msg(move |s: &str| s.len() <= max_length, message)
    }

    /// Tests that the input length is greater or equal to `min_length`.
    ///
    pub fn min_length(self, min_length: usize) -> Self {
        self.length_with_msg(
            min_length,
            format!("The input needs to have length at least {}.", min_length),
        )
    }

    /// Tests that the input length is greater or equal to `min_length`.
    ///
    pub fn min_length_with_msg(self, min_length: usize, message: impl ToString + 'static) -> Self {
        self.str_test_with_msg(move |s: &str| s.len() >= min_length, message)
    }

    /// Set the parser for the input. Errors will NOT be displayed if they occur.
    ///
    /// This is shorthand for `self.parser_with_msg(parser, false)`.
    pub fn parser(mut self, parser: impl Fn(&str) -> eyre::Result<T> + 'static) -> Self {
        self.parser = (Arc::new(parser), false);
        self
    }
}

/// # Advanced methods
impl<T, R, W> QuestionBuilder<T, R, W> {
    pub fn reader<R2: Read + Unpin>(self, other_reader: R2) -> QuestionBuilder<T, R2, W> {
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
            timeout: self.timeout,
            attempts: self.attempts,
            required: self.required,
        }
    }

    pub fn writer<W2: Write + Unpin>(self, other_writer: W2) -> QuestionBuilder<T, R, W2> {
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
            timeout: self.timeout,
            attempts: self.attempts,
            required: self.required,
        }
    }

    /// Add a test over the parsed input whose error is displayed, if one occurs.
    pub fn test_with_feedback(mut self, test: impl Fn(&T) -> eyre::Result<()> + 'static) -> Self {
        self.tests.push((Arc::new(test), true));
        self
    }

    /// Add a test over the unparsed input and displays the error, if one occurs.
    pub fn str_test_with_feedback(
        mut self,
        str_test: impl Fn(&str) -> eyre::Result<()> + 'static,
    ) -> Self {
        self.str_tests.push((Arc::new(str_test), true));
        self
    }

    /// Set the parser for the input. Errors will be displayed if they occur.
    pub fn parser_with_feedback(
        mut self,
        parser: impl Fn(&str) -> eyre::Result<T> + 'static,
    ) -> Self {
        self.parser = (Arc::new(parser), true);
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
            .field("timeout", &self.timeout)
            .field("attempts", &self.attempts)
            .field("required", &self.required)
            .finish_non_exhaustive()
        // .field("feedback", &self.feedback)
        // .field("preparser", &self.preparser)
        // .field("str_tests", &self.str_tests)
        // .field("parser", &self.parser)
        // .field("tests", &self.tests)
    }
}
