# asking
[![Download](https://img.shields.io/crates/d/asking)](https://crates.io/crates/asking)
[![License](https://img.shields.io/crates/l/asking)](https://github.com/saona-raimundo/asking)
[![Docs](https://docs.rs/asking/badge.svg)](https://docs.rs/asking/)
[![Crate](https://img.shields.io/crates/v/asking.svg)](https://crates.io/crates/asking)
[![dependency status](https://deps.rs/crate/asking/latest/status.svg)](https://deps.rs/crate/asking)

Build async prompts.

## About

Ever wanted non-blocking user input? Here you are!

[Asynchronous I/O](https://en.wikipedia.org/wiki/Asynchronous_I/O) is a form of input/output processing that allows you to do something while waiting for an answer.

## Features

- **Asynchronous** - You can work while the user inputs something and even timeout!
- **Common patterns** - Built-in common question patterns including 
  - **yn** - yes/no questions (see `yn` function).
  - **date** - dates in `%Y-%m-%d` format (see `date` function).
  - **select** - choose one option (see `inside` method).
  - **text** - just a String (see `text` function).
  - **T** - your own type! (implementing or not the trait `FromStr`).
- **Cross-platform** - Generic on writer and reader!
- **Help messages** - Help the user to input a correct answer.
- **Test with feedback** - Test the input and, optionally, give feedback upon errors.
- **Default values** - Add a value for empty inputs.
- **Standardized error handling** - You can manage errors!
- **Feedback** - Display a final message depending on the accepted value.
- **Extensive documentation** - If you do not think so, let me know!


## Limitations

- **Internal mutability of functions** - All functions are stored as `Arc<dyn Fn>`. This  allows both functions and closures, but it means that functions can not hold any mutable references (so no internal mutability).
- **Send + Sync + 'static** - To allow asynchronous execution, at the end of the day, all parameters have to implement this traits. This way you can truly harness async execution of questions. If you manage to use something that does not implement these traits, then the future can only be executed synchronously. 
- **Consuming methods** - Methods are consuming allowing one-line constructions, while making more difficult complex construction patterns. This is because of the existence of default values. Check out [C-BUILDER](https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder).

Let me know if you find anything else, I will be happy to add it!

## Quick example

Give only five seconds to the user to confirm something, and continue upon no input! (instead of keep waiting)

```rust,ignore
use asking::error::ProcessingError;
use std::time::Duration;

let question = asking::yn()
    .message("Shall I continue? (you have 5 seconds to answer)")
    .default_value(true) // value upon empty input
    .timeout(Duration::from_secs(5_u64))
    .ask();

match async_std::task::block_on(question) { // we decide to just wait, at most five secs
    Ok(true) => println!("Super!"),
    Ok(false) => println!("Okay, shutting down..."),
    Err(ProcessingError::Timeout { .. }) => println!("I think you are not here, I will continue :)"), // Automatic decision!,
    _ => eprintln!("Error with questionnaire, try again later"),
}
```



Check out [more examples](https://github.com/saona-raimundo/asking/tree/main/examples)!

## Usage

With [cargo-edit](https://crates.io/crates/cargo-edit) installed, simply type

```ignore
cargo add asking
```

and you are good to go!

## Related crates

There are several crates for handling user input, I recommend checking them all out! 

- [ask](https://crates.io/crates/ask) 
  A simple toolset for asking questions through the terminal.
- [dialoguer](https://crates.io/crates/dialoguer)
  A command line prompting library.
- [inquire](https://crates.io/crates/inquire) 
  Library for building interactive prompts on terminals.
- [promptly](https://crates.io/crates/promptly)
  Simple, opinionated CLI prompting helper.
- [requestty](https://crates.io/crates/requestty)
  An easy-to-use collection of interactive cli prompts.
- [rprompt](https://crates.io/crates/rprompt)
  Prompt user input in console applications.
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
  The easiest way is using the `assert_cmd` crate. Check out
  - Example `testing`.
  - Folder `tests` of this repository.
  - For more, go to [Command line apps in Rust book](https://rust-cli.github.io/book/tutorial/testing.html).

## For the reader

Has this project been useful to you? How? :)
Let me know through a github issue!
