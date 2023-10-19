use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::Request,
    middleware::Next,
    response::Response,
};
use sqlx::{query, PgPool};
use tower_cookies::Cookies;

use super::{jwt::verify_token, Error, Result};

pub async fn require_auth<T: std::fmt::Debug>(
    State(pool): State<PgPool>,
    cookies: Cookies,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response> {
    // let token = req.headers().typed_get::<Authorization<Bearer>>().ok_or(Error::InvalidToken)?.token().to_owned();

    let token = cookies
        .get("token")
        .map(|c| c.value().to_string())
        .ok_or(Error::InvalidToken)?;

    let conn = &mut pool.acquire().await?;

    let user = query!(r#"SELECT * FROM Users WHERE token=$1"#, token)
        .fetch_optional(conn.as_mut())
        .await?;

    let Some(_) = user else {
        return Err(Error::Unauthorised);
    };
    let user = user.unwrap();

    verify_token(&token)?;

    let bearer = Authorization::bearer(&token).map_err(|_| Error::InvalidToken)?;

    req.extensions_mut().insert(user.id);
    req.headers_mut()
        .typed_insert::<Authorization<Bearer>>(bearer);

    Ok(next.run(req).await)
}
