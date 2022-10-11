pub mod device;
pub mod github;
pub mod update;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum CommandError {
    #[error("unable to retrieve file: {0:?}")]
    IO(String),
    #[error("unable to perform install: {0:?}")]
    Dfu(String),
    #[error("unable to send command to device: {0:?}")]
    Device(String),
    #[error("unable to fetch releases: {0:?}")]
    Retieval(String),
    #[error("Failed to make a request: {0:?}")]
    Http(String),
    #[error("unable to update: {0:?}")]
    Update(String),
}

impl From<surf::Error> for CommandError {
    fn from(err: surf::Error) -> Self {
        CommandError::Http(err.to_string())
    }
}
