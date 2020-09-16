use futures::future;
use futures::prelude::*;
use gotham::anyhow::Error;
use gotham::handler::{HandlerError, HandlerFuture};
use gotham::helpers::http::response::create_empty_response;
use gotham::hyper::{Body, Response, StatusCode};
use gotham::state::State;
use std::pin::Pin;

pub fn empty_handler(state: State) -> (State, Response<Body>) {
    let res = create_empty_response(&state, StatusCode::NO_CONTENT);

    (state, res)
}

pub fn error_request(state: State, e: Error) -> Pin<Box<HandlerFuture>> {
    let err = HandlerError::from(e);
    let f = future::err((state, err.into()));
    return f.boxed();
}
