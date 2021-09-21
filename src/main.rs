use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Request, Response, Server};
use serde_derive::Deserialize;
use std::convert::Infallible;
use std::fs;
use std::net::SocketAddr;

// import
mod logging;

pub use crate::logging::{info, debug};

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

async fn handler(_: Request<Body>) -> Result<Request<Body>, Infallible> {
    Ok(Request::new(Body::from("Hello World!")))
}

// main loop
#[tokio::main]
async fn main() {
    // load config
    let confile = fs::read_to_string("./config.toml").expect("Unable to read config file");
    let decoded: Config = toml::from_str(&confile).unwrap();
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
                        info(format!("request to {}{} sent to {}", host.from, req.uri(), &host.to))
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
    info(format!("server listening on http://{}", addr));
    debug("running in debug");
    // error handling
    if let Err(err) = server.await {
        panic!("{}", err);
    }
}
