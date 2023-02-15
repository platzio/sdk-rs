use super::error::PlatzClientError;
use async_std::fs::read_to_string;
use chrono::prelude::*;
use futures::future::try_join3;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::{env::var_os, ffi::OsString, io::ErrorKind, path::PathBuf};
use url::Url;

#[derive(Debug, Deserialize)]
struct ServerInfo {
    url: String,
    token: String,
    scheme: AuthScheme,
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    default: Option<String>,
    servers: std::collections::HashMap<String, ServerInfo>,
}

#[derive(Debug, Deserialize, Clone)]
enum AuthScheme {
    Bearer,
    XPlatzToken,
}

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
        if let Some(config) = Self::new_from_configuration(None).await? {
            Ok(config)
        } else if let Some(config) = Self::new_from_env()? {
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
    /// inside deployments when mapping the `platz-creds` secret
    /// under `/var/run/secrets/platz`. The secret is rotated
    /// regularly by Platz.
    pub async fn new_from_secret() -> Result<Option<Self>, PlatzClientError> {
        match try_join3(
            read_to_string("/var/run/secrets/platz/access_token"),
            read_to_string("/var/run/secrets/platz/server_url"),
            read_to_string("/var/run/secrets/platz/expires_at"),
        )
        .await
        {
            Ok((access_token, server_url, expires_at)) => Ok(Some(PlatzClientConfig {
                server_url: server_url
                    .parse()
                    .map_err(PlatzClientError::MountedUrlParseError)?,
                scheme: AuthScheme::Bearer,
                contents: access_token,
                expires_at: Some(
                    expires_at
                        .parse()
                        .map_err(PlatzClientError::MountedExpiryParseError)?,
                ),
            })),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(None),
                kind => Err(PlatzClientError::ConfigReadError(kind)),
            },
        }
    }

    async fn from_config_toml(
        base_path: Option<PathBuf>,
        server_name: &Option<String>,
    ) -> Result<Option<Self>, PlatzClientError> {
        if base_path.is_none() {
            return Ok(None);
        }
        let mut conf_path = base_path.unwrap();
        conf_path.push("platz");
        conf_path.push("config.toml");

        match read_to_string(conf_path).await {
            Ok(toml_data) => {
                let toml_conf: TomlConfig = toml::from_str(&toml_data)
                    .map_err(PlatzClientError::ConfigDeserializationError)?;
                let Some(requested_server_name) = server_name.clone().or(toml_conf.default) else {return Ok(None)};
                let server_info = toml_conf
                    .servers
                    .get(requested_server_name.as_str())
                    .unwrap();
                Ok(Some(Self {
                    server_url: server_info
                        .url
                        .parse()
                        .map_err(PlatzClientError::MountedUrlParseError)?,
                    scheme: server_info.scheme.clone(),
                    contents: server_info.token.clone(),
                    expires_at: None,
                }))
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(None),
                kind => Err(PlatzClientError::ConfigReadError(kind)),
            },
        }
    }

    // Try creating PlatzClient from configuration files. This is the recommended
    // way for CLI tools.
    pub async fn new_from_configuration(
        server_name: Option<String>,
    ) -> Result<Option<Self>, PlatzClientError> {
        let Some(home_dir) = dirs::home_dir() else { return Ok(None) };
        let mut conf_path = home_dir;
        conf_path.push(".config");
        let config_data = Self::from_config_toml(Some(conf_path), &server_name).await?;
        if config_data.is_some() {
            Ok(config_data)
        } else {
            Self::from_config_toml(dirs::config_dir(), &server_name).await
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
    pub async fn get_authorization(&self) -> Result<(String, String), PlatzClientError> {
        match self.scheme {
            AuthScheme::Bearer => Ok((
                AUTHORIZATION.to_string(),
                format!("Bearer {}", self.contents),
            )),
            AuthScheme::XPlatzToken => Ok(("x-platz-token".to_string(), self.contents.clone())),
        }
    }
}
