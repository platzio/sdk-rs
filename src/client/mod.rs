mod config;
mod error;

use async_std::sync::RwLock;
use config::PlatzClientConfig;
use error::PlatzClientError;

lazy_static::lazy_static! {
    static ref HTTP_USER_AGENT: String = format!(
        "{}/{}/{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        option_env!("CARGO_BIN_NAME").unwrap_or("lib")
    );
}

pub struct PlatzClient {
    config: RwLock<PlatzClientConfig>,
}

impl PlatzClient {
    pub async fn new() -> Result<Self, PlatzClientError> {
        Ok(Self {
            config: RwLock::new(PlatzClientConfig::new().await?),
        })
    }

    /// Get a reqwest::RequestBuilder populated with credentials for `path`.
    pub async fn request(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> Result<reqwest::RequestBuilder, PlatzClientError> {
        let mut config = self.config.write().await;
        if config.expired() {
            *config = PlatzClientConfig::new().await?;
        }

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            config
                .get_authorization()
                .await?
                .parse()
                .map_err(|_| PlatzClientError::ErrorCreatingAuthHeader)?,
        );

        let url = config
            .server_url
            .join(path)
            .map_err(PlatzClientError::UrlJoinError)?;

        Ok(reqwest::Client::builder()
            .user_agent(HTTP_USER_AGENT.clone())
            .default_headers(headers)
            .gzip(true)
            .brotli(true)
            .deflate(true)
            .build()
            .map_err(PlatzClientError::ClientBuildError)?
            .request(method, url))
    }
}
