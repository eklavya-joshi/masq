use std::env;

use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use dotenvy::dotenv;

use crate::{
    middleware::error::{Result}
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iat: usize,
    exp: usize,
}

pub fn create_token() -> Result<String> {
    dotenv().ok();

    let iat = Utc::now().timestamp() as usize;
    let exp = (Utc::now() + Duration::seconds(30)).timestamp() as usize;

    let claim = Claims {
        iat: iat,
        exp: exp
    };

    let secret = env::var("TOKEN_SECRET").expect("JWT_SECRET must be set");
    let secret = EncodingKey::from_secret(secret.as_bytes());
    
    encode(&Header::default(), &claim, &secret).map_err(|e| e.into())
}

pub fn verify_token(token: String) -> Result<bool> {
    let secret = env::var("TOKEN_SECRET").expect("JWT_SECRET must be set");
    let secret = DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(&token, &secret, &Validation::new(Algorithm::HS256))?;
    Ok(true)
}

// pub async fn require_auth<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    
//     Ok(next.run(req).await)
// }