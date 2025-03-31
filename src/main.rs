use std::env;
use std::fs::File;
use std::path::Path;
use std::ptr::copy;
use log::{debug, info, LevelFilter};
use serde::{Deserialize};

use HeterogeneousDataOrchester::common::common_trait::Start;
use HeterogeneousDataOrchester::generate::generate_struct::Generate;
use HeterogeneousDataOrchester::process::process_struct::Process;
use HeterogeneousDataOrchester::store::store_struct::Store;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Amount<T> {
    Single(T),
    Multiple(Vec<T>)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
struct Config {
    process: Amount<Process>,
    generate: Amount<Generate>,
    store: Amount<Store>
}

fn main() {
    // Read starting arguments
    let args: Vec<String> = env::args().collect();
    initialise_logger(&args);

    // Read config.json
    let config_path = Path::new("config.json");
    let config_file = File::open(config_path).expect("Unable to open config file");
    let config: Config = serde_json::from_reader(config_file).expect("Unable to parse config to struct");
    info!("Finished parsing config.json");
    dbg!(&config);
    
    //TODO: Set ports
    
    // Start different tasks
    // Note: Task reference not referencable anymore
    match config.store {
        Amount::Single(task) => task.start(),
        Amount::Multiple(tasks) => {
            for t in tasks {
                t.start();
            }
        }
    }
}

pub fn initialise_logger(args: &Vec<String>) {
    let mut log_level: LevelFilter = LevelFilter::Error;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "-l" {
            if let Some(level) = iter.next() {
                log_level = match level.as_str() {
                    "info" => LevelFilter::Info,
                    "warn" => LevelFilter::Warn,
                    "error" => LevelFilter::Error,
                    "debug" => LevelFilter::Debug,
                    "trace" => LevelFilter::Trace,
                    "off" => LevelFilter::Off,
                    _ => log_level,
                };
            }
        }
    }
    env_logger::builder()
        .filter_level(log_level)
        .init();
}

