pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error parsing socket address: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error("error during hyper operation: {0}")]
    HyperError(#[from] hyper::Error),

    #[error("error during I/O operation: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Database server is already the master")]
    AlreadyMaster,

    #[error("{0}")]
    PgCtlError(String),
}

impl<T> Into<Result<T>> for Error {
    fn into(self) -> Result<T> {
        Err(self)
    }
}
