use reqwest::header::HeaderMap;
use reqwest::Method;

struct Request {
    method: Method,
    route: String,
    headers: Optional<HeaderMap>,
    body: Option<&'a [u8]>,
}

impl<'a> Request<'a> {
    pub fn new(&mut self, method: Method, route: &str) {
        self.method = method;
        self.route = route.to_string();
        self.headers = None;
        self.body = None;
    }

    pub fn headers(&mut self, headers: Optional<HeaderMap>) {
        self.headers = headers;
    }

    pub fn body(&mut self, body: Option<&'a [u8]>) {
        self.body = body;
    }
}
