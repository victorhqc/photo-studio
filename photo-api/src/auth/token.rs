use jsonwebtoken::{encode, EncodingKey, Header};
use photo_core::models::User;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
// use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthUser {
  email: String,
  user_id: String,
  exp: u64,
}

impl AuthUser {
  pub fn new(user: &User, expire_in: u64) -> Self {
    AuthUser {
      email: user.email.clone(),
      user_id: user.id.to_string(),
      exp: seconds_from_now(expire_in),
    }
  }

  // pub fn email(&self) -> String {
  //   self.email.clone()
  // }

  // pub fn user_id(&self) -> Uuid {
  //   Uuid::parse_str(&self.user_id).unwrap()
  // }
}

pub fn get_secret() -> String {
  env::var("TOKEN_SECRET").expect("TOKEN_SECRET variable is not defined")
}

pub fn encode_token(user: &User, expire_in: u64) -> String {
  let secret = get_secret();
  let key = EncodingKey::from_secret(secret.as_ref());

  encode(&Header::default(), &AuthUser::new(user, expire_in), &key).unwrap()
}

fn seconds_from_now(secs: u64) -> u64 {
  let expiry_time =
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + Duration::from_secs(secs);
  expiry_time.as_secs()
}
