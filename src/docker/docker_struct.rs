use std::{collections::HashMap, net::{IpAddr, Ipv4Addr}};
use serde::{Deserialize, Serialize};
use crate::address::Address;

pub fn default_name() -> String {
    String::from("store")
}

pub fn default_network() -> String {
    String::from("orchestra")
}

pub fn default_address() -> Address {
    Address {
        ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port: 5000,
        internal_port: 50
    }
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
    #[serde(default = "default_address")]
    pub address: Address
}
