use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Store {
    name: String,
    initialisation_script: String
}