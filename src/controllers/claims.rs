use chrono::{Duration};
use jsonwebtoken::{
    decode, DecodingKey, EncodingKey, Validation,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use axum::{
    async_trait,
    TypedHeader,
    extract::{FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
};

use axum::RequestPartsExt;

use crate::controllers::custom_errors::AuthError;
    

use std::{fmt::Display};


const BEARER: &str = "Bearer ";
const AUTHORIZATION: &str = "Authorization";

const SECRET: &str = "secret";

lazy_static! {
    static ref TOKEN_EXPIRATION: Duration = Duration::minutes(5);
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}






struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}


#[async_trait]
impl<S> FromRequestParts<S> for Claims

where
    S: Send + Sync,

{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

        let TypedHeader(Authorization(bearer)) = parts 
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError:: InvalidToken)?;


        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)        
    }
}    