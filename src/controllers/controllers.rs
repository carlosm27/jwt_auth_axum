use axum::{http::StatusCode, response::IntoResponse, Json};
//use serde_json::json;
use jsonwebtoken::{
    encode, Header,
};

use serde::{Deserialize, Serialize};




use crate::{
    controllers::custom_errors::AuthError,
    controllers::claims::Claims,
};

#[derive(Serialize)]
struct PublicResponse {
    message: String,
}

#[derive(Serialize)]
struct PrivateResponse {
    message: String,
    user: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}



pub async fn public() -> impl IntoResponse {

    let response = PublicResponse {message: "This endpoint is open to anyone".to_string()};

    (StatusCode::OK, Json(response))
}


pub async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}




pub async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<AuthBody>, AuthError> {

    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if payload.username != "foo" || payload.password != "bar" {
        return Err(AuthError::MissingCredentials);
    }

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 2000,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))    

}