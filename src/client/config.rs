use super::error::PlatzClientError;
use async_std::fs::read_to_string;
use chrono::prelude::*;
use serde::Deserialize;
use std::{env::var_os, ffi::OsString, io::ErrorKind};
use url::Url;

#[derive(Deserialize)]
enum AuthScheme {
    Bearer,
}

#[derive(Deserialize)]
pub(super) struct PlatzClientConfig {
    pub server_url: Url,
    scheme: AuthScheme,
    contents: String,
    expires_at: Option<DateTime<Utc>>,
}

impl PlatzClientConfig {
    /// Create a new PlatzClient by trying new_from_env, then
    /// new_from_secret, whichever succeeds first.
    /// If no token can be found, a PlatzClientError::NotFound is returned.
    pub async fn new() -> Result<Self, PlatzClientError> {
        if let Some(config) = Self::new_from_env()? {
            Ok(config)
        } else if let Some(config) = Self::new_from_secret().await? {
            Ok(config)
        } else {
            Err(PlatzClientError::NoConfigFound)
        }
    }

    /// Try creating PlatzClient from PLATZ_URL and PLATZ_API_TOKEN environment
    /// variables. If at least one of the variables is not defined, None is returned.
    /// If the variables exist and there's an error parsing them, this error is
    /// returned and no further configuration would be loaded.
    pub fn new_from_env() -> Result<Option<Self>, PlatzClientError> {
        let server_url: Option<Url> = var_os("PLATZ_URL")
            .map(OsString::into_string)
            .transpose()
            .map_err(|_| PlatzClientError::EnvVarParseError("PLATZ_URL"))?
            .map(|str| Url::parse(&str))
            .transpose()
            .map_err(|_| PlatzClientError::EnvVarParseError("PLATZ_URL"))?;
        let contents = var_os("PLATZ_API_TOKEN")
            .map(OsString::into_string)
            .transpose()
            .map_err(|_| PlatzClientError::EnvVarParseError("PLATZ_API_TOKEN"))?;
        match (server_url, contents) {
            (Some(server_url), Some(contents)) => Ok(Self {
                server_url,
                scheme: AuthScheme::Bearer,
                contents,
                expires_at: None,
            }
            .into()),
            _ => Ok(None),
        }
    }

    /// Try creating PlatzClient from files on disk. This works
    /// inside deployments when mapping the `platz-auth` secret
    /// under `/var/run/secrets/platz`. The secret is rotated
    /// regularly by Platz.
    pub async fn new_from_secret() -> Result<Option<Self>, PlatzClientError> {
        match read_to_string("/var/run/secrets/platz.json").await {
            Ok(contents) => Ok(Some(
                serde_json::from_str::<Self>(&contents)
                    .map_err(PlatzClientError::ConfigParseError)?,
            )),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(None),
                kind => Err(PlatzClientError::ConfigReadError(kind)),
            },
        }
    }

    /// Checks that the current credentials haven't expired
    pub fn expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            expires_at <= Utc::now()
        } else {
            false
        }
    }

    /// Returns Authorization header content, possibly refreshing the current
    /// credentials.
    pub async fn get_authorization(&self) -> Result<String, PlatzClientError> {
        match self.scheme {
            AuthScheme::Bearer => Ok(format!("Bearer {}", self.contents)),
        }
    }
}
