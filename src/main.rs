use env_logger::Env;
use log::{error, info};
use std::path::Path;

// global modules
mod config;
mod server;

const LOGO: &str = r#"                          
   ___  __ __  ___ 
  / _ \/ / _ \/ -_)
 / .__/_/_//_/\__/ 
/_/
"#;

// help message
const HELP: &str = r#"                          
   ___  __ __  ___ 
  / _ \/ / _ \/ -_)
 / .__/_/_//_/\__/ 
/_/

a simple, elegant reverse proxy

usage: 

pine <config file>

"#;

// main app args
#[derive(Debug)]
struct AppArgs {
    configfile: Option<String>,
}

fn main() {
    // logging level setup
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let conf = config::getconfig(&args.configfile.unwrap());

    // start server
    println!("{}", LOGO);
    println!(
        "server endpoint at {}",
        format!("http://localhost:{}", conf.port),
    );
    for (hosts, to) in &conf.hosts {
        println!("proxying {}", format!("{} -> {}", hosts.join(", "), to));
    }
    info!("hit ctrl-c to stop the server");
    server::run(conf);
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    // init config struct
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled first.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let mut args = AppArgs {
        configfile: pargs.opt_free_from_str()?,
    };

    if Path::new("./config.ini").exists() && args.configfile.is_none() {
        args.configfile = Some("./config.ini".to_string())
    }
    // checking if the config file doesn't exist
    else if !Path::new(&args.configfile.clone().unwrap()).exists() {
        error!("{} not found", args.configfile.unwrap());
        println!("{}", HELP);
        std::process::exit(0);
    }

    Ok(args)
}
