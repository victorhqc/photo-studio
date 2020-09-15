use std::env;

pub fn get_url() -> String {
  env::var("PUBLIC_API_URL").expect("Missing PUBLIC_API_URL environment variable.")
}
