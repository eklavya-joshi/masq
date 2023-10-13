use axum::{http::Request, middleware::Next, response::Response, headers::{Authorization, authorization::Bearer, HeaderMapExt}, extract::State};
use sqlx::{query, PgPool};

use crate::middleware::error::{Error, Result};

use super::jwt::verify_token;

pub async fn require_auth<T>(State(pool): State<PgPool>, mut req: Request<T>, next: Next<T>) -> Result<Response> {
    
    let token = req.headers().typed_get::<Authorization<Bearer>>().ok_or(Error::InvalidToken)?.token().to_owned();

    let conn = &mut pool.acquire().await?;

    let user = query!(
        r#"SELECT * FROM Users WHERE token=$1"#, 
        token
        )
        .fetch_optional(conn.as_mut())
        .await?;

    let Some(_) = user else { return Err(Error::Unauthorised); };

    verify_token(token.clone())?;

    let bearer = Authorization::bearer(&token).map_err(|_| Error::InvalidToken)?;
    req.headers_mut().typed_insert::<Authorization<Bearer>>(bearer);

    Ok(next.run(req).await)
}