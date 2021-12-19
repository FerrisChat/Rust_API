use std::{error::Error as StdError, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;

#[cfg(feature = "websocket")]
use websocket::WebsocketError;
use tungstenite::error::Error as TungsteniteError;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
pub enum Error {
    /// An error from the `tungstenite` crate.
    #[cfg(feature = "websocket")]
    Tungstenite(TungsteniteError),
    SerdeJson(SerdeJsonError),
    #[cfg(feature = "websocket")]
    Websocket(WebsocketError),
}

#[cfg(feature = "websocket")]
impl From<TungsteniteError> for Error {
    fn from(e: TungsteniteError) -> Error {
        Error::Tungstenite(e)
    }
}

impl From<SerdeJsonError> for Error {
    fn from(e: SerdeJsonError) -> Error {
        Error::SerdeJson(e)
    }
}

#[cfg(feature = "websocket")]
impl From<WebsocketError> for Error {
    fn from(e: WebsocketError) -> Error {
        Error::Websocket(e)
    }
}
