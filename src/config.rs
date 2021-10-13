extern crate tini;
use std::collections::HashMap;
use tini::Ini;

// main config struct
pub struct Config {
    pub port: u16,
    pub hosts: HashMap<String, String>,
}

fn readfile() -> Ini {
    let conf = Ini::from_file("./config.ini").unwrap();
    return conf;
}

fn gethosts() -> HashMap<String, String> {
    let config = readfile();
    let hostlist: Vec<String> = config.get_vec("config", "hosts").unwrap();
    let mut hosts = HashMap::new();
    for host in hostlist {
        let from: String = config.get(&host, "from").unwrap();
        let to: String = config.get(&host, "to").unwrap();
        hosts.insert(from, to);
    }
    return hosts;
}

// main function
pub fn getconfig() -> Config {
    let conf = readfile();
    let config = Config {
        port: conf.get("config", "port").unwrap(),
        hosts: gethosts(),
    };
    return config;
}
