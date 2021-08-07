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
    parser: (Arc<dyn Fn(&str) -> eyre::Result<T>>, bool),
    tests: Vec<(Arc<dyn Fn(&T) -> eyre::Result<()>>, bool)>,
    timeout: Option<Duration>,
    attempts: Option<usize>,
    required: bool,
}

/// # Message-related
impl<T, R, W> QuestionBuilder<T, R, W> {
    pub fn message<O: ToString>(mut self, message: O) -> Self {
        self.message.0 = message.to_string();
        self
    }
    pub fn message_repeat(mut self, message_repeat: bool) -> Self {
        self.message.1 = message_repeat;
        self
    }
    pub fn repeat_message<O: ToString>(self, message: O) -> Self {
        self.message(message).message_repeat(true)
    }

    pub fn help<O: ToString>(mut self, help: O) -> Self {
        self.help.0 = help.to_string();
        self
    }
    pub fn help_repeat(mut self, help_repeat: bool) -> Self {
        self.help.1 = help_repeat;
        self
    }
    pub fn repeat_help<O: ToString>(self, help: O) -> Self {
        self.help(help).help_repeat(true)
    }
}

/// # Test-related
impl<T, R, W> QuestionBuilder<T, R, W> {
    /// Add a test over the parsed input and choose to display the error, if one occurs.
    pub fn test_with_msg(
        mut self,
        test: impl Fn(&T) -> eyre::Result<()> + 'static,
        display_err: bool,
    ) -> Self {
        self.tests.push((Arc::new(test), display_err));
        self
    }

    /// Add a test over the parsed input. Errors will not be displayed if they occur.
    ///
    /// This is shorthand for `self.test_with_msg(parser, false)`.
    pub fn test(self, test: impl Fn(&T) -> eyre::Result<()> + 'static) -> Self {
        self.test_with_msg(test, false)
    }

    /// Forgets all tests.
    pub fn clear(mut self) -> Self {
        self.tests = vec![];
        self
    }

    pub fn quick_test(self, quick_test: impl Fn(&T) -> bool + 'static) -> Self {
        let test = move |value: &T| match quick_test(value) {
            true => Ok(()),
            false => Err(Report::msg("Failed a test.")),
        };
        self.test(test)
    }

    pub fn quick_test_with_msg(
        self,
        quick_test: impl Fn(&T) -> bool + 'static,
        message: impl ToString + 'static,
    ) -> Self {
        let test = move |value: &T| match quick_test(value) {
            true => Ok(()),
            false => Err(Report::msg("Failed a test.").wrap_err(message.to_string())),
        };
        self.test(test)
    }

    // TODO: more test options
}

/// # Useful
impl<T, R, W> QuestionBuilder<T, R, W> {
    ///
    /// # Remarks
    ///
    /// Default values are tested, so make sure that it is a value that passes your tests!
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

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
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
            parser: self.parser,
            tests: self.tests,
            timeout: self.timeout,
            attempts: self.attempts,
            required: self.required,
        }
    }
    pub fn feedback(mut self, feedback: impl Fn(&T) -> String + 'static) -> Self {
        self.feedback = Arc::new(feedback);
        self
    }

    /// Set the parser for the input and choose to display the error, if one occurs while parsing.
    pub fn parser_with_msg(
        mut self,
        parser: impl Fn(&str) -> eyre::Result<T> + 'static,
        display_err: bool,
    ) -> Self {
        self.parser = (Arc::new(parser), display_err);
        self
    }

    /// Set the parser for the input. Errors will not be displayed if they occur.
    ///
    /// This is shorthand for `self.parser_with_msg(parser, false)`.
    pub fn parser(self, parser: impl Fn(&str) -> eyre::Result<T> + 'static) -> Self {
        self.parser_with_msg(parser, false)
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
            println!("Started iteration!");
            self.write_message().await?;
            let input = self.take_input().await?;
            println!("input: {:?}", input);
            self.decrease_attempts()?;
            if input.is_empty() && !self.required && self.default.is_some() {
                println!("Going for the default!");
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

    async fn parse_input(&mut self, input: &str) -> eyre::Result<T> {
        if input == "" || self.required {
            Err(crate::error::Looping::Required(
                "Answer can not be empty.".to_string(),
            ))?;
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
