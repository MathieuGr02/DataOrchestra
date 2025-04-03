use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::address::Address;

pub fn default_name() -> String {
    String::from("store")
}

pub fn default_network() -> String {
    String::from("orchestra")
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Docker {
    #[serde(default = "default_name")]
    pub name: String,
    pub image: Option<String>,
    #[serde(default = "default_network")]
    pub network: String,
    pub mount: Option<String>,
    pub target: Option<String>,
    // Additional options
    pub options: Option<HashMap<String, String>>,
    pub address: Option<Address>
}
