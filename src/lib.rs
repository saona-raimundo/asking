//! Build async prompts for non-blocking user input!

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod error;
mod pattern;
mod question;

pub use pattern::{date, question, text, yn};
pub use question::{QuestionBuilder, StdQuestionBuilder};
