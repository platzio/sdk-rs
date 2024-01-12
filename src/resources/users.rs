use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub display_name: String,
    pub email: String,
    pub is_admin: bool,
    pub is_active: bool,
}

#[derive(Default, IntoVec)]
pub struct UserFilter {
    #[kv(optional)]
    pub display_name: Option<String>,
    #[kv(optional)]
    pub email: Option<String>,
    #[kv(optional)]
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UpdateUser {
    pub is_admin: Option<bool>,
    pub is_active: Option<bool>,
}

impl PlatzClient {
    pub async fn users(&self, filters: UserFilter) -> Result<Vec<User>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/users")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn user(&self, user_id: Uuid) -> Result<User> {
        Ok(self
            .request(reqwest::Method::GET, format!("/api/v2/users/{user_id}"))
            .send()
            .await?)
    }

    pub async fn update_user(&self, user_id: Uuid, update_user: UpdateUser) -> Result<User> {
        Ok(self
            .request(reqwest::Method::PUT, format!("/api/v2/users/{user_id}"))
            .send_with_body(update_user)
            .await?)
    }
}
