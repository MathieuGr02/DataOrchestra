use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use serde::{Deserialize, Serialize};

pub fn default_name() -> String {
    String::from("store")
}

pub fn default_network() -> String {
    String::from("orchestra")
}

pub fn default_port() -> u16 {
    5000
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
    pub ip: Option<IpAddr>,
    #[serde(default = "default_port")]
    pub port: u16
}