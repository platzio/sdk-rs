use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct UserToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Default, IntoVec)]
pub struct UserTokenFilters {
    #[kv(optional)]
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct NewUserToken {
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UserTokenCreationResponse {
    pub created_token: String,
}

impl PlatzClient {
    pub async fn user_tokens(&self, filters: UserTokenFilters) -> Result<Vec<UserToken>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/user-tokens")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn user_token(&self, token_id: Uuid) -> Result<UserToken> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/user-tokens/{token_id}"),
            )
            .send()
            .await?)
    }

    pub async fn create_user_token(
        &self,
        new_user_token: NewUserToken,
    ) -> Result<UserTokenCreationResponse> {
        Ok(self
            .request(reqwest::Method::POST, "/api/v2/user-tokens")
            .send_with_body(new_user_token)
            .await?)
    }

    pub async fn delete_user_token(&self, token_id: Uuid) -> Result<()> {
        Ok(self
            .request(
                reqwest::Method::DELETE,
                format!("/api/v2/user-tokens/{token_id}"),
            )
            .send_with_no_response()
            .await?)
    }
}
