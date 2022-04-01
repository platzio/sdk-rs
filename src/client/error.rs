#[derive(Debug, thiserror::Error)]
pub enum PlatzClientError {
    #[error("Could not find any Platz config")]
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
    ClientBuildError(reqwest::Error),

    #[error("Error creating authorization header")]
    ErrorCreatingAuthHeader,
}
