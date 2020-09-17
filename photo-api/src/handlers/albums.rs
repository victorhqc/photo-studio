use super::utils::{extract_json, ExtractJsonError};
use crate::auth::AuthUser;
use crate::conduit::{albums, users};
use crate::connection::Repo;
use gotham::handler::HandlerResult;
use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};
use gotham_middleware_jwt::AuthorizationToken;
use hyper::StatusCode;
use photo_core::models::Album;
use serde::{Deserialize, Serialize};
use snafu::{Backtrace, ResultExt, Snafu};

#[derive(Deserialize)]
pub struct NewAlbumRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct NewAlbumResponse {
    album: Album,
}

pub async fn new_album(mut state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();
    let req_data: NewAlbumRequest = match extract_json(&mut state).await.context(ExtractJson) {
        Ok(data) => data,
        Err(e) => return Err((state, e.into())),
    };

    let token = AuthorizationToken::<AuthUser>::borrow_from(&state);
    let email = token.0.claims.email();

    let user = match users::find_by_email(repo.clone(), email)
        .await
        .context(UserIssue)
    {
        Ok(u) => u,
        Err(e) => return Err((state, e.into())),
    };

    let description = req_data.description.clone();

    let response = match albums::create(repo, &user, req_data.name, description).await {
        Ok(album) => {
            let response = NewAlbumResponse { album };
            let body = serde_json::to_string(&response).expect("Fail to serialize album");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);

            res
        }
        Err(e) => return Err((state, e.into())),
    };

    Ok((state, response))
}

#[derive(Debug, Snafu)]
pub enum AlbumHandlersError {
    #[snafu(display("Could not get request: {}", cause))]
    ExtractJson {
        #[snafu(source)]
        cause: ExtractJsonError,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not get user: {}", cause))]
    UserIssue {
        #[snafu(source)]
        cause: users::UserError,
        backtrace: Backtrace,
    },
}
