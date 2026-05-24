use crate::client::PlatzClient;
use anyhow::Result;
use chrono::prelude::*;
use kv_derive::{prelude::*, IntoVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct BotToken {
    pub id: Uuid,
    pub bot_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub created_by_user_id: Uuid,
}

#[derive(Default, IntoVec)]
pub struct BotTokenFilters {
    #[kv(optional)]
    pub bot_id: Option<Uuid>,
    #[kv(optional)]
    pub created_by_user_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct CreateBotToken {
    pub bot_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreatedBotToken {
    pub created_token: String,
}

impl PlatzClient {
    pub async fn bot_tokens(&self, filters: BotTokenFilters) -> Result<Vec<BotToken>> {
        Ok(self
            .request(reqwest::Method::GET, "/api/v2/bot-tokens")
            .add_to_query(filters.into_vec())
            .paginated()
            .await?)
    }

    pub async fn bot_token(&self, token_id: Uuid) -> Result<BotToken> {
        Ok(self
            .request(
                reqwest::Method::GET,
                format!("/api/v2/bot-tokens/{token_id}"),
            )
            .send()
            .await?)
    }

    pub async fn create_bot_token(&self, body: CreateBotToken) -> Result<CreatedBotToken> {
        Ok(self
            .request(reqwest::Method::POST, "/api/v2/bot-tokens")
            .send_with_body(body)
            .await?)
    }

    pub async fn delete_bot_token(&self, token_id: Uuid) -> Result<()> {
        Ok(self
            .request(
                reqwest::Method::DELETE,
                format!("/api/v2/bot-tokens/{token_id}"),
            )
            .send_with_no_response()
            .await?)
    }
}
