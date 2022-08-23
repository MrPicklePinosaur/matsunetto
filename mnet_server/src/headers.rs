use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use thiserror::Error;

use crate::utils::decode_jwt;

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
        let auth_header = req.headers().get_one("Authorization");

        if auth_header.is_none() {
            return Outcome::Failure((Status::Unauthorized, AuthorizationHeaderError::Missing));
        }
        let jwt_string = auth_header.unwrap();

        match decode_jwt(jwt_string) {
            Ok(api_key) => Outcome::Success(AuthorizationHeader(api_key.to_owned())),
            Err(_) => Outcome::Failure((Status::Unauthorized, AuthorizationHeaderError::Invalid)),
        }
    }
}
