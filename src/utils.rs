use hyper::{Body, Response, StatusCode};
use std::convert::Infallible;

pub fn custom_hyper_response(status: StatusCode, body: Body) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(status)
        .body(body)
        .unwrap())
}