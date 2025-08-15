use std::net::TcpStream;
use tungstenite::{ServerHandshake, handshake::server::NoCallback};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Missing initial length specifier")]
    MissingInitialLength,

    #[error("Unexpected element found")]
    UnexpectedElement,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("{0}")]
    Tungstenite(String),

    #[error(transparent)]
    Misc(#[from] ErrorKind),
}

impl From<tungstenite::HandshakeError<ServerHandshake<TcpStream, NoCallback>>> for Error {
    fn from(value: tungstenite::HandshakeError<ServerHandshake<TcpStream, NoCallback>>) -> Self {
        Self::Tungstenite(value.to_string())
    }
}

impl From<tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Self {
        Self::Tungstenite(value.to_string())
    }
}
