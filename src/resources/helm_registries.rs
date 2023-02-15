use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct HelmRegistry {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub domain_name: String,
    pub repo_name: String,
    pub kind: String,
    pub available: bool,
    pub fa_icon: String,
}

#[derive(Default, IntoVec)]
pub struct HelmRegistryFilters {
    #[kv(optional)]
    pub repo_name: Option<String>,
    #[kv(optional)]
    pub kind: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateHelmRegistry {
    pub fa_icon: Option<String>,
}

impl PlatzClient {
    pub async fn helm_registries(&self, filters: HelmRegistryFilters) -> Result<Vec<HelmRegistry>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/helm-registries")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn helm_registry(&self, registry_id: Uuid) -> Result<HelmRegistry> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/helm-registry/{registry_id}"),
            )
            .send()
            .await?)
    }

    pub async fn update_helm_registry(
        &self,
        registry_id: Uuid,
        update_registry: UpdateHelmRegistry,
    ) -> Result<HelmRegistry> {
        Ok(self
            .request(
                reqwest::Method::PUT,
                format!("/api/v2/helm-registry/{registry_id}"),
            )
            .send_with_body(update_registry)
            .await?)
    }
}
