use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;
use uuid::Uuid;

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
    pub async fn send(self) -> Result<DeploymentResource> {
        let url = Url::parse(&env::var("PLATZ_URL")?)?.join("/api/v1/deployment-resources")?;
        Ok(reqwest::Client::new()
            .post(url)
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
