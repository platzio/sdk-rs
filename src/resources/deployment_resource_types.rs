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

impl PlatzClient {
    pub async fn find_global_deployment_resource_type(
        self,
        deployment_kind: String,
        key: String,
    ) -> Result<DeploymentResourceType> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/deployment-resource-types")
            .query("deployment_kind", &deployment_kind)
            .query("key", &key)
            .paginated_expect_one()
            .await?)
    }

    pub async fn find_deployment_resource_type(
        self,
        env_id: Uuid,
        deployment_kind: String,
        key: String,
    ) -> Result<DeploymentResourceType> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/deployment-resource-types")
            .query("env_id", &env_id.to_string())
            .query("deployment_kind", &deployment_kind)
            .query("key", &key)
            .paginated_expect_one()
            .await?)
    }
}
