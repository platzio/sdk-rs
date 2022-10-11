use crate::client::PlatzClient;
use crate::types::Uuid;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct K8sCluster {
    pub id: Uuid,
    pub env_id: Option<Uuid>,
    pub provider_id: String,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub name: String,
    pub region_name: String,
    pub is_ok: bool,
    pub not_ok_reason: Option<String>,
    pub ignore: bool,
    pub ingress_domain: Option<String>,
    pub ingress_class: Option<String>,
    pub ingress_tls_secret_name: Option<String>,
    pub grafana_url: Option<String>,
    pub grafana_datasource_name: Option<String>,
}

#[derive(Default, IntoVec)]
pub struct K8sClusterFilters {
    #[kv(optional)]
    pub name: Option<String>,
}

impl PlatzClient {
    pub async fn k8s_clusters(&self, filters: K8sClusterFilters) -> Result<Vec<K8sCluster>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/k8s-clusters")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }
}
