use reqwest::{Error as ReqwestError, Response as ReqwestResponse, StatusCode, Url};

#[derive(Clone, Debug, Serialize, PartialEq)]
struct JsonDecodeError {
    pub response_text: String,
}


#[derive(Clone, Debug, PartialEq)]
struct HttpError {
    pub status_code: StatusCode,
    pub url: Url,
    pub error: ReqwestError,
}

impl HttpError {
    pub async fn from_response(r: ReqwestResponse) -> Self {
        let mut error = r.json().await;

        match error {
            Ok(err) => {
                error = err;
            }
            Err(_) => {
                error = JsonDecodeError {
                    response_text: r.text().await.unwrap_or_else(|| String::new()),
                };
            }
        }
        Self {
            status_code: r.status(),
            url: r.url().clone(),
            error,
        }
    }
}
