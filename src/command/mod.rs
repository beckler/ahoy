pub mod device;
pub mod release;
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
}
