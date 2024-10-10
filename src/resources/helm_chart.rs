use crate::client::PlatzClient;
use anyhow::{bail, Result};
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct HelmChart {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub helm_registry_id: Uuid,
    pub image_digest: String,
    pub image_tag: String,
    pub available: bool,
    pub values_ui: Option<platz_chart_ext::UiSchema>,
    pub actions_schema: Option<platz_chart_ext::ChartExtActions>,
    pub features: Option<platz_chart_ext::ChartExtFeatures>,
    pub resource_types: Option<serde_json::Value>,
    pub error: Option<String>,
    pub tag_format_id: Option<Uuid>,
    pub parsed_version: Option<String>,
    pub parsed_revision: Option<String>,
    pub parsed_branch: Option<String>,
    pub parsed_commit: Option<String>,
}

impl HelmChart {
    pub fn get_actions_schema(&self) -> Result<platz_chart_ext::ChartExtActions> {
        match self.actions_schema.as_ref() {
            Some(schema) => Ok(schema.clone()),
            None => bail!("No actions schema for helm chart"),
        }
    }

    pub fn get_actions(&self) -> Result<Vec<platz_chart_ext::ChartExtActionV0>> {
        Ok(if let Some(actions_schema) = self.actions_schema.as_ref() {
            actions_schema.get_actions()
        } else {
            Vec::new()
        })
    }
}

#[derive(Default, IntoVec)]
pub struct HelmChartFilters {
    #[kv(optional)]
    pub helm_registry_id: Option<Uuid>,
    #[kv(optional)]
    pub parsed_branch: Option<String>,
    #[kv(optional)]
    pub in_use: Option<bool>,
    #[kv(optional)]
    pub kind_id: Option<Uuid>,
}

impl PlatzClient {
    pub async fn helm_charts(&self, filters: HelmChartFilters) -> Result<Vec<HelmChart>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/helm-charts")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }
    pub async fn helm_chart(&self, helm_chart_id: Uuid) -> Result<HelmChart> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/helm-charts/{helm_chart_id}"),
            )
            .send()
            .await?)
    }
}
