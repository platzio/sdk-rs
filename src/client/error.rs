#[derive(Debug, thiserror::Error)]
pub enum PlatzClientError {
    #[error(r###"Could not find any Platz config. Use one of the following methods:
    1. Set the PLATZ_URL and PLATZ_API_TOKEN environment variables.
    2. Use "platz/config.toml" configuration file on your config directory.
    2. When running from inside a Platz deployment, mount the "platz-creds" secret to /var/run/secrets/platz.json .
"###)]
    NoConfigFound,

    #[error("OS error while trying to read config: {0:?}")]
    ConfigReadError(std::io::ErrorKind),

    #[error("Error parsing configuration file: {0:?}")]
    ConfigDeserializationError(toml::de::Error),

    #[error("Error in configuration file: {0:?}")]
    ConfigTomlError(&'static str),

    #[error("Error parsing {0} environment variable")]
    EnvVarParseError(&'static str),

    #[error("Error parsing URL from mounted credentials: {0}")]
    MountedUrlParseError(url::ParseError),

    #[error("Error parsing token expiry from mounted credentials: {0}")]
    MountedExpiryParseError(chrono::ParseError),

    #[error("Error joining URL: {0}")]
    UrlJoinError(url::ParseError),

    #[error("Error building client: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Error creating authorization header")]
    ErrorCreatingAuthHeader,

    #[error("Can't paginate request since the RequestBuilder can't be cloned")]
    CannotPaginate,

    #[error("Expected exactly one item, got none")]
    ExpectedOneGotNone,

    #[error("Expected exactly one item, got {0}")]
    ExpectedOneGotMany(usize),
}
