# asking
[![Download](https://img.shields.io/crates/d/asking)](https://crates.io/crates/asking)
[![License](https://img.shields.io/crates/l/asking)](https://github.com/saona-raimundo/asking)
[![Docs](https://docs.rs/asking/badge.svg)](https://docs.rs/asking/)
[![Crate](https://img.shields.io/crates/v/asking.svg)](https://crates.io/crates/asking)

Build async prompts.

## About

Ever wanted non-blocking user input? Here you are!

## Features

- **Async** - You can work while the user inputs something and even timeout!
- **Common patterns** - Built-in common question patterns including `confirm`, `date`, `select`, `multiselect`, `password`, `text` and more! (TODO: link to the actual structs)
- **Cross-platform** - It is actually generic on writer and reader!
- **Standardized error handling**
- **Customized help messages** - Help the user input a correct answer.
- **Default values** - 
- **Validation** - Include tests that the input has to pass.
- **Custom formatting** - Display back the input as you wish. (TODO)
- **Total control** - Any decision can be reprogrammed!

## Limitations

Let me know about more, I will be happy to add them!

## Quick example

(TODO)



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
