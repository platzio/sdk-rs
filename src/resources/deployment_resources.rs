use crate::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Default, IntoVec)]
pub struct DeploymentResourceFilters {
    #[kv(optional)]
    pub type_id: Option<Uuid>,
}

impl PlatzClient {
    pub async fn deployment_resources(
        &self,
        filters: DeploymentResourceFilters,
    ) -> Result<Vec<DeploymentResource>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/deployment-resources")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }
    pub async fn deployment_resource(
        &self,
        deployment_resource_id: Uuid,
    ) -> Result<DeploymentResource> {
        Ok(self
            .request(
                reqwest::Method::POST,
                format!("/api/v2/deployment-resources/{deployment_resource_id}"),
            )
            .send()
            .await?)
    }

    pub async fn create_deployment_resource(
        &self,
        values: NewDeploymentResource,
    ) -> Result<DeploymentResource> {
        Ok(self
            .request(reqwest::Method::POST, "/api/v2/deployment-resources")
            .send_with_body(values)
            .await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
