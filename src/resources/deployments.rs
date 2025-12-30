use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use strum::Display;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct Deployment {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub kind_id: Uuid,
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

#[derive(Debug, Deserialize, Clone, Display)]
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

#[derive(Debug, Serialize)]
pub struct NewDeployment {
    #[serde(default)]
    pub name: String,
    pub kind_id: Uuid,
    pub cluster_id: Uuid,
    pub helm_chart_id: Uuid,
    pub config: Option<serde_json::Value>,
    pub values_override: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateDeployment {
    pub name: Option<String>,
    pub cluster_id: Option<Uuid>,
    pub helm_chart_id: Option<Uuid>,
    pub config: Option<serde_json::Value>,
    pub values_override: Option<Option<serde_json::Value>>,
    pub enabled: Option<bool>,
    pub description_md: Option<String>,
}

#[derive(Default, IntoVec)]
pub struct DeploymentFilters {
    #[kv(optional)]
    pub name: Option<String>,
    #[kv(optional)]
    pub kind_id: Option<String>,
    #[kv(optional)]
    pub cluster_id: Option<Uuid>,
    #[kv(optional)]
    pub enabled: Option<bool>,
    #[kv(optional)]
    pub env_id: Option<Uuid>,
}

impl PlatzClient {
    pub async fn deployments(&self, filters: DeploymentFilters) -> Result<Vec<Deployment>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/deployments")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn deployment(&self, deployment_id: Uuid) -> Result<Deployment> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/deployments/{deployment_id}"),
            )
            .send()
            .await?)
    }

    pub async fn update_deployment(
        &self,
        deployment_id: Uuid,
        update_deployment: UpdateDeployment,
    ) -> Result<Deployment> {
        Ok(self
            .request(
                reqwest::Method::PUT,
                format!("/api/v2/deployments/{deployment_id}"),
            )
            .send_with_body(update_deployment)
            .await?)
    }

    pub async fn create_deployment(&self, new_deployment: NewDeployment) -> Result<Deployment> {
        Ok(self
            .request(reqwest::Method::POST, "/api/v2/deployments")
            .send_with_body(new_deployment)
            .await?)
    }
}
