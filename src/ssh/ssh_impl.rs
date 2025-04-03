use crate::remote::remote_trait::Remote;
use std::net::TcpStream;
use std::process::Output;

use std::error::Error;

use ssh2::Session;

/*
impl Remote for Ssh {
    fn connect(&self) {
        let tcp = TcpStream::connect(&self) 
    }

    fn execute(&self, arg: &str) -> std::process::Child {
        
    }

    fn get_ip(&self) {
        
    }

    fn get_port(&self) {
        
    }

    fn get_host(&self) {
        
    }
}

*/
