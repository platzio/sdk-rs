use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct Bot {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub display_name: String,
}

#[derive(Default, IntoVec)]
pub struct BotFilters {
    #[kv(optional)]
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NewBot {
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateBot {
    pub display_name: Option<String>,
}

impl PlatzClient {
    pub async fn bots(&self, filters: BotFilters) -> Result<Vec<Bot>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/bots")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn bot(&self, bot_id: Uuid) -> Result<Bot> {
        Ok(self
            .request(reqwest::Method::GET, format!("/api/v2/bots/{bot_id}"))
            .send()
            .await?)
    }

    pub async fn create_bot(&self, new_bot: NewBot) -> Result<Bot> {
        Ok(self
            .request(reqwest::Method::POST, "/api/v2/bots")
            .send_with_body(new_bot)
            .await?)
    }

    pub async fn update_bot(&self, bot_id: Uuid, update_bot: UpdateBot) -> Result<Bot> {
        Ok(self
            .request(reqwest::Method::PUT, format!("/api/v2/bots/{bot_id}"))
            .send_with_body(update_bot)
            .await?)
    }

    pub async fn delete_bot(&self, bot_id: Uuid) -> Result<()> {
        Ok(self
            .request(reqwest::Method::DELETE, format!("/api/v2/bots/{bot_id}"))
            .send_with_no_response()
            .await?)
    }
}
