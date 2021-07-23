use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod space;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityRequest {
    pub name: String,
    pub color: String,
    pub integration: String,
    pub space_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityResponse {
    pub id: String,
    pub name: String,
    pub color: String,
    pub integration: String,
    pub space_id: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagRequest {
    pub label: String,
    pub key: Option<String>,
    pub scope: String,
    pub space_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagResponse {
    pub id: i64,
    pub key: String,
    pub label: String,
    pub scope: String,
    pub space_id: String,
}
