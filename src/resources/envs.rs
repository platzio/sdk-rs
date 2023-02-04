use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
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

impl PlatzClient {
    pub async fn envs(&self, filters: EnvFilters) -> Result<Vec<Env>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/envs")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }
}
