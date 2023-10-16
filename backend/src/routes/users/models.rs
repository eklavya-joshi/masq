use serde::{Serialize, Deserialize};

use crate::api::user::UserInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct FindUsersQuery {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindUsersResponse {
    pub users: Vec<UserInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutPayload {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub result: String
}