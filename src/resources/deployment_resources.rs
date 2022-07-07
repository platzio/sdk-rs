use crate::types::Uuid;
use crate::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentResource {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub type_id: Uuid,
    pub deployment_id: Option<Uuid>,
    pub name: String,
    pub exists: bool,
    pub props: serde_json::Value,
    pub sync_status: SyncStatus,
    pub sync_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewDeploymentResource {
    pub id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub type_id: Uuid,
    pub deployment_id: Uuid,
    pub name: String,
    pub props: serde_json::Value,
    pub sync_status: Option<SyncStatus>,
}

impl NewDeploymentResource {
    pub async fn send(self, client: &PlatzClient) -> Result<DeploymentResource> {
        Ok(client
            .request(reqwest::Method::POST, "/api/v1/deployment-resources")
            .await?
            .json(&self)
            .send()
            .await?
            .json()
            .await?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SyncStatus {
    Creating,
    Updating,
    Deleting,
    Ready,
    Error,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self::Creating
    }
}
