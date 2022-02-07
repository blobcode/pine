use serde::Deserialize;
use std::{collections::HashMap, fs};

// main config struct

pub struct Config {
    pub port: u16,
    pub hosts: HashMap<String, String>,
}

// structs that parse toml
#[derive(Deserialize)]
pub struct ConfigToml {
    pub port: u16,
    pub host: Vec<HostToml>,
}

#[derive(Deserialize)]
pub struct HostToml {
    pub from: Vec<String>,
    pub to: String,
}

// loads file
fn readfile(file: &str) -> ConfigToml {
    // read file
    let buf = fs::read_to_string(file).unwrap();

    // parse file contents
    toml::from_str(&buf).unwrap()
}

// parse config file
fn gethosts(file: &str) -> HashMap<String, String> {
    // load config
    let config = readfile(file);
    // parse list
    let mut hosts = HashMap::new();
    // add all "to" and "from" fields to the hashmap
    for host in config.host {
        for from in host.from {
            let to = &host.to;
            hosts.insert(from.to_string(), to.to_string());
        }
    }
    hosts
}

// main function to get config struct
pub fn parse(file: &str) -> Config {
    // load
    let config = readfile(file);
    // create config struct
    Config {
        port: config.port,
        hosts: gethosts(file),
    }
}
