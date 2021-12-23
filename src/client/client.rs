use std::sync::Arc;

use crate::http::HttpClient;
use crate::websocket::Websocket;

use crate::internal::prelude::*;

struct Client {
    pub token: Arc<String>,
    pub http: Arc<HttpClient>,
    pub websocket: Option<Websocket>,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Client {
            token: Arc::new(token.to_string()),
            http: Arc::new(HttpClient::new(token)),
            websocket: None,
        }
    }
}
