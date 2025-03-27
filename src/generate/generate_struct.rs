use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Generate {
    amount: i32
}