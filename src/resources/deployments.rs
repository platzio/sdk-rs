use crate::client::PlatzClient;
use crate::types::Uuid;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Deployment {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub kind: String,
    pub cluster_id: Uuid,
    pub enabled: bool,
    pub status: DeploymentStatus,
    pub description_md: Option<String>,
    pub reason: Option<String>,
    pub revision_id: Option<Uuid>,
    pub reported_status: Option<serde_json::Value>,
    pub helm_chart_id: Uuid,
    pub config: serde_json::Value,
    pub values_override: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub enum DeploymentStatus {
    Unknown,
    Installing,
    Renaming,
    Upgrading,
    Running,
    Error,
    Uninstalling,
    Uninstalled,
    Deleting,
}

#[derive(Default, IntoVec)]
pub struct DeploymentFilters {
    #[kv(optional)]
    pub name: Option<String>,
    #[kv(optional)]
    pub kind: Option<String>,
    #[kv(optional)]
    pub cluster_id: Option<Uuid>,
    #[kv(optional)]
    pub enabled: Option<bool>,
}

impl PlatzClient {
    pub async fn deployments(&self, filters: DeploymentFilters) -> Result<Vec<Deployment>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/deployments")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }
}
