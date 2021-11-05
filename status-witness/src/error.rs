pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error parsing socket address: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error("error during hyper operation: {0}")]
    HyperError(#[from] hyper::Error),
}
