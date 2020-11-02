use super::utils::{extract_json, HandlerUtilsError};
use crate::auth::AuthUser;
use crate::conduit::{book_me, users};
use crate::connection::Repo;
use gotham::handler::HandlerResult;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::state::{FromState, State};
use gotham_middleware_jwt::AuthorizationToken;
use hyper::StatusCode;
use photo_core::models::BookMe;
use reqwest;
use serde::{Deserialize, Serialize};
use snafu::{Backtrace, ResultExt};
use std::env;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookMeRequest {
    pub email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookMeResponse {
    info: BookMe,
}

pub async fn update(mut state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();

    let req_data: UpdateBookMeRequest =
        match extract_json(&mut state).await.context(HandlerUtilsIssue) {
            Ok(data) => data,
            Err(e) => {
                debug!("{:?}", e);
                return Err((state, e.into()));
            }
        };

    let token = AuthorizationToken::<AuthUser>::borrow_from(&state);
    let email = token.0.claims.email();

    let user = match users::find_by_email(repo.clone(), email)
        .await
        .context(UserIssue)
    {
        Ok(u) => u,
        Err(e) => {
            debug!("{:?}", e);
            return Err((state, e.into()));
        }
    };

    let response = match book_me::update_or_create(repo, &user, req_data.email)
        .await
        .context(BookMeIssue)
    {
        Ok(info) => {
            let response = UpdateBookMeResponse { info };
            let body = serde_json::to_string(&response).expect("Failed to serialize booke me info");

            create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body)
        }
        Err(e) => return Err((state, e.into())),
    };

    Ok((state, response))
}

pub async fn find_by_user(state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();

    let token = AuthorizationToken::<AuthUser>::borrow_from(&state);
    let email = token.0.claims.email();

    let user = match users::find_by_email(repo.clone(), email)
        .await
        .context(UserIssue)
    {
        Ok(u) => u,
        Err(e) => {
            debug!("{:?}", e);
            return Err((state, e.into()));
        }
    };

    let response = match book_me::find_by_user(repo, &user)
        .await
        .context(BookMeIssue)
    {
        Ok(info) => {
            let response = UpdateBookMeResponse { info };
            let body = serde_json::to_string(&response).expect("Failed to serialize booke me info");

            create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body)
        }
        Err(AlbumHandlersError::BookMeIssue { .. }) => {
            create_empty_response(&state, StatusCode::NOT_FOUND)
        }
        Err(e) => return Err((state, e.into())),
    };

    Ok((state, response))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookMeRequest {
    pub name: String,
    pub email: String,
    pub message: String,
    pub phone: Option<String>,
    pub date: Option<String>,
    pub venue: Option<String>,
    pub city: Option<String>,
}

#[derive(Deserialize, Serialize, StateData, StaticResponseExtender)]
pub struct WithIdExtractor {
    id: String,
}

pub async fn book_me(mut state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();
    let query_param = WithIdExtractor::take_from(&mut state);

    let req_data: BookMeRequest = match extract_json(&mut state).await.context(HandlerUtilsIssue) {
        Ok(data) => data,
        Err(AlbumHandlersError::HandlerUtilsIssue {
            cause: HandlerUtilsError::JsonParse { .. },
        }) => {
            let body = create_empty_response(&state, StatusCode::BAD_REQUEST);

            return Ok((state, body));
        }
        Err(e) => {
            debug!("{:?}", e);
            return Err((state, e.into()));
        }
    };

    let domain = match env::var("MAILGUN_DOMAIN") {
        Ok(v) => v,
        Err(e) => {
            debug!("{:?}", e);
            return Err((state, e.into()));
        }
    };

    let api_key = match env::var("MAILGUN_API_KEY") {
        Ok(v) => v,
        Err(e) => {
            debug!("{:?}", e);
            return Err((state, e.into()));
        }
    };

    let user = match users::find_by_id(repo.clone(), query_param.id).await {
        Ok(u) => u,
        Err(e) => {
            debug!("{:?}", e);
            return Err((state, e.into()));
        }
    };

    let info = match book_me::find_by_user(repo, &user).await {
        Ok(b) => b,
        Err(e) => {
            debug!("{:?}", e);
            return Err((state, e.into()));
        }
    };

    let uri = format!("https://api.mailgun.net/v3/{}/messages", domain);

    let phone = match req_data.phone {
        Some(p) => p,
        None => String::from("Not specified."),
    };

    let date = match req_data.date {
        Some(p) => p,
        None => String::from("Not specified."),
    };

    let venue = match req_data.venue {
        Some(p) => p,
        None => String::from("Not specified."),
    };

    let city = match req_data.city {
        Some(p) => p,
        None => String::from("Not specified."),
    };

    let params = [
        ("from", &format!("Website <website@{}>", domain)),
        ("to", &info.email),
        ("subject", &String::from("Contact from Website")),
        (
            "html",
            &String::from(format!(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                  <head>
                    <meta charset="utf-8" />
                    <title>Contact from Website</title>
                  </head>
                  <body>
                    <p>
                        <strong>Name:</strong>
                        <span>{}</span>
                    </p>
                    <p>
                        <strong>Email:</strong>
                        <span>{}</span>
                    </p>
                    <p>
                        <strong>Message:</strong>
                        <span>{}</span>
                    </p>
                    <hr />
                    <p>
                        <strong>Phone:</strong>
                        <span>{}</span>
                    </p>
                    <p>
                        <strong>Date:</strong>
                        <span>{}</span>
                    </p>
                    <p>
                        <strong>Venue:</strong>
                        <span>{}</span>
                    </p>
                    <p>
                        <strong>City:</strong>
                        <span>{}</span>
                    </p>
                  </body>
                </html>
        "#,
                req_data.name, req_data.email, req_data.message, phone, date, venue, city,
            )),
        ),
    ];

    let client = reqwest::Client::new();

    let res = match client
        .post(&uri)
        .basic_auth("api", Some(api_key))
        .form(&params)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            debug!("{:?}", e);
            return Err((state, e.into()));
        }
    };

    debug!("Response: {:?}", res);
    match res.status() {
        StatusCode::OK => {}
        code => {
            let body = create_empty_response(&state, code);
            return Ok((state, body));
        }
    }

    let body = create_empty_response(&state, StatusCode::OK);

    Ok((state, body))
}

#[derive(Debug, Snafu)]
pub enum AlbumHandlersError {
    #[snafu(display("Could not get request: {}", cause))]
    HandlerUtilsIssue {
        #[snafu(source)]
        cause: HandlerUtilsError,
        // backtrace: Backtrace,
    },

    #[snafu(display("Could not get book me info: {}", cause))]
    BookMeIssue {
        #[snafu(source)]
        cause: book_me::BookMeError,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not get user: {}", cause))]
    UserIssue {
        #[snafu(source)]
        cause: users::UserError,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not complete request: {}", source))]
    ReqwestError {
        source: reqwest::Error,
        backtrace: Backtrace,
    },
}
