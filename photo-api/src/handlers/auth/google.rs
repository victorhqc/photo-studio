use crate::auth::encode_token;
use crate::auth::google::{
  build_client, exchange_token, gen_authorize_url, get_user_profile, GoogleRedirectExtractor,
};
use crate::conduit::users::find_or_create;
use crate::connection::Repo;
use crate::handlers::utils::error_request;
use futures::future;
use futures::prelude::*;
use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::{create_response, create_temporary_redirect};
use gotham::state::{FromState, State};
use hyper::StatusCode;
use photo_core::models::User;
use serde::Serialize;
use std::env;
use std::pin::Pin;

#[derive(Serialize)]
struct AuthenticatedUser {
  user: User,
  token: String,
}

pub fn google_authorize_handler(state: State) -> Pin<Box<HandlerFuture>> {
  let google_client = match build_client() {
    Ok(c) => c,
    Err(e) => return error_request(state, e.into()),
  };
  let (authorize_url, _) = gen_authorize_url(google_client);
  let res = create_temporary_redirect(&state, authorize_url.to_string());
  let f = future::ok((state, res));

  f.boxed()
}

pub fn google_redirect_handler(mut state: State) -> Pin<Box<HandlerFuture>> {
  let query_param = GoogleRedirectExtractor::take_from(&mut state);
  let google_client = match build_client() {
    Ok(c) => c,
    Err(e) => return error_request(state, e.into()),
  };

  let token = match exchange_token(&query_param, &google_client) {
    Ok(t) => t,
    Err(e) => return error_request(state, e.into()),
  };

  let profile = match get_user_profile(&token) {
    Ok(p) => p,
    Err(e) => return error_request(state, e.into()),
  };

  let repo = Repo::borrow_from(&state).clone();
  let results = find_or_create(repo, profile).then(|result| match result {
    Ok(user) => {
      let token = encode_token(&user, 3600);
      let redirect_url = env::var("REDIRECT_CLIENT_URL");

      let res = match redirect_url {
        Ok(u) => create_temporary_redirect(&state, format!("{}?token={}", u, token)),
        _ => {
          let response = AuthenticatedUser { user, token };
          let body = serde_json::to_string(&response).expect("Failed to serialize user.");
          create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body)
        }
      };

      future::ok((state, res))
    }
    Err(e) => future::err((state, e.into())),
  });

  results.boxed()
}
