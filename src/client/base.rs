use super::config::PlatzClientConfig;
use super::error::PlatzClientError;
use super::request::PlatzRequest;
use async_std::sync::RwLock;
use reqwest::Url;

pub struct PlatzClient {
    config: RwLock<PlatzClientConfig>,
}

impl PlatzClient {
    pub async fn new() -> Result<Self, PlatzClientError> {
        Ok(Self {
            config: RwLock::new(PlatzClientConfig::new().await?),
        })
    }

    pub(super) async fn build_url(&self, path: &str) -> Result<Url, PlatzClientError> {
        self.config
            .read()
            .await
            .server_url
            .join(path)
            .map_err(PlatzClientError::UrlJoinError)
    }

    pub(super) async fn authorization(&self) -> Result<(String, String), PlatzClientError> {
        let mut config = self.config.write().await;
        if config.expired() {
            *config = PlatzClientConfig::new().await?;
        }

        config
            .get_authorization()
            .await
            .map_err(|_| PlatzClientError::ErrorCreatingAuthHeader)
    }

    pub fn request<S>(&self, method: reqwest::Method, path: S) -> PlatzRequest
    where
        S: AsRef<str>,
    {
        PlatzRequest::new(self, method, path)
    }
}
