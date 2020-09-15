// use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse};

pub mod google;

mod token;
pub use self::token::*;

mod traits;
pub use self::traits::*;

// pub type BasicToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
