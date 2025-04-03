use serde::{Deserialize, Serialize};
use crate::docker::docker_struct::Docker;

#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    // Relation structure
    pub initialisation_script: Option<String>,
    // Additional options
    pub docker: Option<Docker>,
}





