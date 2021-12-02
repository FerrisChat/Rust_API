use std::sync::Arc;

use std::result::Result;

use super::request::Request;
// use dashmap::DashMap;

use reqwest::{header, Client, ClientBuilder, Response as ReqwestResponse};

use super::HttpError;

pub struct HttpClient {
    pub client: Arc<Client>,
    pub token: String,
    pub base_url: String,
    // pub routes_ratelimit: Arc<DashMap<String, Mutex>>,
}

use tokio::sync::Mutex;

impl HttpClient {
    pub fn new(token: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(token).unwrap(),
        ); // It shouldn't be Err but if it is for whatever reasson, it panic.

        let client_builder = configure_tls(ClientBuilder::new())
            .user_agent(format!("ferrischat.rs/{}", env!("CARGO_PKG_VERSION")))
            .default_headers(headers);

        let built_client = client_builder.build().expect("Failed to build http client");

        HttpClient {
            client: Arc::new(built_client),
            token: token.to_string(),
            base_url: "https://api.ferris.chat/v0".to_string(),
            // routes_ratelimit: Arc<DashMap::new()>,
        }

        // add_locks(&http);
    }

    pub async fn request(&self, request: Request<'_>) -> Result<ReqwestResponse, HttpError> {
        let request_builder = self.client.request(
            request.method.to_reqwest_method(),
            format!("{}/{}", self.base_url, request.route),
        );

        if let Some(body) = request.body {
            request_builder.json(body);
        }

        if let Some(headers) = request.headers {
            &mut request_builder.headers(headers);
        }

        let response = request_builder.send().await;

        if let Ok(resp) = response {
            if resp.status().is_success() {
                return Ok(resp);
            } else {
                Err(HttpError::from_response(resp).await)
            }
        }

        let response_error = response
            .err()
            .unwrap_or_else(|| unreachable!("Unable to unwrap error from response"));

        Err(HttpError {
            status_code: response_error.status(),
            url: response_error.url().clone(),
            error: "Something went wrong when to request".to_string(),
        })
    }
}

// fn add_locks(&http) -> () {
//     let routes = vec![
//         "GET /auth",
//         "/guilds"
//         "/"
//     ]
// }

#[cfg(feature = "rustls_tls")]
fn configure_tls(builder: ClientBuilder) -> ClientBuilder {
    builder.use_rustls_tls()
}

#[cfg(feature = "native_tls")]
fn configure_tls(builder: ClientBuilder) -> ClientBuilder {
    builder.use_native_tls()
}
