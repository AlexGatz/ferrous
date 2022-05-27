//! Title:          ferrous_waf 
//! Written by:     Alex J. Gatz 
//! Date:           2022/05/21 
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

use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{service_fn, make_service_fn};
use std::{convert::Infallible, net::SocketAddr};
use std::net::IpAddr;

async fn handle(client_ip: IpAddr, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        // will forward requests to port 13901
        match hyper_reverse_proxy::call(client_ip, "http://127.0.0.1:13901", req).await {
            Ok(response) => {Ok(response)}
            Err(_error) => {Ok(Response::builder()
                                  .status(StatusCode::INTERNAL_SERVER_ERROR)
                                  .body(Body::empty())
                                  .unwrap())}
        }
}

#[tokio::main]
async fn main() {
    let bind_addr = "127.0.0.1:8000";
    let addr:SocketAddr = bind_addr.parse().expect("Could not parse ip:port.");

    let make_svc = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr().ip();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| handle(remote_addr, req)))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Running server on {:?}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
