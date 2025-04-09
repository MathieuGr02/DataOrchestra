use crate::ssh::ssh_struct::ssh;

use super::{super::common::common_trait::Start, store_struct::Store};
use std::{path::Path, thread::{self, JoinHandle}};
use log::info;

impl Start<()> for Store {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `store.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        info!("Spawning storing thread");
        
        thread::Builder::new().name("store".to_string()).spawn(move || {
            let mut ssh: Option<ssh> = None;
            if let Some(ref mut docker) = self.docker {
                docker.init();
                ssh = Some(docker.get_ssh());
            }

            if let Some(ref mut ssh) = ssh {
                ssh.exec("pwd");
            }
        }).unwrap()
    }
}

