// imports
mod config;
mod logging;
mod server;

use std::path::Path;

use crate::logging::{debug, error, info};

const HELP: &str = r#"                          
   ___  __ __  ___ 
  / _ \/ / _ \/ -_)
 / .__/_/_//_/\__/ 
/_/

a simple, elegant reverse proxy

usage: 

pine <config file>

"#;

#[derive(Debug)]
struct AppArgs {
    configfile: Option<String>,
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let conf = config::getconfig(&args.configfile.unwrap());

    info(format!(
        "server listening on http://localhost:{}",
        conf.port
    ));
    debug("running in debug mode");
    server::run(conf);
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let mut args = AppArgs {
        configfile: pargs.opt_free_from_str()?,
    };

    // overwrite args if local config.ini file is found in the cwd
    if Path::new("./config.ini").exists() && args.configfile.is_none() {
        args.configfile = Some("./config.ini".to_string())
    }
    // checking if the config file doesn't exist
    else if !Path::new(&args.configfile.clone().unwrap()).exists() {
        error(&format!("{} not found", args.configfile.unwrap()));
        print!("{}", HELP);
        std::process::exit(0);
    }

    Ok(args)
}
