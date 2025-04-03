use std::env;
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use log::{debug, info, LevelFilter};
use rand::Rng;
use serde::{Deserialize};
use std::io::Write;

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
pub struct Node {
    address: Address
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Address {
    ip: IpAddr,
    port: u16,
    internal_port: u8
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
struct Config {
   // process: Amount<Process>,
   // generate: Amount<Generate>,
    store: Amount<Store>
}

fn main() {
    // Read starting arguments
    let args: Vec<String> = env::args().collect();
    initialise_logger(&args);

    // Read config.json
    let config_path = Path::new("config.json");
    let config_file = File::open(config_path).expect("Unable to open config file");
    let mut config: Config = serde_json::from_reader(config_file).expect("Unable to parse config to struct");
    info!("Finished parsing config.json");

    // Get amount of docker containers to assign ports
    let mut docker_amount: usize = 0;

    let addresses: Vec<Address> = gen_unique_address(docker_amount);
    let mut current_address: usize = 0;

    // Start different tasks
    // Note: Task reference not referencable anymore
    match config.store {
        Amount::Single(task) => task.start(),
        Amount::Multiple(tasks) => tasks.start()
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
        .format_timestamp(None)
        .init();
}

pub fn gen_unique_address(amount: usize) -> Vec<Address> {
    let mut addresses: Vec<Address> = Vec::<Address>::new();
    for _ in 0..amount {
        let port = rand::thread_rng().gen_range(100..10000);
        let internal_port = rand::thread_rng().gen_range(10..100);
        addresses.push(Address {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port,
            internal_port
        }); 
    }

    addresses
}

