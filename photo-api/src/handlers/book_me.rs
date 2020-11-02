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
use serde::{Deserialize, Serialize};
use snafu::{Backtrace, ResultExt};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookMeRequest {
    pub email: String,
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookMeResponse {
    info: BookMe,
}

#[derive(Debug, Snafu)]
pub enum AlbumHandlersError {
    #[snafu(display("Could not get request: {}", cause))]
    HandlerUtilsIssue {
        #[snafu(source)]
        cause: HandlerUtilsError,
        backtrace: Backtrace,
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
}
