//! Build async prompts for non-blocking user input!

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod error;
mod pattern;
mod question;

pub use pattern::{date, password, question, select, text, yn};
pub use question::{QuestionBuilder, StdQuestionBuilder};
