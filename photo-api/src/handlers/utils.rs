use bytes::Bytes;
use futures::future;
use futures::prelude::*;
use gotham::anyhow::Error;
use gotham::handler::{HandlerError, HandlerFuture};
use gotham::helpers::http::response::create_empty_response;
use gotham::hyper::{
    body, header::CONTENT_TYPE, Body, Error as HyperError, HeaderMap, Response, StatusCode,
};
use gotham::state::{FromState, State};
use multipart::server::Multipart;
use snafu::{Backtrace, ResultExt, Snafu};
use std::io::{Cursor, Read};
use std::pin::Pin;

pub async fn get_body_bytes(state: &mut State) -> JsonResult<Bytes> {
    let body_from_state = Body::take_from(state);
    let body_bytes = body::to_bytes(body_from_state).await.context(BodyParse)?;

    Ok(body_bytes)
}

pub async fn extract_json<T>(state: &mut State) -> JsonResult<T>
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

pub type JsonResult<T> = std::result::Result<T, ExtractJsonError>;

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

pub fn handle_multipart(mut state: State) -> Pin<Box<HandlerMultipart>> {
    const BOUNDARY: &str = "boundary=";
    let header_map = HeaderMap::borrow_from(&state);
    let boundary = header_map
        .get(CONTENT_TYPE)
        .and_then(|ct| {
            let ct = ct.to_str().ok()?;
            let idx = ct.find(BOUNDARY)?;
            Some(ct[idx + BOUNDARY.len()..].to_string())
        })
        .unwrap();

    let f = body::to_bytes(Body::take_from(&mut state)).then(|full_body| match full_body {
        Ok(valid_body) => {
            let mut m = Multipart::with_body(Cursor::new(valid_body), boundary);
            match m.read_entry() {
                Ok(Some(mut field)) => {
                    let mut data: Vec<u8> = Vec::new();
                    field.data.read_to_end(&mut data).expect("can't read");

                    let multipart_data = MultiPartData {
                        data,
                        filename: field.headers.filename.clone(),
                        content_type: field.headers.content_type.clone(),
                    };

                    future::ok((state, Some(multipart_data)))
                }
                Ok(None) => future::ok((state, None)),
                Err(e) => future::err((state, MultiPartError::ReadEntry { source: e })),
            }
        }
        Err(e) => future::err((state, MultiPartError::BodyParseIssue { source: e })),
    });

    f.boxed()
}

pub type HandlerMultipart = dyn Future<Output = MultipartResult> + Send;

pub type MultipartResult =
    std::result::Result<(State, Option<MultiPartData>), (State, MultiPartError)>;

pub struct MultiPartData {
    pub data: Vec<u8>,
    pub filename: Option<String>,
    pub content_type: Option<mime::Mime>,
}

#[derive(Debug, Snafu)]
pub enum MultiPartError {
    #[snafu(display("Could not read multipart: {}", source))]
    ReadEntry { source: std::io::Error },

    #[snafu(display("Could not get body: {}", source))]
    BodyParseIssue { source: HyperError },
}
