# TODO

How to save functions?

- v: Vec<Mutex<dyn Fn()>>, // document this!
- https://stackoverflow.com/questions/64298245/in-rust-what-is-fn
- https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

## Chaining methods

- Check out [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder)

## Documentation

- Add gifs as in [inqiure](https://crates.io/crates/inquire)

## Examples

- Complete as in [read_input]!

## Main code

- Design the process
  - formatter
  - parser
- Review `Prompt functionalities`
  - By hand
  - test
  - Ordered tests?
    - required test should have priority, no?

## Tests

- required() // answer not null, check with default value
- length(usize) // of the input
- max_length 
- min_length 
- inside(iterator) // required for `select` and `multiple_select`
- min(lower_bound) // T: PartialOrd
- max(upper_bound) // T: PartialOrd
- min_max(lower_bound, upper_bound)
- not(other) // T: PartialEq
- suggest(impl Fn(&str) -> Vec<String>) // Maybe

All variants with `with_msg` method.
