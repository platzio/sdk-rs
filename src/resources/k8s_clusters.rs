use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Default, IntoVec, Debug, Serialize)]
pub struct K8sClusterFilters {
    #[kv(optional)]
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateK8sCluster {
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub env_id: Option<Option<Uuid>>,
    pub ignore: Option<bool>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub ingress_domain: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub ingress_class: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub ingress_tls_secret_name: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub grafana_url: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub grafana_datasource_name: Option<Option<String>>,
}

impl PlatzClient {
    pub async fn k8s_clusters(&self, filters: K8sClusterFilters) -> Result<Vec<K8sCluster>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/k8s-clusters")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn k8s_cluster(&self, k8s_cluster_id: Uuid) -> Result<K8sCluster> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/k8s-clusters/{k8s_cluster_id}"),
            )
            .send()
            .await?)
    }

    pub async fn update_k8s_cluster(
        &self,
        k8s_cluster_id: Uuid,
        update_deployment: UpdateK8sCluster,
    ) -> Result<K8sCluster> {
        Ok(self
            .request(
                reqwest::Method::PUT,
                format!("/api/v2/k8s-clusters/{k8s_cluster_id}"),
            )
            .send_with_body(update_deployment)
            .await?)
    }
}
