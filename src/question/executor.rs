use std::time::Duration;

#[derive(Debug)]
pub enum Executor {
    None,
    Timeout(Duration),
}
