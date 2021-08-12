//! Choose one option.
//!
//! This examples shows the use of
//! - `select`

fn main() {
    let options = ["A".to_string(), "B".to_string()]; // You may as well define an enum
    let ans = asking::select_with_msg(options.clone(), format!("Options available {:?}", options))
        .message("Which option should I go for? ")
        .help("Try again: ")
        .ask();

    match async_std::task::block_on(ans) {
        Ok(value) => match value.as_str() {
            "A" => println!("A it is!"),
            "B" => println!("B it is!"),
            _ => unreachable!(),
        },
        Err(_) => eprintln!("Error with questionnaire, try again later."),
    }
}
