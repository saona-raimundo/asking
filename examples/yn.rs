extern crate chrono;
use std::time::Duration;

fn main() {
    let ans = async {
        let mut question = asking::yn();
        question
            .message("Shall I continue? (you have 5 seconds to answer)")
            .default_value(true)
            .timeout(Duration::from_secs(5_u64));
        question.ask().await
    };

    match async_std::task::block_on(ans) {
        Ok(true) => println!("Super!"),
        Ok(false) => println!("Okay, shutting down..."),
        Err(report) => {
            if report.is::<async_std::future::TimeoutError>() {
                println!("I think you are not here, I will continue :)")
            } else {
                eprintln!("Error with questionnaire, try again later")
            }
        }
    }
}
