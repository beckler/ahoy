pub mod device;
pub mod github;
pub mod update;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CommandError {
    #[error("unable to retrieve file")]
    IO(String),
    #[error("unable to perform install")]
    Dfu(String),
    #[error("unable to send command to device")]
    Device(String),
    #[error("unable to fetch releases")]
    Retieval(String),
    #[error("Failed to make a request: {0:?}")]
    Http(String),
}

impl From<surf::Error> for CommandError {
    fn from(err: surf::Error) -> Self {
        CommandError::Http(err.to_string())
    }
}
