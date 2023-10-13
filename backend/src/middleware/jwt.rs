use std::env;


use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm, get_current_timestamp};
use serde::{Serialize, Deserialize};
use dotenvy::dotenv;

use crate::middleware::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

pub fn create_token(sub: String) -> Result<String> {
    dotenv().ok();

    let iat = get_current_timestamp();
    let exp = get_current_timestamp() + 300;

    let claim = Claims {
        sub,
        iat,
        exp,
    };

    let secret = env::var("TOKEN_SECRET").expect("JWT_SECRET must be set");
    let secret = EncodingKey::from_secret(secret.as_bytes());
    
    encode(&Header::default(), &claim, &secret).map_err(|e| e.into())
}

pub fn verify_token(token: String) -> Result<bool> {
    let secret = env::var("TOKEN_SECRET").expect("JWT_SECRET must be set");
    let secret = DecodingKey::from_secret(secret.as_bytes());

    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 60;
    decode::<Claims>(&token, &secret, &validation)?;

    Ok(true)
}
