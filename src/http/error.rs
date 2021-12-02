use reqwest::{Response as ReqwestResponse, StatusCode, Url};

#[derive(Clone, Debug, PartialEq)]
pub struct HttpError {
    pub status_code: Optional<StatusCode>,
    pub url: Optional<Url>,
    pub error: String,
}

impl HttpError {
    pub async fn from_response(r: ReqwestResponse) -> Self {
        HttpError {
            status_code: Some(r.status()),
            url: r.url().clone(),
            error: r.text().await.unwrap_or_else(|| String::new()),
        }
    }
}
