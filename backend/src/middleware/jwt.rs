use std::env;
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm, get_current_timestamp};
use serde::{Serialize, Deserialize};
use dotenvy::dotenv;

use super::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

pub fn create_token(sub: &str) -> Result<String> {
    dotenv().ok();

    // let exp = env::var("TOKEN_EXP").map_or_else(|_| 300, |x| x.parse().expect("TOKEN_EXP must be an integer"));
    let exp = 43200;

    let iat = get_current_timestamp();
    let exp = get_current_timestamp() + exp;

    let claim = Claims {
        sub: sub.to_string(),
        iat,
        exp,
    };

    let secret = env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set");
    let secret = EncodingKey::from_secret(secret.as_bytes());
    
    encode(&Header::default(), &claim, &secret).map_err(|e| e.into())
}

pub fn verify_token(token: &str) -> Result<bool> {
    dotenv().ok();

    let secret = env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set");
    let secret = DecodingKey::from_secret(secret.as_bytes());

    let leeway = env::var("TOKEN_LEEWAY").map_or_else(|_| 60, |x| x.parse().expect("TOKEN_LEEWAY must be an integer"));
    
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = leeway;
    decode::<Claims>(&token, &secret, &validation)?;

    Ok(true)
}
