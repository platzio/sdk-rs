use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct Env {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub node_selector: serde_json::Value,
    pub tolerations: serde_json::Value,
    pub auto_add_new_users: bool,
}

#[derive(Default, IntoVec)]
pub struct EnvFilters {
    #[kv(optional)]
    pub name: Option<String>,
    #[kv(optional)]
    pub auto_add_new_users: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UpdateEnv {
    pub name: Option<String>,
    pub node_selector: Option<serde_json::Value>,
    pub tolerations: Option<serde_json::Value>,
    pub auto_add_new_users: Option<bool>,
}

impl PlatzClient {
    pub async fn envs(&self, filters: EnvFilters) -> Result<Vec<Env>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/envs")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn env(&self, env_id: Uuid) -> Result<Env> {
        Ok(self
            .request(reqwest::Method::GET, format!("/api/v2/envs/{env_id}"))
            .send()
            .await?)
    }

    pub async fn update_env(&self, env_id: Uuid, update_env: UpdateEnv) -> Result<Env> {
        Ok(self
            .request(reqwest::Method::GET, format!("/api/v2/envs/{env_id}"))
            .send_with_body(update_env)
            .await?)
    }
}
