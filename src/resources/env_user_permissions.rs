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
pub enum EnvUserRole {
    Admin,
    User,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnvUserPermission {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub env_id: Uuid,
    pub user_id: Uuid,
    pub role: EnvUserRole,
}

#[derive(Default, IntoVec)]
pub struct EnvUserPermissionFilters {
    #[kv(optional)]
    pub env_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct NewEnvUserPermission {
    pub env_id: Uuid,
    pub user_id: Uuid,
    pub role: EnvUserRole,
}

impl PlatzClient {
    pub async fn env_user_permissions(
        &self,
        filters: EnvUserPermissionFilters,
    ) -> Result<Vec<EnvUserPermission>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/env-user-permissions")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn env_user_permission(&self, permission_id: Uuid) -> Result<EnvUserPermission> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/env-user-permissions/{permission_id}"),
            )
            .send()
            .await?)
    }

    pub async fn create_env_user_permission(
        &self,
        new_permission: NewEnvUserPermission,
    ) -> Result<EnvUserPermission> {
        Ok(self
            .request(reqwest::Method::POST, "/api/v2/env-user-permissions")
            .send_with_body(new_permission)
            .await?)
    }

    pub async fn delete_env_user_permission(&self, permission_id: Uuid) -> Result<()> {
        Ok(self
            .request(
                reqwest::Method::DELETE,
                format!("/api/v2/env-user-permissions/{permission_id}"),
            )
            .send_with_no_response()
            .await?)
    }
}
