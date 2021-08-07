extern crate chrono;
use asking::error::Processing;
use std::time::Duration;

fn main() {
    let ans = async {
        asking::yn()
            .message("Shall I continue? (you have 5 seconds to answer)")
            .feedback(|b| {
                if *b {
                    "Great!\n".to_string()
                } else {
                    "Too bad\n".to_string()
                }
            })
            .help("You can do it!\n")
            .quick_test_with_msg(|b| !b, "Think hard!\n")
            .required(true)
            .timeout(Duration::from_secs(50_u64))
            .ask()
            .await
    };

    match async_std::task::block_on(ans) {
        Ok(true) => println!("Super!"),
        Ok(false) => println!("Okay, shutting down..."),
        Err(p) => match p {
            Processing::Timeout { .. } => {
                println!("I think you are not here, I will continue :)")
            }
            _ => eprintln!("Error with questionnaire, try again later"),
        },
    }
}
