//! Build async prompts for non-blocking user input!
//!
//! [Asynchronous I/O] allows the (usually slow) I/O operations run concurrently with
//! the rest of your (highlt efficient) code.
//!
//! [Asynchronous I/O]: https://en.wikipedia.org/wiki/Asynchronous_I/O
//!
//! # Features
//!
//! - **[Asynchronous]** - You can work while the user inputs something and even timeout!
//! - **Common patterns** - Built-in common question patterns including
//!   - **[`yn`]** - yes/no questions.
//!   - **[`date`]** - dates in `%Y-%m-%d` format.
//!   - **[`select`]** - choose one option.
//!   - **[`text`]** - just a String.
//!   - **[`T`]** - your own type! (implementing or not the trait [`FromStr`]).
//! - **Cross-platform** - Generic on [`reader`] and [`writer`]!
//! - **[`Help`] messages** - Help the user to input a correct answer.
//! - **[`Test`] with feedback** - Test the input and, optionally, give feedback upon errors.
//! - **[Default values]** - Add a value for empty inputs.
//! - **Standardized [`error`] handling** - You can manage errors!
//! - **[`Feedback`]** - Display a final message depending on the accepted value.
//!
//! [Asynchronous]: struct.QuestionBuilder.html#method.ask
//! [`yn`]: fn.yn.html
//! [`date`]: fn.date.html
//! [`select`]: fn.select.html
//! [`T`]: struct.QuestionBuilder.html#method.new
//! [`FromStr`]: https://doc.rust-lang.org/core/str/trait.FromStr.html
//! [`reader`]: struct.QuestionBuilder.html#method.reader
//! [`writer`]: struct.QuestionBuilder.html#method.writer
//! [`Help`]: struct.QuestionBuilder.html#method.help
//! [`Test`]: struct.QuestionBuilder.html#method.test
//! [Default values]: struct.QuestionBuilder.html#method.default_value
//! [`error`]: error/enum.Processing.html
//! [`Feedback`]: struct.QuestionBuilder.html#method.feedback
//!
//! # Quick example
//!
//! Give only five seconds to the user to confirm something, and continue upon no input! (instead of keep waiting)
//!
//! ```
//! use asking::error::ProcessingError;
//! use std::time::Duration;
//!
//! let question = asking::yn()
//!     .message("Shall I continue? (you have 5 seconds to answer)")
//!     .default_value(true) // value upon empty input
//!     .timeout(Duration::from_secs(5_u64))
//!     .ask();
//!
//! match async_std::task::block_on(question) { // we decide to just wait, at most five secs
//!     Ok(true) => println!("Super!"),
//!     Ok(false) => println!("Okay, shutting down..."),
//!     Err(ProcessingError::Timeout { .. }) => println!("I think you are not here, I will continue :)"), // Automatic decision!,
//!     _ => eprintln!("Error with questionnaire, try again later"),
//! }
//! ```
//!
//! Check out [more examples](https://github.com/saona-raimundo/asking/tree/main/examples)!

// Testing code in README.md
#[cfg(doctest)]
doc_comment::doctest!("../README.md");

/// Errors while asking a question.
pub mod error;
mod pattern;
mod question;

pub use pattern::{date, question, select, select_with_msg, text, yn};
pub use question::{QuestionBuilder, StdQuestionBuilder};
