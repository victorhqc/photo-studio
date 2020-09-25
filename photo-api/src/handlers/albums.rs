use super::utils::{extract_json, HandlerUtilsError};
use crate::auth::AuthUser;
use crate::conduit::{albums, users};
use crate::connection::Repo;
use gotham::handler::HandlerResult;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::state::{FromState, State};
use gotham_middleware_jwt::AuthorizationToken;
use hyper::StatusCode;
use photo_core::models::Album;
use serde::{Deserialize, Serialize};
use snafu::{Backtrace, ResultExt};

#[derive(Serialize)]
pub struct AllAlbumsResponse {
    list: Vec<Album>,
}

pub async fn all_albums(state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();
    let token = AuthorizationToken::<AuthUser>::borrow_from(&state);
    let email = token.0.claims.email();

    let user = match users::find_by_email(repo.clone(), email)
        .await
        .context(UserIssue)
    {
        Ok(u) => u,
        Err(e) => return Err((state, e.into())),
    };
    let response = match albums::find_all(repo, &user).await.context(AlbumIssue) {
        Ok(albums) => {
            let response = AllAlbumsResponse { list: albums };
            let body = serde_json::to_string(&response).expect("Failed to serialize albums");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);

            res
        }
        Err(e) => return Err((state, e.into())),
    };

    Ok((state, response))
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct AlbumPathExtractor {
    id: String,
}

#[derive(Deserialize)]
pub struct NewAlbumRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct AlbumResponse {
    album: Album,
}

pub async fn new_album(mut state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();
    let req_data: NewAlbumRequest = match extract_json(&mut state).await.context(HandlerUtilsIssue)
    {
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
            let response = AlbumResponse { album };
            let body = serde_json::to_string(&response).expect("Failed to serialize album");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);

            res
        }
        Err(e) => return Err((state, e.into())),
    };

    Ok((state, response))
}

#[derive(Deserialize)]
pub struct UpdateAlbumRequest {
    pub name: String,
    pub description: Option<String>,
}

pub async fn update_album(mut state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();
    let req_data: UpdateAlbumRequest =
        match extract_json(&mut state).await.context(HandlerUtilsIssue) {
            Ok(data) => data,
            Err(e) => return Err((state, e.into())),
        };
    let path_data = AlbumPathExtractor::borrow_from(&state);

    let album = match albums::find_by_id(repo.clone(), path_data.id.clone())
        .await
        .context(AlbumIssue)
    {
        Ok(a) => a,
        Err(e) => return Err((state, e.into())),
    };

    let response = match albums::update(repo, &album, req_data.name, req_data.description).await {
        Ok(album) => {
            let response = AlbumResponse { album };
            let body = serde_json::to_string(&response).expect("Failed to serialize response");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);

            res
        }
        Err(e) => return Err((state, e.into())),
    };

    Ok((state, response))
}

pub async fn delete_album(state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();
    let path_data = AlbumPathExtractor::borrow_from(&state);

    let album = match albums::find_by_id(repo.clone(), path_data.id.clone())
        .await
        .context(AlbumIssue)
    {
        Ok(a) => a,
        Err(e) => return Err((state, e.into())),
    };

    let response = match albums::delete(repo, &album).await {
        Ok(_) => {
            let res = create_empty_response(&state, StatusCode::OK);

            res
        }
        Err(e) => return Err((state, e.into())),
    };

    Ok((state, response))
}

#[derive(Debug, Snafu)]
pub enum AlbumHandlersError {
    #[snafu(display("Could not get request: {}", cause))]
    HandlerUtilsIssue {
        #[snafu(source)]
        cause: HandlerUtilsError,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not get album: {}", cause))]
    AlbumIssue {
        #[snafu(source)]
        cause: albums::AlbumError,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not get user: {}", cause))]
    UserIssue {
        #[snafu(source)]
        cause: users::UserError,
        backtrace: Backtrace,
    },
}
