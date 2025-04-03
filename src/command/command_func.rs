use std::f32::consts::E;
use std::io::Error;
use std::process::{Child, Command, Output};
use log::debug;

pub fn spawn_command(arg: &String) -> Child {
    debug!("{}", format!("Running command \"{}\"", arg));
    let spawn: Child;
    if cfg!(target_os = "windows") {
        spawn = Command::new("cmd")
            .arg("/C")
            .arg(arg)
            .spawn()
            .expect("Failed to spawn command")
    } else {
        spawn = Command::new("sh")
            .arg("-c")
            .arg(arg)
            .spawn()
            .expect("Failed to spawn command")
    }

    spawn
}

pub fn output_command(arg: &str) -> String {
    debug!("{}", format!("Running command \"{}\"", arg));
    let output;
    if cfg!(target_os = "windows") {
        output = Command::new("cmd")
            .arg("/C")
            .arg(arg)
            .output()
            .expect("Unable to output command")
    } else {
        output = Command::new("sh")
            .arg("-c")
            .arg(arg)
            .output()
            .expect("Unable to output command")
    }
    String::from_utf8(output.stdout).unwrap()
}

pub fn spawn_commands(args: &[String]) -> Child {
    //debug!("{}", format!("Running commands \"{}\"", args));
    let spawn: Child;
    if cfg!(target_os = "windows") {
        spawn = Command::new("cmd")
            .arg("/C")
            .args(args)
            .spawn()
            .expect("Failed to spawn command")
    } else {
        spawn = Command::new("sh")
            .arg("-c")
            .args(args)
            .spawn()
            .expect("Failed to spawn command")
    }

    spawn
}