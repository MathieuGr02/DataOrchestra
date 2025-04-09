use crate::ssh::ssh_struct::ssh;

use super::{super::common::common_trait::Start, generate_struct::Generate};
use core::panic;
use std::thread::{self, JoinHandle};
use log::info;

impl Start<()> for Generate {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `generate.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        info!("Spawning generate thread");
        
        thread::Builder::new().name("generate".to_string()).spawn(move || {
            let mut ssh: Option<ssh> = None;

            if let Some(ref mut docker) = self.docker {
                docker.init();
                ssh = Some(docker.get_ssh());
            }
            
            if let Some(ref mut ssh) = ssh {
                ssh.exec("pwd");
            }
            else {
                panic!("Unable to get ssh client");
            }
        }).unwrap()
    }
}
