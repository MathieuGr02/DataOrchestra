use crate::{address::Address, docker::docker_struct::Docker, remote::remote_trait::Remote};

pub struct TerminalSession<T> {
    pub address: Address,
    pub session_type: T
}

impl Remote for TerminalSession<Docker> {
    fn connect(&self) {
        
    }
    
    fn execute(&self, arg: &str) -> std::process::Child {
        
    }

    fn get_host(&self) {
        
    }

    fn get_port(&self) {
        
    }

    fn get_ip(&self) {
        
    }
}


