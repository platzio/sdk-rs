use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct Secret {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub env_id: Uuid,
    pub collection: String,
    pub name: String,
}

#[derive(Default, IntoVec)]
pub struct SecretFilters {
    #[kv(optional)]
    pub name: Option<String>,
    #[kv(optional)]
    pub env_id: Option<Uuid>,
    #[kv(optional)]
    pub collection: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateSecret {
    name: Option<String>,
    contents: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NewSecret {
    pub env_id: Uuid,
    pub collection: String,
    pub name: String,
    pub contents: String,
}

impl PlatzClient {
    pub async fn secrets(&self, filters: SecretFilters) -> Result<Vec<Secret>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/secrets")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn secret(&self, secret_id: Uuid) -> Result<Secret> {
        Ok(self
            .request(reqwest::Method::GET, format!("/api/v2/secrets/{secret_id}"))
            .send()
            .await?)
    }

    pub async fn update_secret(
        &self,
        secret_id: Uuid,
        update_secret: UpdateSecret,
    ) -> Result<Secret> {
        Ok(self
            .request(reqwest::Method::PUT, format!("/api/v2/secrets/{secret_id}"))
            .send_with_body(update_secret)
            .await?)
    }

    pub async fn create_secret(&self, new_secret: NewSecret) -> Result<Secret> {
        Ok(self
            .request(reqwest::Method::POST, "/api/v2/secrets")
            .send_with_body(new_secret)
            .await?)
    }
}
