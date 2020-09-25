use super::Profile;
use crate::utils::get_url;
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    reqwest::{async_http_client, Error as Oauth2ReqwestError},
    AsyncCodeTokenRequest, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    EmptyExtraTokenFields, RedirectUrl, RequestTokenError, Scope, StandardErrorResponse,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use photo_core::models::User;
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use snafu::{Backtrace, ResultExt};
use std::env;
use url::ParseError;

const GOOGLE_PEOPLE_ENDPOINT: &str = "https://www.googleapis.com";

pub fn build_client() -> Result<BasicClient> {
    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID environment variable."),
    );

    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing GOOGLE_CLIENT_SECRET environment variable."),
    );

    let auth_url = AuthUrl::new(String::from("https://accounts.google.com/o/oauth2/v2/auth"))
        .context(UrlParseError)?;

    let token_url = TokenUrl::new(String::from("https://www.googleapis.com/oauth2/v3/token"))
        .context(UrlParseError)?;

    let redirect_url =
        RedirectUrl::new(format!("{}/google/redirect", get_url())).context(UrlParseError)?;

    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_url(redirect_url);

    Ok(client)
}

pub fn gen_authorize_url(client: BasicClient) -> (url::Url, CsrfToken) {
    client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .url()
}

pub async fn exchange_token(
    extractor: &GoogleRedirectExtractor,
    client: &BasicClient,
) -> Result<BasicToken> {
    let code = AuthorizationCode::new(extractor.code.to_owned());
    let token = client
        .exchange_code(code)
        .request_async(async_http_client)
        .await
        .context(OAuth2Request)?;

    Ok(token)
}

pub async fn get_user_profile(token: &BasicToken) -> Result<GoogleProfile> {
    let token_header = format!("Bearer {}", token.access_token().secret());

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token_header).unwrap());

    let url = format!("{}/oauth2/v1/userinfo?alt=json", GOOGLE_PEOPLE_ENDPOINT);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .context(ReqwestBuild)?;

    let response = client.get(&url).send().await.context(ReqwestIssue)?;
    let profile: GoogleProfile = response.json().await.context(JsonParse)?;

    Ok(profile)
}

#[derive(Deserialize, Serialize, StateData, StaticResponseExtender)]
pub struct GoogleRedirectExtractor {
    state: String,
    code: String,
    scope: Vec<String>,
    prompt: String,
    authuser: i32,
}

type BasicToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleProfile {
    id: String,
    email: String,
    family_name: Option<String>,
    gender: Option<String>,
    given_name: Option<String>,
    locale: Option<String>,
    picture: Option<String>,
    verified_email: bool,
}

impl Profile for GoogleProfile {
    fn new_user(&self) -> User {
        let email = self.email.clone();
        let picture = self.picture.clone();

        User::new(email, picture)
    }
}

pub type Result<T, E = GoogleError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum GoogleError {
    #[snafu(display("Could not parse URL {}", source))]
    UrlParseError {
        source: ParseError,
        backtrace: Backtrace,
    },
    #[snafu(display("Could not complete oauth2 request {}", source))]
    OAuth2Request {
        #[snafu(source(from(TokenError, failure::Fail::compat)))]
        source: failure::Compat<TokenError>,
    },

    #[snafu(display("Could not complete request {}", source))]
    ReqwestIssue {
        source: ReqwestError,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not build client {}", source))]
    ReqwestBuild {
        source: ReqwestError,
        backtrace: Backtrace,
    },

    #[snafu(display("Could not parse JSON {}", source))]
    JsonParse { source: ReqwestError },
}

type TokenError = RequestTokenError<
    Oauth2ReqwestError<ReqwestError>,
    StandardErrorResponse<BasicErrorResponseType>,
>;
