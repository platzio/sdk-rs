use crate::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct K8sResource {
    pub id: Uuid,
    pub last_updated_at: DateTime<Utc>,
    pub cluster_id: Uuid,
    pub deployment_id: Uuid,
    pub kind_id: Uuid,
    pub api_version: String,
    pub name: String,
    pub status_color: Vec<String>,
    pub metadata: serde_json::Value,
}

#[derive(Default, IntoVec)]
pub struct K8sResourceFilters {
    #[kv(optional)]
    pub name: Option<String>,
    #[kv(optional)]
    pub kind_id: Option<Uuid>,
    #[kv(optional)]
    pub cluster_id: Option<Uuid>,
    #[kv(optional)]
    pub deployment_id: Option<Uuid>,
}

impl PlatzClient {
    pub async fn k8s_resources(&self, filters: K8sResourceFilters) -> Result<Vec<K8sResource>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/k8s-resources")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn k8s_resource(&self, deployment_resource_id: Uuid) -> Result<K8sResource> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/k8s-resources/{deployment_resource_id}"),
            )
            .send()
            .await?)
    }
}
