use super::WebsocketStream;
use super::error::WebsocketError;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use futures_util::{SinkExt, StreamExt};

use serde_json::{json, Value};

use url::Url;

use async_trait::async_trait;

use crate::internal::prelude::*;

pub struct Websocket {
    pub stream: WebsocketStream,
    pub url: Url,
    pub token: String,
}

impl Websocket {
    pub async fn new(url: String, token: String) -> Result<Websocket> {
        let parsed_url = Url::parse(&url).map_err(|e| panic!("Failed to parse url: {}", e))?;

        let stream = create_ws_stream(parsed_url).await?;

        Ok(Websocket { stream, url, token })
    }

    pub async fn connect(&self) -> Result<()> {
        self.identify().await?;

        while let Some(message) = self.stream.next().await {
            let message = message?;

            match message {
                Message::Text(payload) => {
                    tokio::task::spawn(self.handle_event(serde_json::from_str(&payload)?));
                }
                Message::Close(Some(frame)) => {
                    return Err(Error::Websocket(WebsocketError(Some(frame))));
                }
                _ => (),
            }
        }
    }

    pub async fn identify(&self) -> Result<()> {
        let payload = json!({
            "c": "Identify",
            "d": {
                "token": self.token.clone(),
                "intents": 0 as u8
            }
        });

        self.stream.send(payload).await?;

        Ok(())
    }

    pub async fn handle_event(&mut self, event: Value) -> Result<()> {
        Ok(())
    }
}

async fn create_ws_stream(url: Url) -> Result<WebsocketStream> {
    let (stream, _) = connect_async(url).await?;

    Ok(stream)
}
