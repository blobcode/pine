extern crate tini;
use std::collections::HashMap;
use tini::Ini;

// main config struct
pub struct Config {
    pub port: u16,
    pub hosts: HashMap<Vec<String>, String>,
}

// loads file
fn readfile(file: &str) -> Ini {
    Ini::from_file(file).unwrap()
}

// parse config file
fn gethosts(file: &str) -> HashMap<Vec<String>, String> {
    let config = readfile(file);
    let hostlist: Vec<String> = config.get_vec("config", "hosts").unwrap();
    let mut hosts = HashMap::new();
    for host in hostlist {
        let input: String = config.get(&host, "from").unwrap();
        let from = input.split(", ").map(|s| s.to_string()).collect();
        let to: String = config.get(&host, "to").unwrap();
        hosts.insert(from, to);
    }
    hosts
}

// main function to get config struct
pub fn getconfig(file: &str) -> Config {
    let conf = readfile(file);
    Config {
        port: conf.get("config", "port").unwrap(),
        hosts: gethosts(file),
    }
}
