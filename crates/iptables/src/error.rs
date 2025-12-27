use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BindingError {
    #[error("Directory not empty. stderr: '{stderr}' stdout: '{stdout}'")]
    DirectoryNotEmpty { stderr: String, stdout: String },
    #[error("Chain already exists. stderr: '{stderr}' stdout: '{stdout}'")]
    ChainAlreadyExists { stderr: String, stdout: String },
    #[error("Not found chain/target/match by that name. stderr: '{stderr}' stdout: '{stdout}'")]
    NotFoundByThatName { stderr: String, stdout: String },
    #[error("Unknown iptables error (IO). error: {error}")]
    UnknownIO {
        #[from]
        error: io::Error,
    },
    #[error("Unknown iptables error. stderr: '{stderr}' stdout: '{stdout}'")]
    Unknown { stderr: String, stdout: String },
}
