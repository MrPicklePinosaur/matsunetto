use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthorizationHeaderError {
    #[error("Missing authorization header")]
    Missing,
    #[error("Invalid authorization header")]
    Invalid,
}

pub struct AuthorizationHeader(pub String);

#[async_trait]
impl<'r> FromRequest<'r> for AuthorizationHeader {
    type Error = AuthorizationHeaderError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key = req.headers().get_one("Authorization");
        match api_key {
            Some(api_key) => Outcome::Success(AuthorizationHeader(api_key.to_owned())),
            None => Outcome::Failure((Status::Unauthorized, AuthorizationHeaderError::Missing)),
        }
    }
}
