use std::process::{Child, Output};

pub trait Remote {
    fn connect(&self);
    fn execute(&self, arg: &str) -> Child;
    fn get_ip(&self);
    fn get_port(&self);
    fn get_host(&self);
} 
