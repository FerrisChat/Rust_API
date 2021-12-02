use super::HttpMethod;
use reqwest::header::HeaderMap;

pub struct Request<'a> {
    pub method: Method,
    pub route: String,
    pub headers: Option<HeaderMap>,
    pub body: Option<&'a [u8]>,
}

impl<'a> Request<'a> {
    pub fn new(&mut self, method: HttpMethod, route: &str) {
        self.method = method;
        self.route = route.to_string();
        self.headers = None;
        self.body = None;
    }

    pub fn headers(&mut self, headers: Option<HeaderMap>) {
        self.headers = headers;
    }

    pub fn body(&mut self, body: Option<&'a [u8]>) {
        self.body = body;
    }
}
