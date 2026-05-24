use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, Display,
)]
pub enum UserDeploymentRole {
    Owner,
    Maintainer,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeploymentPermission {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub env_id: Uuid,
    pub user_id: Uuid,
    pub kind_id: Uuid,
    pub role: UserDeploymentRole,
}

#[derive(Default, IntoVec)]
pub struct DeploymentPermissionFilters {
    #[kv(optional)]
    pub env_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct NewDeploymentPermission {
    pub env_id: Uuid,
    pub user_id: Uuid,
    pub kind_id: Uuid,
    pub role: UserDeploymentRole,
}

impl PlatzClient {
    pub async fn deployment_permissions(
        &self,
        filters: DeploymentPermissionFilters,
    ) -> Result<Vec<DeploymentPermission>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/deployment-permissions")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn deployment_permission(
        &self,
        permission_id: Uuid,
    ) -> Result<DeploymentPermission> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/deployment-permissions/{permission_id}"),
            )
            .send()
            .await?)
    }

    pub async fn create_deployment_permission(
        &self,
        new_permission: NewDeploymentPermission,
    ) -> Result<DeploymentPermission> {
        Ok(self
            .request(reqwest::Method::POST, "/api/v2/deployment-permissions")
            .send_with_body(new_permission)
            .await?)
    }

    pub async fn delete_deployment_permission(&self, permission_id: Uuid) -> Result<()> {
        Ok(self
            .request(
                reqwest::Method::DELETE,
                format!("/api/v2/deployment-permissions/{permission_id}"),
            )
            .send_with_no_response()
            .await?)
    }
}
