use axum::{extract::State, Json, Extension};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::message::create_dm,
    routes::error::Result, 
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDmResponse {
    dm: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDmPayload {
    target: String,
}


#[debug_handler]
pub async fn create(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>, 
    Json(payload): Json<CreateDmPayload>,
) -> Result<Json<CreateDmResponse>> {
    let conn = &mut pool.acquire().await?;
    let dm = create_dm(conn, user, &payload.target).await?;

    Ok(Json(CreateDmResponse { dm }))
}
