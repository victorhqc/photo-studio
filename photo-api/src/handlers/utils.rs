use bytes::Bytes;
use futures::future;
use futures::prelude::*;
use gotham::anyhow::Error;
use gotham::handler::{HandlerError, HandlerFuture};
use gotham::helpers::http::response::create_empty_response;
use gotham::hyper::{body, Body, Error as HyperError, Response, StatusCode};
use gotham::state::{FromState, State};
use snafu::{Backtrace, ResultExt, Snafu};
use std::pin::Pin;

pub async fn get_body_bytes(state: &mut State) -> Result<Bytes> {
    let body_from_state = Body::take_from(state);
    let body_bytes = body::to_bytes(body_from_state).await.context(BodyParse)?;

    Ok(body_bytes)
}

pub async fn extract_json<T>(state: &mut State) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let body_bytes = get_body_bytes(state).await?;
    let json = serde_json::from_slice::<T>(&body_bytes[..]).context(JsonParse)?;

    Ok(json)
}

pub fn empty_handler(state: State) -> (State, Response<Body>) {
    let res = create_empty_response(&state, StatusCode::NO_CONTENT);

    (state, res)
}

pub fn error_request(state: State, e: Error) -> Pin<Box<HandlerFuture>> {
    let err = HandlerError::from(e);
    let f = future::err((state, err.into()));
    return f.boxed();
}

pub type Result<T> = std::result::Result<T, ExtractJsonError>;

#[derive(Debug, Snafu)]
pub enum ExtractJsonError {
    #[snafu(display("Something went wrong parsing body: {}", source))]
    BodyParse {
        source: HyperError,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not parse JSON: {}", source))]
    JsonParse {
        source: serde_json::error::Error,
        backtrace: Backtrace,
    },
}
