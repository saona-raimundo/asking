use async_std::{
    io::{
        prelude::{BufReadExt, WriteExt},
        BufReader, BufWriter, Read, Write,
    },
    sync::Arc,
};
use core::str::FromStr;
use eyre::Report;
use std::{error::Error, marker::Unpin, string::ToString, time::Duration};

mod standard;
pub use standard::StdQuestion;

#[derive()]
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

/// # Message-related
impl<T, R, W> QuestionBuilder<T, R, W> {
    pub fn message(mut self, message: impl ToString) -> Self {
        self.message.0 = message.to_string();
        self
    }
    pub fn message_repeat(mut self, message_repeat: bool) -> Self {
        self.message.1 = message_repeat;
        self
    }
    pub fn repeat_message(self, message: impl ToString) -> Self {
        self.message(message).message_repeat(true)
    }

    pub fn help(mut self, help: impl ToString) -> Self {
        self.help.0 = help.to_string();
        self
    }
    pub fn help_repeat(mut self, help_repeat: bool) -> Self {
        self.help.1 = help_repeat;
        self
    }
    pub fn repeat_help(self, help: impl ToString) -> Self {
        self.help(help).help_repeat(true)
    }
}

/// # Testing value
impl<T, R, W> QuestionBuilder<T, R, W> {
    /// Forgets all tests.
    pub fn clear(mut self) -> Self {
        self.tests = vec![];
        self
    }

    /// Add a test over the parsed input. Errors will not be displayed if they occur.
    ///
    /// # Remark
    ///
    /// We encourage to write tests whose error should be displayed to the user,
    /// and add them with the `test_with_msg` method.
    pub fn test(mut self, test: impl Fn(&T) -> eyre::Result<()> + 'static) -> Self {
        self.tests.push((Arc::new(test), false));
        self
    }

    /// Add a test over the parsed input whose error is displayed, if one occurs.
    pub fn test_with_feedback(mut self, test: impl Fn(&T) -> eyre::Result<()> + 'static) -> Self {
        self.tests.push((Arc::new(test), true));
        self
    }

    pub fn quick_test(self, quick_test: impl Fn(&T) -> bool + 'static) -> Self {
        self.quick_test_with_msg(quick_test, "The value failed a test.")
    }

    pub fn quick_test_with_msg(
        self,
        quick_test: impl Fn(&T) -> bool + 'static,
        message: impl ToString + 'static,
    ) -> Self {
        let test = move |value: &T| match quick_test(value) {
            true => Ok(()),
            false => Err(Report::msg(message.to_string())),
        };
        self.test_with_feedback(test)
    }
}

/// # Testing values with more type constrains.
///
/// # Remarks
///
/// Not all bounds are used in all methods. Open an issue in GitHub if this is a problem for you.
impl<T, R, W> QuestionBuilder<T, R, W>
where
    T: PartialEq + 'static + PartialOrd,
{
    /// Test if the value is inside an iterator
    ///
    /// # Remarks
    ///
    /// To prevent infinite loops, make sure `iterator` is finite.
    pub fn inside<I>(self, iterator: I) -> Self
    where
        I: IntoIterator<Item = T> + Clone + 'static,
    {
        self.quick_test_with_msg(
            move |value: &T| iterator.clone().into_iter().any(|option| option == *value),
            "Value is not one of the options.",
        )
    }

    /// Test if the value is inside an iterator
    ///
    /// # Remarks
    ///
    /// To prevent infinite loops, make sure `iterator` is finite.
    pub fn inside_with_msg<I>(self, iterator: I, message: impl ToString + 'static) -> Self
    where
        I: IntoIterator<Item = T> + Clone + 'static,
    {
        self.quick_test_with_msg(
            move |value: &T| iterator.clone().into_iter().any(|option| option == *value),
            message,
        )
    }

    /// Test if the value is at most `upper_bound`.
    pub fn max(self, upper_bound: T) -> Self {
        self.max_with_msg(upper_bound, "The value can not be so big.")
    }

    /// Test if the value is at most `upper_bound`.
    pub fn max_with_msg(self, upper_bound: T, message: impl ToString + 'static) -> Self {
        self.quick_test_with_msg(move |value: &T| *value <= upper_bound, message)
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
        self.quick_test_with_msg(
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
        self.quick_test_with_msg(move |value: &T| *value >= lower_bound, message)
    }

    /// Test if the value is not `other`.
    pub fn not(self, other: T) -> Self {
        self.not_with_msg(other, "This value is not allowed.")
    }

    /// Test if the value is not `other`.
    pub fn not_with_msg(self, other: T, message: impl ToString + 'static) -> Self {
        self.quick_test_with_msg(move |value: &T| *value != other, message)
    }
}

/// # Useful
impl<T, R, W> QuestionBuilder<T, R, W> {
    ///
    /// # Remarks
    ///
    /// Default values are NOT tested, so make sure that it is a value that passes your tests!
    pub fn default_value<S: Into<Option<T>>>(mut self, value: S) -> Self {
        self.default = value.into();
        self
    }

    pub fn timeout<O: Into<Option<Duration>>>(mut self, duration: O) -> Self {
        self.timeout = duration.into();
        self
    }

    pub fn attempts<O: Into<Option<usize>>>(mut self, attempts: O) -> Self {
        self.attempts = attempts.into();
        self
    }

    pub fn required_with_msg(mut self, required: bool, message: impl ToString) -> Self {
        self.required = (message.to_string(), required);
        self
    }

    /// Requires that the input is not empty to continue.
    ///
    /// # Remarks
    ///
    /// There is a default mesage that will be displayed, so you might want to overwrite it
    /// using `required_with_msg`.
    pub fn required(mut self, required: bool) -> Self {
        self.required.1 = required;
        self
    }

    pub fn feedback(mut self, feedback: impl Fn(&T) -> String + 'static) -> Self {
        self.feedback = Arc::new(feedback);
        self
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

    /// Add a test over the unparsed input. Errors will not be displayed if they occur.
    ///
    /// This is shorthand for `self.test_with_msg(parser, false)`.
    pub fn str_test(mut self, str_test: impl Fn(&str) -> eyre::Result<()> + 'static) -> Self {
        self.str_tests.push((Arc::new(str_test), false));
        self
    }

    /// Add a test over the unparsed input displays the error, if one occurs.
    pub fn str_test_with_feedback(
        mut self,
        str_test: impl Fn(&str) -> eyre::Result<()> + 'static,
    ) -> Self {
        self.str_tests.push((Arc::new(str_test), true));
        self
    }

    pub fn quick_str_test(self, quick_str_test: impl Fn(&str) -> bool + 'static) -> Self {
        self.quick_str_test_with_msg(
            quick_str_test,
            "The input failed a test before being parsed.",
        )
    }

    pub fn quick_str_test_with_msg(
        self,
        quick_str_test: impl Fn(&str) -> bool + 'static,
        message: impl ToString + 'static,
    ) -> Self {
        let str_test = move |s: &str| match quick_str_test(s) {
            true => Ok(()),
            false => Err(Report::msg(message.to_string())),
        };
        self.str_test(str_test)
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
        self.quick_str_test_with_msg(move |s: &str| s.len() == exact_length, message)
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
        self.quick_str_test_with_msg(move |s: &str| s.len() <= max_length, message)
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
        self.quick_str_test_with_msg(move |s: &str| s.len() >= min_length, message)
    }

    /// Set the parser for the input. Errors will not be displayed if they occur.
    ///
    /// This is shorthand for `self.parser_with_msg(parser, false)`.
    pub fn parser(mut self, parser: impl Fn(&str) -> eyre::Result<T> + 'static) -> Self {
        self.parser = (Arc::new(parser), false);
        self
    }

    /// Set the parser for the input and choose to display the error, if one occurs while parsing.
    pub fn parser_with_feedback(
        mut self,
        parser: impl Fn(&str) -> eyre::Result<T> + 'static,
    ) -> Self {
        self.parser = (Arc::new(parser), true);
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
}
