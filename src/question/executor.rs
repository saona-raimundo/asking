use std::time::Duration;

#[derive(Debug)]
pub enum Executor {
    BlockOn,
    None,
    Timeout(Duration),
}
