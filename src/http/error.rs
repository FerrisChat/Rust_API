use reqwest::{Response as ReqwestResponse, StatusCode, Url};

#[derive(Clone, Debug, PartialEq)]
pub struct HttpError {
    pub status_code: StatusCode,
    pub url: Url,
    pub error: String,
}

impl HttpError {
    pub async fn from_response(r: ReqwestResponse) -> Self {

        Self {
            status_code: r.status(),
            url: r.url().clone(),
            error: r.text().await.unwrap_or_else(|| String::new()),
        }
    }
}
