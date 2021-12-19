use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub type WebsocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub mod client;
pub mod error;

pub use client::*;
pub use error::*;
