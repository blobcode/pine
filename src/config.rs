extern crate tini;
use std::collections::HashMap;
use tini::Ini;

pub fn readfile() -> Ini {
    let conf = Ini::from_file("./config.ini").unwrap();
    return conf;
}

pub fn gethosts() -> HashMap<String, String> {
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
