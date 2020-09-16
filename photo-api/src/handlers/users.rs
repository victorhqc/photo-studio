use crate::auth::AuthUser;
use crate::conduit::users;
use crate::connection::Repo;
use gotham::handler::HandlerResult;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::state::{FromState, State};
use gotham_middleware_jwt::AuthorizationToken;
use hyper::StatusCode;
use photo_core::models::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    user: User,
}

pub async fn me(state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();
    let token = AuthorizationToken::<AuthUser>::borrow_from(&state);
    let email = token.0.claims.email();

    let user = users::find_by_email(repo, email).await;

    let response = match user {
        Ok(user) => {
            let response = UserResponse { user };
            let body = serde_json::to_string(&response).expect("Failed to serialize user.");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);

            res
        }
        Err(_) => create_empty_response(&state, StatusCode::UNAUTHORIZED),
    };

    Ok((state, response))
}
