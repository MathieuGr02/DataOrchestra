use super::{super::common::common_trait::Start, process_struct::Process};
use std::{path::Path, thread::{self, JoinHandle}};
use log::info;

impl Start<()> for Process {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `process.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        info!("Spawning process thread");
        
        thread::Builder::new().name("process".to_string()).spawn(move || {
            if let Some(ref mut docker) = self.docker {
                docker.init();
                let ssh = docker.get_ssh();
                let result = ssh.exec("pwd");
                println!("{}", result);
                //let result = ssh.upload(&Path::new(&self.initialisation_script.unwrap()), &Path::new("/config.json"));
                /*
                if !init_docker_success.await {
                    error!("Unable to create docker container for store \"{}\"", &docker.name);
                    return;
                }
                else {
                    info!("Successfully created docker container for store \"{}\"", &docker.name);
                }
                */
            }
        }).unwrap()
    }
}
