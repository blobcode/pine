use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Error, Server};
use serde_derive::Deserialize;
use std::fs;
use std::net::SocketAddr;

// logging crates
use fern::colors::{Color, ColoredLevelConfig};
use log::{error, info};

// config structs
#[derive(Debug, Deserialize)]
struct Config {
    port: Option<u16>,
    #[serde(alias = "host")]
    hosts: Vec<HostConfig>,
}

// struct for hosts in the config file
#[derive(Debug, Deserialize)]
struct HostConfig {
    from: String,
    to: String,
}

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta)
        .info(Color::BrightBlue)
        .error(Color::BrightRed);

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                // This will color the log level only, not the whole line. Just a touch.
                colors.color(record.level()),
                message
            ))
        })
        .apply()
        .unwrap();
    Ok(())
}

// main loop
#[tokio::main]
async fn main() {
    // load config
    let confile = fs::read_to_string("./config.toml").expect("Unable to read config file");
    let decoded: Config = toml::from_str(&confile).unwrap();

    // Configure logger at runtime
    setup_logger().map_err(|err| error!("{}", err)).ok();

    // set server address
    let port = decoded.port.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let client_main = Client::new();

    // The closure inside `make_service_fn` is run for each connection,
    // creating a 'service' to handle requests for that specific connection.
    let make_service = make_service_fn(move |_| {
        // clone vars
        let decoded: Config = toml::from_str(&confile).unwrap();
        let client = client_main.clone();

        async move {
            // Request handler
            Ok::<_, Error>(service_fn(move |mut req| {
                let mut toaddr = "";
                let headers = req.headers();
                // check for host matches in the config file
                for host in &decoded.hosts {
                    if host.from == headers["host"] {
                        toaddr = &host.to;
                        info!("request to {}{} sent to {}", host.from, req.uri(), &host.to)
                    }
                }
                // format new uri
                let uri_string = format!(
                    "http://{}{}",
                    toaddr,
                    req.uri()
                        .path_and_query()
                        .map(|x| x.as_str())
                        .unwrap_or("/")
                );
                // request new url
                let uri = uri_string.parse().unwrap();
                *req.uri_mut() = uri;
                client.request(req)
            }))
        }
    });
    // start server
    let server = Server::bind(&addr).serve(make_service);
    info!("server listening on http://{}", addr);

    // error handling
    if let Err(err) = server.await {
        error!("{}", err);
    }
}
