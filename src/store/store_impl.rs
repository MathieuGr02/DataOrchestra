use std::collections::HashMap;
use std::fmt::format;
use std::io::read_to_string;
use std::process::Command;
use super::{super::common::common_trait::Start, store_struct::Store};
use std::thread;
use log::{info, error};

impl Start for Store {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `store.start()` moves the store into the thread
    fn start(mut self) {
        info!("Spawning storing thread");
        
        let store_thread = thread::spawn(move || {
            let init_docker_success = self.init_docker();
            if !init_docker_success {
                error!("Unable to create docker container for store : [{}]", self.image);
                return;
            }
            
            let init_structure_success = self.init_structure();
            if !init_structure_success {
                error!("Unable to run initialisation script for store : [{}]", self.image);
                return;
            }
        });
        
        store_thread.join().expect("TODO: panic message");
    }
}

impl Store {
    /// Initialise docker container
    pub fn init_docker(&self) -> bool {
        info!("Initializing docker");

        let docker_run = Command::new("cmd")
            .arg(format!("/C docker run --name store {}", build_postgres_container(&self.store_options)))
            .spawn()
            .expect("Failed to initialize Docker Container")
            .wait();

        docker_run.is_ok()
    }
    
    /// Initialise relation structure of database
    pub fn init_structure(&self) -> bool {
        info!("Initializing structure");
        true
    }
}


pub struct Factory {
    value: String
}

impl FactoryA for Factory {
    fn add(mut self, var: &str) -> Self {
        self.value.push_str(format!(" {} ", var).as_str());
        self
    }

    fn build(&self) -> &String {
        &self.value
    }
}

pub trait FactoryA {
    fn add(self, var: &str) -> Self;
    fn build(&self) -> &String;
}

pub fn build_postgres_container(options: &Option<HashMap<String, String>>) -> String {
    let mut command: String = String::from("-it postgres");

    let option_map: &HashMap<String, String>;

    match options {
        Some(value) => option_map = value,
        None => return command
    }

    if let Some(value) = option_map.get("POSTGRES_PASSWORD") {
        command = format!("{command} -e POSTGRES_PASSWORD={value}");
    }

    command
}