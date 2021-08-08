//! Input a point in 2D space.
//!
//! This examples shows how to:
//! - Deal with types that do NOT implement FromStr

use thiserror::Error;

fn main() {
    async_std::task::block_on(
        asking::StdQuestionBuilder::from(&parser)
            .parser_feedback_toggle() // Parse errors will be printed
            .message("Please input a point in 2D space.\n")
            .help("Use a `x,y` format please :)\n")
            .feedback(|point| format!("Thank you! Your point is {:?}", point))
            .ask(),
    )
    .expect("Failed to read line");
}

/// Custom error for our parser.
#[derive(Error, Debug)]
enum MyError {
    #[error("Parsing failed.")]
    Parse {
        #[from]
        source: core::num::ParseFloatError,
    },
    #[error("Wrong amount of coordinates.")]
    WrongDimension(usize),
}

/// Very basic parser.
///
/// Accepts only the form `x,y`, ie comma separated values.
fn parser(s: &str) -> Result<(f64, f64), MyError> {
    let mut values = Vec::new();
    for sub_s in s.split(',') {
        values.push(sub_s.parse()?)
    }
    let dimensions = values.len();
    if dimensions == 2 {
        Ok((values[0], values[1]))
    } else {
        Err(MyError::WrongDimension(dimensions))
    }
}
