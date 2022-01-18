use hashbrown::HashMap;
use tini::Ini;

// main config struct
pub struct Config {
    pub port: u16,
    pub hosts: HashMap<String, String>,
}

// loads file
fn readfile(file: &str) -> Ini {
    Ini::from_file(file).unwrap()
}

// parse config file
fn gethosts(file: &str) -> HashMap<String, String> {
    // load config file
    let config = readfile(file);
    // parse list
    let hostlist: Vec<String> = config.get_vec("config", "hosts").unwrap();
    let mut hosts = HashMap::new();

    // add all "to" and "from" fields to the hashmap
    for host in hostlist {
        let input: String = config.get(&host, "from").unwrap();
        for from in input.split(", ") {
            let to: String = config.get(&host, "to").unwrap();
            hosts.insert(from.to_string(), to);
        }
    }
    println!("{:#?}", hosts);
    hosts
}

// main function to get config struct
pub fn parse(file: &str) -> Config {
    // load file
    let conf = readfile(file);
    // create config struct
    Config {
        port: conf.get("config", "port").unwrap(),
        hosts: gethosts(file),
    }
}
