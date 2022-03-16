use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentResourceType {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub env_id: Option<Uuid>,
    pub deployment_kind: String,
    pub key: String,
    pub spec: serde_json::Value,
}

impl DeploymentResourceType {
    pub async fn find_global(deployment_kind: String, key: String) -> Result<Self> {
        let mut url =
            Url::parse(&env::var("PLATZ_URL")?)?.join("/api/v1/deployment-resource-types")?;
        url.query_pairs_mut()
            .append_pair("deployment_kind", &deployment_kind)
            .append_pair("key", &key);
        Ok(reqwest::get(url).await?.json().await?)
    }

    pub async fn find(env_id: Uuid, deployment_kind: String, key: String) -> Result<Self> {
        let mut url =
            Url::parse(&env::var("PLATZ_URL")?)?.join("/api/v1/deployment-resource-types")?;
        url.query_pairs_mut()
            .append_pair("env_id", &env_id.to_string())
            .append_pair("deployment_kind", &deployment_kind)
            .append_pair("key", &key);
        Ok(reqwest::get(url).await?.json().await?)
    }
}
