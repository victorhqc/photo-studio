use super::utils::{extract_json, handle_multipart, ExtractJsonError, MultiPartError};
use crate::auth::AuthUser;
use crate::conduit::{albums, photos, users};
use crate::connection::Repo;
use gotham::handler::HandlerResult;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::hyper::{Client, StatusCode};
use gotham::state::{FromState, State};
use gotham_middleware_jwt::AuthorizationToken;
use hyper_tls::HttpsConnector;
use photo_core::models::Photo;
use rusoto_core::{ByteStream, HttpClient, Region};
use rusoto_credential::EnvironmentProvider;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use serde::{Deserialize, Serialize};
use snafu::{Backtrace, OptionExt, ResultExt, Snafu};
use std::env;

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct AlbumPathExtractor {
    id: String,
}

#[derive(Deserialize)]
pub struct NewPhotoRequest {
    pub index_in_album: i32,
    pub name: String,
    pub src: String,
    pub main_color: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct PhotoResponse {
    photo: Photo,
}

pub async fn new_photo(mut state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();
    let req_data: NewPhotoRequest = match extract_json(&mut state).await.context(ExtractJson) {
        Ok(data) => data,
        Err(e) => return Err((state, e.into())),
    };
    let album_data = AlbumPathExtractor::borrow_from(&state);
    let token = AuthorizationToken::<AuthUser>::borrow_from(&state);
    let email = token.0.claims.email();

    let user = match users::find_by_email(repo.clone(), email)
        .await
        .context(UserIssue)
    {
        Ok(u) => u,
        Err(e) => return Err((state, e.into())),
    };
    let album = match albums::find_by_id(repo.clone(), album_data.id.clone())
        .await
        .context(AlbumIssue)
    {
        Ok(a) => a,
        Err(e) => return Err((state, e.into())),
    };

    let response = match photos::create(
        repo,
        &album,
        &user,
        req_data.index_in_album,
        req_data.src,
        req_data.main_color,
        req_data.description,
    )
    .await
    {
        Ok(photo) => {
            let response = PhotoResponse { photo };
            let body = serde_json::to_string(&response).expect("Fail to serialize album");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);

            res
        }
        Err(e) => return Err((state, e.into())),
    };

    Ok((state, response))
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct PhotoPathExtractor {
    id: String,
}

#[derive(Deserialize)]
pub struct UpdatePhotoRequest {
    pub index_in_album: i32,
    pub description: Option<String>,
}

pub async fn update_photo(mut state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();
    let req_data: UpdatePhotoRequest = match extract_json(&mut state).await.context(ExtractJson) {
        Ok(data) => data,
        Err(e) => return Err((state, e.into())),
    };
    let path_data = PhotoPathExtractor::borrow_from(&state);

    let photo = match photos::find_by_id(repo.clone(), path_data.id.clone())
        .await
        .context(PhotoIssue)
    {
        Ok(p) => p,
        Err(e) => return Err((state, e.into())),
    };

    let response = match photos::update(repo, &photo, req_data.index_in_album, req_data.description)
        .await
        .context(PhotoIssue)
    {
        Ok(photo) => {
            let response = PhotoResponse { photo };
            let body = serde_json::to_string(&response).expect("Fail to serialize album");
            let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);

            res
        }
        Err(e) => return Err((state, e.into())),
    };

    Ok((state, response))
}

pub async fn delete_photo(state: State) -> HandlerResult {
    let repo = Repo::borrow_from(&state).clone();
    let path_data = PhotoPathExtractor::borrow_from(&state);

    let photo = match photos::find_by_id(repo.clone(), path_data.id.clone())
        .await
        .context(PhotoIssue)
    {
        Ok(p) => p,
        Err(e) => return Err((state, e.into())),
    };

    let response = match photos::delete(repo, &photo).await.context(PhotoIssue) {
        Ok(_) => {
            let res = create_empty_response(&state, StatusCode::OK);

            res
        }
        Err(e) => return Err((state, e.into())),
    };

    Ok((state, response))
}

pub async fn upload_photo(state: State) -> HandlerResult {
    let (state, data) = match handle_multipart(state).await {
        Ok(d) => d,
        Err((state, e)) => return Err((state, e.into())),
    };
    let multipart = match data.context(NoMultipartData) {
        Ok(d) => d,
        Err(e) => return Err((state, e.into())),
    };

    let byte_stream = ByteStream::from(multipart.data);

    let hyper_builder = Client::builder();
    let https_connector = HttpsConnector::new();
    let http_client = HttpClient::from_builder(hyper_builder, https_connector);

    let credentials_provider = EnvironmentProvider::default();
    let s3 = S3Client::new_with(http_client, credentials_provider, Region::default());

    let bucket = match env::var("AWS_S3_BUCKET_NAME").context(NoBucket) {
        Ok(b) => b,
        Err(e) => return Err((state, e.into())),
    };

    let content_type = match multipart.content_type {
        Some(c) => Some(c.to_string()),
        None => None,
    };
    let key = match multipart.filename {
        Some(f) => f,
        None => return Err((state, PhotoHandlersError::MultipartNoFilename.into())),
    };

    let input = PutObjectRequest {
        key,
        body: Some(byte_stream),
        bucket,
        acl: None,
        cache_control: None,
        content_disposition: None,
        content_encoding: None,
        content_language: None,
        content_length: None,
        content_md5: None,
        content_type,
        expires: None,
        grant_full_control: None,
        grant_read: None,
        grant_read_acp: None,
        grant_write_acp: None,
        metadata: None,
        object_lock_legal_hold_status: None,
        object_lock_mode: None,
        object_lock_retain_until_date: None,
        request_payer: None,
        sse_customer_algorithm: None,
        sse_customer_key: None,
        sse_customer_key_md5: None,
        ssekms_encryption_context: None,
        ssekms_key_id: None,
        server_side_encryption: None,
        storage_class: None,
        tagging: None,
        website_redirect_location: None,
    };

    let s3_object = s3.put_object(input).await;
    println!("{:?}", s3_object);

    let response = create_empty_response(&state, StatusCode::OK);

    Ok((state, response))
}

#[derive(Debug, Snafu)]
pub enum PhotoHandlersError {
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

    #[snafu(display("Could not get album: {}", cause))]
    AlbumIssue {
        #[snafu(source)]
        cause: albums::AlbumError,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not get photo: {}", cause))]
    PhotoIssue {
        #[snafu(source)]
        cause: photos::PhotoError,
        backtrace: Backtrace,
    },

    NoMultipartData,

    MultipartNoFilename,

    #[snafu(display("Bucket name is not defined: {}", source))]
    NoBucket {
        source: std::env::VarError,
        backtrace: Backtrace,
    },
}
