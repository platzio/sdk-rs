use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct DeploymentKind {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
}

#[derive(Default, IntoVec)]
pub struct DeploymentKindFilters {
    #[kv(optional)]
    pub name: Option<String>,
}

impl PlatzClient {
    pub async fn deployment_kinds(
        &self,
        filters: DeploymentKindFilters,
    ) -> Result<Vec<DeploymentKind>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/deployments")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn deployment_kind(&self, deployment_kind_id: Uuid) -> Result<DeploymentKind> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/deployment-kinds/{deployment_kind_id}"),
            )
            .send()
            .await?)
    }
}
