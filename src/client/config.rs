use super::error::PlatzClientError;
use async_std::fs::read_to_string;
use chrono::prelude::*;
use futures::future::try_join3;
use reqwest::header::{HeaderName, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use std::{env::var_os, ffi::OsString, io::ErrorKind, path::PathBuf};
use url::Url;

fn always_false() -> bool {
    false
}

#[derive(Deserialize, Clone, Debug)]
struct ProfileInfo {
    url: String,
    #[serde(flatten)]
    credentials: Credentials,
    #[serde(default = "always_false")]
    default_profile: bool,
}

impl ProfileInfo {
    pub fn to_client(&self) -> Result<PlatzClientConfig, PlatzClientError> {
        let server_url = self
            .url
            .parse()
            .map_err(PlatzClientError::MountedUrlParseError)?;

        Ok(match &self.credentials {
            Credentials::AccessToken {
                access_token,
                expired_at,
            } => PlatzClientConfig {
                server_url,
                scheme: AuthScheme::Bearer,
                contents: access_token.clone(),
                expires_at: *expired_at,
            },
            Credentials::UserToken { user_token } => PlatzClientConfig {
                server_url,
                scheme: AuthScheme::XPlatzToken,
                contents: user_token.clone(),
                expires_at: None,
            },
        })
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
enum Credentials {
    AccessToken {
        access_token: String,
        expired_at: Option<DateTime<Utc>>,
    },
    UserToken {
        user_token: String,
    },
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    profile: std::collections::HashMap<String, ProfileInfo>,
}

impl TomlConfig {
    pub fn get_default_profile(&self) -> Result<Option<&ProfileInfo>, PlatzClientError> {
        let mut default_profile: Option<&ProfileInfo> = None;
        for profile_info in self.profile.values() {
            if profile_info.default_profile {
                if default_profile.is_some() {
                    return Err(PlatzClientError::ConfigTomlError(
                        "Multiple default profiles",
                    ));
                }
                default_profile = Some(profile_info)
            }
        }
        Ok(default_profile)
    }
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

fn get_env_var_or_value(env_var_name: &'static str) -> Result<Option<String>, PlatzClientError> {
    var_os(env_var_name)
        .map(OsString::into_string)
        .transpose()
        .map_err(move |_| PlatzClientError::EnvVarParseError(env_var_name))
}
impl PlatzClientConfig {
    /// Create a new PlatzClient by trying new_from_env, then
    /// new_from_secret, whichever succeeds first.
    /// If no token can be found, a PlatzClientError::NotFound is returned.
    pub async fn new() -> Result<Self, PlatzClientError> {
        if let Some(config) = Self::new_from_env()? {
            Ok(config)
        } else if let Some(config) = Self::new_from_configuration(None).await? {
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
        let server_url: Option<Url> = get_env_var_or_value("PLATZ_URL")?
            .map(|str| Url::parse(&str))
            .transpose()
            .map_err(|_| PlatzClientError::EnvVarParseError("PLATZ_URL"))?;
        let (scheme, contents) = if let Some(api_token) = get_env_var_or_value("PLATZ_API_TOKEN")? {
            (AuthScheme::XPlatzToken, Some(api_token))
        } else {
            (
                AuthScheme::Bearer,
                get_env_var_or_value("PLATZ_USER_TOKEN")?,
            )
        };
        match (server_url, contents) {
            (Some(server_url), Some(contents)) => Ok(Self {
                server_url,
                scheme,
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
        profile_name: &Option<String>,
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
                let default_profile = toml_conf.get_default_profile()?;
                let requested_profile: Option<String> = if profile_name.is_some() {
                    profile_name.clone()
                } else {
                    var_os("PLATZ_PROFILE")
                        .map(OsString::into_string)
                        .transpose()
                        .map_err(|_| PlatzClientError::EnvVarParseError("PLATZ_PROFILE"))?
                };
                let profile_info = if let Some(name) = requested_profile {
                    toml_conf.profile.get(name.as_str()).ok_or_else(|| {
                        PlatzClientError::ConfigTomlError(
                            "Requested profile does not exist in configuration",
                        )
                    })?
                } else {
                    default_profile.ok_or_else(|| {
                        PlatzClientError::ConfigTomlError("Not default profile configured")
                    })?
                };

                Ok(Some(profile_info.to_client()?))
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
        let Some(home_dir) = dirs::home_dir() else {
            return Ok(None);
        };
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
    pub async fn get_authorization(&self) -> Result<(HeaderName, HeaderValue), PlatzClientError> {
        match self.scheme {
            AuthScheme::Bearer => Ok((
                AUTHORIZATION,
                HeaderValue::try_from(format!("Bearer {}", self.contents))
                    .map_err(|_| PlatzClientError::ErrorCreatingAuthHeader)?,
            )),
            AuthScheme::XPlatzToken => Ok((
                HeaderName::try_from("x-platz-token")
                    .map_err(|_| PlatzClientError::ErrorCreatingAuthHeader)?,
                HeaderValue::try_from(self.contents.clone())
                    .map_err(|_| PlatzClientError::ErrorCreatingAuthHeader)?,
            )),
        }
    }
}
