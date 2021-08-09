// use asking::error::Processing;
use asking::error::Processing;
use async_std::{fs::OpenOptions, prelude::*, task};
use core::time::Duration;

#[async_std::main]
async fn main() -> eyre::Result<()> {
    let file_in = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("in.txt")
        .await?;

    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open("out.txt")
        .await?;

    let question = asking::QuestionBuilder::new_fromstr(file_in, file_out)
        // .message("Shall I continue? (you have 5 seconds to answer)")
        // .help("Format true/false")
        // .timeout(Duration::from_secs(5_u64))
        .ask();

    is_send(question);

    let child = task::block_on(question);

    match child {
        Ok(true) => println!("Super!"),
        Ok(false) => println!("Okay, shutting down..."),
        Err(Processing::Timeout { .. }) => {
            println!("I think you are not here, I will continue :)")
        }
        _ => eprintln!("Error with questionnaire, try again later"),
    }

    Ok(())
}

fn is_send<T: Send>(t: T) {}
