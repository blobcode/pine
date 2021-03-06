use env_logger::Env;
use log::{error, info};
// global modules
mod args;
mod config;
mod server;

const LOGO: &str = r#"                          
   ___  __ __  ___ 
  / _ \/ / _ \/ -_)
 / .__/_/_//_/\__/ 
/_/
"#;

fn main() {
    // logging level setup
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    // parse args
    let args = match args::parse() {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    };

    // load config
    let conf = config::parse(&args.configfile.unwrap());

    // start server
    println!("{}", LOGO);
    println!(
        "server endpoint at {}",
        format!("http://localhost:{}", conf.port),
    );
    for (host, to) in &conf.hosts {
        println!("proxying {} -> {}", host, to);
    }
    println!();
    info!("hit ctrl-c to stop the server");

    server::run(conf);
}
