use log::error;
use std::path::Path;

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

#[derive(Debug)]
pub struct AppArgs {
    pub configfile: Option<String>,
}

pub fn parse() -> Result<AppArgs, pico_args::Error> {
    // init config struct
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled first.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    // try to load args
    let mut args = AppArgs {
        configfile: pargs.opt_free_from_str()?,
    };

    // file checking
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
