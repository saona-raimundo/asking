# asking
[![Download](https://img.shields.io/crates/d/asking)](https://crates.io/crates/asking)
[![License](https://img.shields.io/crates/l/asking)](https://github.com/saona-raimundo/asking)
[![Docs](https://docs.rs/asking/badge.svg)](https://docs.rs/asking/)
[![Crate](https://img.shields.io/crates/v/asking.svg)](https://crates.io/crates/asking)

Build async prompts.

## About

Ever wanted non-blocking user input? Here you are!

## Roadmap

- **Customized help messages** - Help the user input a correct answer.
- **Custom formatting** - Display back the input as you wish. (TODO)
- **Total control** - Any decision can be reprogrammed!

## Features

- **Asynchronous** - You can work while the user inputs something and even timeout!
- **Common patterns** - Built-in common question patterns including 
  - `yn` - yes/no questions.
  - `date` - uses `chrono::naive::NaiveDate`.
  - `select` - choose one option.
  - `multiselect` - choose options out of some alternatives.
  - `text` - just a String
  - `T: std::str::FromStr` - your own type!
- **Cross-platform** - It is actually generic on writer and reader!
- **Default values** - Add a value for empty inputs.
- **Standardized error handling** - You can manage errors!
- **Feedback** - Display a message in response to the input.
- **Validation** - Include tests that the input has to pass.


## Limitations

- **Internal mutability of functions** - All functions passed are `Arc<dyn Fn>`. This  allows both functions and closures, but it means that functions can not hold any mutable references inside.
- **Consuming methods** - Methods are consuming allowing one-line constructions, while making more difficult complex construction patterns. This is because of the existence of default values. Check out [C-BUILDER](https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder).

Let me know about more, I will be happy to add them!

## Quick example

(TODO: Add timeout)

```rust
extern crate chrono;
use std::time::Duration;

let ans = async {
    asking::yn().message("Shall I continue? (you have 5 seconds to answer)")
        .default_value(true)
        .timeout(Duration::from_secs(5_u64)).ask().await
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
```



Check out [more examples](https://github.com/saona-raimundo/asking/tree/main/examples)!

## Usage

(TODO)

## Optional features

(TODO)

## Related crates

There are several crates for handling user input, I recommend checking them all out! 

- [ask](https://crates.io/crates/ask) 
  A simple toolset for asking questions through the terminal.
- [inquire](https://crates.io/crates/inquire) 
  Library for building interactive prompts on terminals.

- [question](https://crates.io/crates/question)
  Ask a question, what more could you want?
- [read_input](https://crates.io/crates/read_input)
  A simple CLI tool that asks for user input until the data inputted is valid.
- [termion](https://docs.rs/termion/1.5.6/termion/index.html)::[AsyncReader](https://docs.rs/termion/1.5.6/termion/struct.AsyncReader.html)
  An asynchronous reader.
- [timeout-readwrite](https://crates.io/crates/timeout-readwrite)
  Adds timeout capabilities to Readers and Writers.

If you've got a crate that would be a good fit, open an issue and let me know. I'd love to add it!

### Good matchups

Some crates are good to use together!

- [async-dup](https://crates.io/crates/async-dup) - Duplicate an I/O handle

## FAQ

### Testing

Testing projects with user input can be challenging.

- How to give input to Stdin and read from Stdout?
  - https://doc.rust-lang.org/std/process/index.html
  - https://stackoverflow.com/questions/21615188/how-to-send-input-to-a-program-through-stdin-in-rust/32069040
