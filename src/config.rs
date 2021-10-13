extern crate tini;
use tini::Ini;

pub fn readfile() -> Ini{
    let conf = Ini::from_file("./config.ini").unwrap();
    return conf;
}
