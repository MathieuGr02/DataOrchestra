use std::{fs, thread, env};
use std::fs::File;
use std::path::Path;
use std::process::{Child, Command};
use serde::{Deserialize};

use HeterogeneousDataOrchester::generate::generate_struct::Generate;
use HeterogeneousDataOrchester::process::process_struct::Process;
use HeterogeneousDataOrchester::store::store_struct::Store;

#[derive(Debug, Deserialize)]
struct Config {
    process: Process,
    generate: Generate,
    store: Store
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    let s = fs::read_to_string("config.json").expect("Failed to load config.json");
    let config_struct: Config = serde_json::from_str(&s).expect("Failed to parse JSON");
    println!("{:?}", config_struct);

    let docker = thread::spawn(|| {
        let x = spawn_command("docker run -d --name Database -it ubuntu").wait();
        let z = spawn_command("docker exec -it Database mkdir TEST").wait();
    });

    let x = docker.join();
}

pub fn spawn_command(command: &str) -> Child {
    Command::new("cmd")
        .args(["/C" , command])
        .spawn()
        .expect("Failed to execute Docker command")
}

