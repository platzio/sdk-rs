use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct HelmTagFormat {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub pattern: String,
}

#[derive(Default, IntoVec)]
pub struct HelmTagFormatFilters {
    #[kv(optional)]
    pub pattern: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NewHelmTagFormat {
    pub pattern: String,
}

impl PlatzClient {
    pub async fn helm_tag_formats(
        &self,
        filters: HelmTagFormatFilters,
    ) -> Result<Vec<HelmTagFormat>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/helm-tag-formats")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn helm_tag_format(&self, format_id: Uuid) -> Result<HelmTagFormat> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/helm-tag-formats/{format_id}"),
            )
            .send()
            .await?)
    }

    pub async fn create_helm_tag_format(
        &self,
        new_format: NewHelmTagFormat,
    ) -> Result<HelmTagFormat> {
        Ok(self
            .request(reqwest::Method::POST, "/api/v2/helm-tag-formats")
            .send_with_body(new_format)
            .await?)
    }

    pub async fn delete_helm_tag_format(&self, format_id: Uuid) -> Result<()> {
        Ok(self
            .request(
                reqwest::Method::DELETE,
                format!("/api/v2/helm-tag-formats/{format_id}"),
            )
            .send_with_no_response()
            .await?)
    }
}
