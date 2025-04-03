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
            if let Some(ref mut docker) = self.docker {
                let init_docker_success = docker.init();
                if !init_docker_success {
                    error!("Unable to create docker container for store \"{}\"", &docker.name);
                    return;
                }
                else {
                    info!("Successfully created docker container for store \"{}\"", &docker.name);
                }
            }
        });
        
        store_thread.join().expect("Unable to join store thread");
    }
}

impl Start for Vec<Store> {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `store.start()` moves the store into the thread
    fn start(self) {
        // Set unique names
        let mut names = Vec::<String>::new();
        for mut task in self {
            if let Some(docker) = &task.docker{
                let mut new_name = docker.name.clone();
                let mut amount = 1;
                while names.contains(&new_name) {
                    new_name = format!("{new_name}-{amount}");
                    amount += 1;
                }
                names.push(new_name.clone());

                if let Some(docker_ref) = task.docker.as_mut() {
                    docker_ref.name = new_name;
                }
            }

            task.start();
        }
    }
}