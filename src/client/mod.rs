mod base;
mod config;
mod error;
mod request;

pub use base::PlatzClient;
pub use error::PlatzClientError;
pub(crate) use request::Paginated;
pub use request::PlatzRequest;
