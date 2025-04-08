use serde::Deserialize;

use crate::docker::docker_struct::Docker;

#[derive(Debug, Deserialize)]
pub struct Generate {
    #[serde(default = "default_amount")]
    pub amount: usize,
    pub docker: Option<Docker>
}

pub fn default_amount() -> usize {
    1
}
