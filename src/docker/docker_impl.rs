use std::collections::HashMap;
use std::fmt::format;
use std::process::Command;
use log::{error, info, warn};
use crate::docker::docker_struct::Docker;
use crate::command::command_func::{output_command, spawn_command, spawn_commands};

impl Docker {
    /// Create docker container based on specified data
    pub fn init(&mut self) -> bool {
        info!("Initializing docker container");

        // Create network
        let networks: String = output_command("docker network ls");
        if !networks.contains(&self.network) {
            info!("{}", format!("Creating network bridge {}", &self.network));
            let create_bridge = spawn_command(&format!("docker network create -d bridge {}", &self.network)).wait();

            if create_bridge.is_err() || (create_bridge.is_ok() && !&create_bridge.as_ref().unwrap().success()) {
                warn!("Unable to create bridge {} | Code : {}", &self.network, &create_bridge.unwrap().code().unwrap());
            }
            else {
                info!("Successfully created bridge \"{}\"", &self.network)
            }
        }

        // Create image
        let containers: String = output_command("docker ps");
        if containers.contains(&self.name){
            warn!("Re initiating container {}", &self.name);
            let container_stop = spawn_command(&format!("docker stop {}", &self.name)).wait();
            let container_rm = spawn_command(&format!("docker rm {}", &self.name)).wait();

        }
        let docker = spawn_command(&format!("docker run {}", self.get_options())).wait();
        if docker.is_err() || (docker.is_ok() && !&docker.as_ref().unwrap().success()) {
            warn!("Unable to create docker container \"{}\" | Code : {}", &self.name, &docker.unwrap().code().unwrap());
        }
        else {
            info!("Successfully created docker container \"{}\"", &self.name)
        }

        // get ip
        let result = Command::new("sh")
            .arg("-c")
            .arg("docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' store")
            .output();

        match result {
            Ok(value) => info!("Docker container \"{}\" ip : {:?}", &self.name, String::from_utf8(value.stdout).unwrap()),
            Err(value) => warn!("Unable to retrieve ip address from {} | Code : {}", &self.name, value),
        };
        
        true
    }

    /// Get docker container options as command input
    pub fn get_options(&self) -> String {
        let mut command: String = String::from("-d -q");

        command = format!("{command} --network={}", &self.network);

        command = format!("{command} --name={}", &self.name);

        command = format!("{command} -p 127.0.0.1:{}:{}", &self.port, &self.port % 100);

        if let Some(options) = &self.options {
            for (key, value) in options {
                command = format!("{command} -e {key}={value}")
            }
        }

        if let Some(value) = &self.image {
            command = format!("{command} -it {value}");
        }

        if let Some(source) = &self.mount {
            if let Some(target) = &self.target {
                command = format!("{command} --mount source={source}, target={target}");
            }
        }

        command
    }
}