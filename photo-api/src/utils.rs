use std::env;

pub fn get_url() -> String {
    env::var("PUBLIC_API_URL").expect("Missing PUBLIC_API_URL environment variable.")
}

const MAX_CHAR_VAL: u32 = std::char::MAX as u32;

// Function from https://rosettacode.org/wiki/URL_encoding#Rust
// Equivalent of encodeUriComponent() from `js`
pub fn encode_url_component(key: String) -> String {
    let mut buff = [0; 4];
    key.chars()
        .map(|ch| match ch as u32 {
            0..=47 | 58..=64 | 91..=96 | 123..=MAX_CHAR_VAL => {
                ch.encode_utf8(&mut buff);
                buff[0..ch.len_utf8()]
                    .iter()
                    .map(|&byte| format!("%{:X}", byte))
                    .collect::<String>()
            }
            _ => ch.to_string(),
        })
        .collect::<String>()
}
