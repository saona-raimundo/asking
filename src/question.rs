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
    parser: Arc<dyn Fn(&str) -> eyre::Result<T>>,
    tests: Vec<Arc<dyn Fn(&T) -> eyre::Result<()>>>,
    timeout: Option<Duration>,
    attempts: Option<usize>,
}

/// # Message-related
impl<T, R, W> QuestionBuilder<T, R, W> {
    pub fn message<O: ToString>(&mut self, message: O) -> &mut Self {
        self.message.0 = message.to_string();
        self
    }
    pub fn message_repeat(&mut self, message_repeat: bool) -> &mut Self {
        self.message.1 = message_repeat;
        self
    }
    pub fn repeat_message<O: ToString>(&mut self, message: O) -> &mut Self {
        self.message(message).message_repeat(true)
    }

    pub fn help<O: ToString>(&mut self, help: O) -> &mut Self {
        self.help.0 = help.to_string();
        self
    }
    pub fn help_repeat(&mut self, help_repeat: bool) -> &mut Self {
        self.help.1 = help_repeat;
        self
    }
    pub fn repeat_help<O: ToString>(&mut self, help: O) -> &mut Self {
        self.help(help).help_repeat(true)
    }
}

/// # Test-related
impl<T, R, W> QuestionBuilder<T, R, W> {
    pub fn test(&mut self, test: impl Fn(&T) -> eyre::Result<()> + 'static) -> &mut Self {
        self.tests.push(Arc::new(test));
        self
    }

    /// Forgets all tests.
    pub fn clear(&mut self) -> &mut Self {
        self.tests = vec![];
        self
    }

    pub fn quick_test(&mut self, quick_test: impl Fn(&T) -> bool + 'static) -> &mut Self {
        let test = move |value: &T| match quick_test(value) {
            true => Ok(()),
            false => Err(Report::msg("Failed a test.")),
        };
        self.test(test)
    }

    pub fn quick_test_with_msg(
        &mut self,
        quick_test: impl Fn(&T) -> bool + 'static,
        message: impl ToString + 'static,
    ) -> &mut Self {
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
    pub fn default_value<S: Into<Option<T>>>(&mut self, value: S) -> &mut Self {
        self.default = value.into();
        self
    }

    pub fn timeout<O: Into<Option<Duration>>>(&mut self, duration: O) -> &mut Self {
        self.timeout = duration.into();
        self
    }

    pub fn attempts<O: Into<Option<usize>>>(&mut self, attempts: O) -> &mut Self {
        self.attempts = attempts.into();
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
        }
    }
    pub fn feedback(&mut self, feedback: impl Fn(&T) -> String + 'static) -> &mut Self {
        self.feedback = Arc::new(feedback);
        self
    }

    pub fn parser(&mut self, parser: impl Fn(&str) -> eyre::Result<T> + 'static) -> &mut Self {
        self.parser = Arc::new(parser);
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
    pub async fn ask(mut self) -> eyre::Result<T> {
        write!(self.writer, "{}", &self.message.0).await?;
        self.writer.flush().await?;
        if let Some(duration) = self.timeout {
            Ok(async_std::future::timeout(duration, self.loop_for_input()).await??)
        } else {
            Ok(self.loop_for_input().await?)
        }
    }

    async fn loop_for_input(mut self) -> eyre::Result<T> {
        let mut input = String::new();
        loop {
            self.reader.read_line(&mut input).await?;
            // Preprocessing
            if input.ends_with('\n') {
                input.pop();
                if input.ends_with('\r') {
                    input.pop();
                }
            }

            if input.is_empty() {
                if let Some(x) = self.default {
                    return Ok(x);
                }
            };

            match self.parse_and_test(input.clone()) {
                Ok(x) => return Ok(x),
                Err(e) => {
                    write!(self.writer, "{}", e).await?;
                    self.writer.flush().await?;
                }
            };

            if self.message.1 {
                write!(self.writer, "{}", self.message.0).await?;
                self.writer.flush().await?;
            };
        }
    }

    fn parse_and_test(&self, string: String) -> eyre::Result<T> {
        let value: T = string.parse()?;
        for test in &self.tests {
            test(&value)?;
        }
        Ok(value)
    }
}
