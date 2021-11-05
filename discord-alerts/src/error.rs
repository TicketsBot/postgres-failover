pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error performing i/o operation: {0}")]
    IoError(#[from] std::io::Error),

    #[error("error performing http request: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("discord returned a non-ok status code: {0}")]
    DiscordError(String),
}

impl<T> Into<Result<T>> for Error {
    fn into(self) -> Result<T> {
        Err(self)
    }
}
