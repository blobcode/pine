use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Error, Server};
use std::net::SocketAddr;

// imports
mod config;
mod logging;

pub use crate::config::getconfig;
pub use crate::logging::{debug, error, info};

// main event loop
#[tokio::main]
async fn main() {
    // load config
    let config = getconfig();
    // set server address
    let port = config.port;
    let hosts = config.hosts;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let client_main = Client::new();

    // The closure inside `make_service_fn` is run for each connection,
    // creating a 'service' to handle requests for that specific connection, and
    // will run on EVERY request.
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
                            info(format!(
                                "request to {}{} sent to {}",
                                fromhost,
                                req.uri(),
                                to,
                            ))
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
    info(format!("server listening on http://localhost:{}", port));
    debug("running in debug");
    // error handling
    if let Err(err) = server.await {
        error(&format!("{}", err));
    }
}
