use crate::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
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
    pub async fn find_global(
        client: &PlatzClient,
        deployment_kind: String,
        key: String,
    ) -> Result<Self> {
        Ok(client
            .request(reqwest::Method::GET, "/api/v1/deployment-resource-types")
            .await?
            .query(&["deployment_kind", &deployment_kind])
            .query(&["key", &key])
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn find(
        client: &PlatzClient,
        env_id: Uuid,
        deployment_kind: String,
        key: String,
    ) -> Result<Self> {
        Ok(client
            .request(reqwest::Method::GET, "/api/v1/deployment-resource-types")
            .await?
            .query(&["env_id", &env_id.to_string()])
            .query(&["deployment_kind", &deployment_kind])
            .query(&["key", &key])
            .send()
            .await?
            .json()
            .await?)
    }
}
