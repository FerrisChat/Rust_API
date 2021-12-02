use reqwest::{Response as ReqwestResponse, StatusCode, Url};

#[derive(Clone, Debug, PartialEq)]
pub struct HttpError {
    pub status_code: Option<StatusCode>,
    pub url: Option<Url>,
    pub error: String,
}

impl HttpError {
    pub async fn from_response(r: ReqwestResponse) -> Self {
        HttpError {
            status_code: Some(r.status()),
            url: Some(r.url().clone()),
            error: r.text().await.unwrap_or_else(|_| String::new()),
        }
    }
}
