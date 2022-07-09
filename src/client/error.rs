#[derive(Debug, thiserror::Error)]
pub enum PlatzClientError {
    #[error("Could not find any Platz config. Please set the PLATZ_URL and PLATZ_API_TOKEN environment variables. When running from inside a Platz deployment, mount the \"platz-creds\" secret to /var/run/secrets/platz.json .")]
    NoConfigFound,

    #[error("OS error while trying to read config: {0:?}")]
    ConfigReadError(std::io::ErrorKind),

    #[error("Error parsing {0} environment variable")]
    EnvVarParseError(&'static str),

    #[error("Error parsing mounted secret: {0}")]
    ConfigParseError(serde_json::Error),

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
