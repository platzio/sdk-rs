use crate::{PlatzClient, PlatzRequest};
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{IntoVec, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeploymentResourceType {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub env_id: Option<Uuid>,
    pub deployment_kind_id: Uuid,
    pub key: String,
    pub spec: serde_json::Value,
}

#[derive(Default, IntoVec)]
pub struct DeploymentResourceTypeFilters {
    #[kv(optional)]
    pub env_id: Option<Uuid>,
    #[kv(optional)]
    pub deployment_kind_id: Option<Uuid>,
    #[kv(optional)]
    pub key: Option<String>,
}

impl<'s> PlatzClient {
    fn deployment_resource_types_request_builder(
        &'s self,
        filters: DeploymentResourceTypeFilters,
    ) -> PlatzRequest<'s> {
        self.request(reqwest::Method::GET, "/api/v2/deployment-resource-types")
            .add_to_query(filters.into_vec())
    }

    pub async fn deployment_resource_types(
        &self,
        filters: DeploymentResourceTypeFilters,
    ) -> Result<Vec<DeploymentResourceType>> {
        Ok(self
            .deployment_resource_types_request_builder(filters)
            .paginated()
            .await?)
    }

    pub async fn deployment_resource_type(
        &self,
        deployment_resource_type_id: Uuid,
    ) -> Result<DeploymentResourceType> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/deployment-resource-types/{deployment_resource_type_id}"),
            )
            .send()
            .await?)
    }

    pub async fn find_global_deployment_resource_type(
        self,
        deployment_kind_id: Uuid,
        key: String,
    ) -> Result<DeploymentResourceType> {
        Ok(self
            .deployment_resource_types_request_builder(DeploymentResourceTypeFilters {
                deployment_kind_id: Some(deployment_kind_id),
                key: Some(key),
                ..DeploymentResourceTypeFilters::default()
            })
            .paginated_expect_one()
            .await?)
    }

    pub async fn find_deployment_resource_type(
        self,
        env_id: Uuid,
        deployment_kind_id: Uuid,
        key: String,
    ) -> Result<DeploymentResourceType> {
        Ok(self
            .deployment_resource_types_request_builder(DeploymentResourceTypeFilters {
                env_id: Some(env_id),
                deployment_kind_id: Some(deployment_kind_id),
                key: Some(key),
            })
            .paginated_expect_one()
            .await?)
    }
}
