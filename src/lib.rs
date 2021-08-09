//! Build async prompts for non-blocking user input!
//!

// Testing code in README.md
#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod error;
mod pattern;
mod question;

pub use pattern::{date, question, text, yn};
pub use question::{QuestionBuilder, StdQuestionBuilder};
