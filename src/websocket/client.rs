use super::error::WebsocketError;
use super::WebsocketStream;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use futures_util::{SinkExt, StreamExt};

use serde_json::{json, Value};

use url::Url;

use crate::internal::prelude::*;

pub struct Websocket {
    pub stream: WebsocketStream,
    pub url: Url,
    pub token: String,
}

impl Websocket {
    pub async fn new(url: &str, token: String) -> Result<Websocket> {
        let url = Url::parse(url).unwrap_or_else(|e| panic!("Failed to parse url: {}", e));

        let stream = create_ws_stream(url.clone()).await?;

        Ok(Websocket { stream, url, token })
    }

    pub async fn connect(&mut self) -> Result<()> {
        self.identify().await?;

        Ok(())
    }

    pub async fn receive_json(&mut self) -> Result<Option<Value>> {
        let message = self.stream.next().await;

        let payload: Value = match message {
            Some(Ok(Message::Text(payload))) => serde_json::from_str(&payload)?,
            Some(Ok(Message::Close(Some(frame)))) => {
                return Err(Error::Websocket(WebsocketError::Closed(Some(frame))))
            }
            _ => return Ok(None),
        };

        Ok(Some(payload))
    }

    pub async fn send_json(&mut self, payload: &Value) -> Result<()> {
        Ok(serde_json::to_string(payload)
            .map(Message::Text)
            .map_err(Error::from)
            .map(|m| self.stream.send(m))?
            .await?)
    }

    pub async fn identify(&mut self) -> Result<()> {
        let payload = json!({
            "c": "Identify",
            "d": {
                "token": self.token.clone(),
                "intents": 0_u8
            }
        });

        self.send_json(&payload).await?;

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
