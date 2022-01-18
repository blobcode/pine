use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Error, Server};
use log::{error, info};
use std::net::SocketAddr;

use crate::config::Config;

// main event loop
#[tokio::main]
pub async fn run(config: Config) {
    // set server address
    let port = config.port;
    let hosts = config.hosts;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // init client
    let client_main = Client::new();

    // main server function
    let make_service = make_service_fn(move |_| {
        // clone vars
        let hosts = hosts.clone();
        let client = client_main.clone();

        async move {
            // Request handler
            Ok::<_, Error>(service_fn(move |mut req| {
                let mut toaddr = " ";
                let headers = req.headers();
                // check for host matches in the config file
                for (hostgroup, to) in &hosts {
                    for fromhost in hostgroup {
                        if fromhost == &headers["host"] {
                            toaddr = to;
                            info!("request to {}{} -> {}", fromhost, req.uri(), to)
                        }
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
    // error handling
    if let Err(e) = server.await {
        error!("{}", e);
    }
}
