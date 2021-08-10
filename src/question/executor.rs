use std::time::Duration;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Executor {
    None,
    Timeout(Duration),
}
