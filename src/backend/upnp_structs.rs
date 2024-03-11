use std::net::IpAddr;

use serde::{Deserialize, Serialize};

/// A struct for mappings
#[derive(Debug, Deserialize, Serialize)]
pub struct Mapping {
    /// the ip the incoming traffic should be sent to
    pub dest_ip: IpAddr,
    /// the port the incoming traffic should be sent to
    pub dest_port: u16,
    /// the external port that should be used
    pub ext_port: u16,
    /// the description of the port mapping
    pub description: String,
}

impl Mapping {
    pub fn new(dest_ip: IpAddr, dest_port: u16, ext_port: u16, description: String) -> Self {
        Self {
            dest_ip,
            dest_port,
            ext_port,
            description,
        }
    }
}
