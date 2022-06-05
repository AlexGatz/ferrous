//! Title:          ferrous_waf
//! Written by:     Alex J. Gatz
//! Date:           2022/05/29
//!
//! Description:
//! This is a bare bones implementatoin of a waf that only works over port 80, for now (rustls?).
//! The intention of this project is to help me gain a better understanding of various methods
//! of parsing http traffic and comparing that traffic's uniqe components with "rules".
//!
//! Test cases:
//! 1.) Blocking rule: A single request should be able to be blocked.
//! 2.) Risk rule: A single request/response should be able to increment a "score".
//! 3.) More to be determined...

// TODO: Config file import stuff.
// TODO: Set this up as a clap app.
// TODO: Rule engine.
// TODO: Create some tests.
// Low Prio: Graceful config updates, see SOZU's "hot configurable" feature.

// Random utilty functions to keep main clean, maybe?
mod config;
mod utils;
use config::Config;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::net::IpAddr;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};

async fn handle(
    client_ip: IpAddr,
    config: String,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    if match_req(&req).await {
        utils::custom_hyper_response(StatusCode::FORBIDDEN, Body::from("Game Over."))
    } else {
        let res = match hyper_reverse_proxy::call(client_ip, &config, req).await {
            Ok(response) => Ok(response),
            Err(_error) => utils::custom_hyper_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::from("Welp. The server is mad."),
            ),
        };
        // TODO: Create match_res fn and do stuff with it.
        // If response has stuff an attacker shouldn't get, respond with block page.
        // ex. The res contains the structure of a etc/shadow file.
        res
    }
}

// TODO: Move matching stuff into its own 'rule engine'
// Does this really need to be async?
async fn match_req(req: &Request<Body>) -> bool {
    // TODO: Parse the req using something like serde maybe?
    // TODO: Process the req with matching rules.
    // Returning true will cause a block, false will allow the request to be made.
   
    // Ok so this worked but is there a better way?
    let new_req = Request::new(req);
    let (parts, body) = new_req.into_parts();
    dbg!(parts.uri);
    dbg!(parts.version);
    dbg!(parts.headers);
    dbg!(parts.method);
    dbg!(parts.extensions);
    // dbg!(body);
    false
}

#[tokio::main]
async fn main() {
    // Arn needed here to deal with the move within the closures below.
    let config = Arc::new(Config::default());
    let server_addr: SocketAddr = config.server.parse().expect("Could not parse ip:port.");

    // Uses closures, `move` and `async move` to handle all incoming connections asychronously.
    // Clone used twice due to the scope of the nested closures.
    // IMO nested closures is hardly readable...
    // This is much more expensive than it should be. We are passing the config object each time
    // and we really only need to pass the single slice.
    let make_svc = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr().ip();
        let upstream_addr = config.upstream.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle(remote_addr, upstream_addr.clone(), req)
            }))
        }
    });

    let server = Server::bind(&server_addr).serve(make_svc);

    println!("Running server on {:?}", server_addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
