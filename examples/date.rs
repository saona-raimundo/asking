//! Input a date and get back how long it is until then!
//!
//! This examples shows how to use `date`.

use chrono::offset::Local;

fn main() {
    let awaited_date = async_std::task::block_on(
        asking::date()
            .message("Please input your awaited date: ")
            .min(Local::today().naive_local())
            .help("Use a %Y-%m-%d format please.\n")
            .feedback(|_| "Thank you!".to_string())
            .ask(),
    )
    .expect("Failed to read line");

    let offset = awaited_date.signed_duration_since(Local::today().naive_local());
    println!(
        "There are {} weeks, and {} days left!",
        offset.num_weeks(),
        offset.num_days() - 7 * offset.num_weeks()
    )
}
