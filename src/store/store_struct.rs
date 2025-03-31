use std::collections::HashMap;
use serde::{Deserialize, Serialize};

fn default_name() -> String {
    "store".to_string()
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    // Container image
    pub image: String,
    // Relation structure
    pub initialisation_script: Option<String>,
    // Container name
    #[serde(default = "default_name")]
    pub name: String,
    // Additional options
    pub docker_options: ContainerOptions,
    pub store_options: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContainerOptions {
    // https://docs.docker.com/reference/cli/docker/container/run/#add-host
    pub host: String,
    pub ip: i8,
    // https://docs.docker.com/reference/cli/docker/container/run/#publish
    pub expose_port: i8,
    // https://docs.docker.com/reference/cli/docker/container/run/#memory
    pub memory_limit: i8,
    // https://docs.docker.com/reference/cli/docker/container/run/#gpus
    pub gpus: String
}





