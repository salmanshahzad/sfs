use std::collections::HashMap;

use crate::http::Status;

pub struct Response {
    body: Option<Vec<u8>>,
    headers: HashMap<String, String>,
    status: Status,
}

impl Response {
    pub fn new(status: Status) -> Self {
        Self {
            body: None,
            headers: HashMap::new(),
            status,
        }
    }

    pub fn set_body(&mut self, body: Option<Vec<u8>>) {
        self.body = body;
    }

    pub fn headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let content_length = self
            .body
            .as_ref()
            .map(|body| body.len())
            .unwrap_or_default();
        let mut response = Vec::with_capacity(content_length + 50);

        response.extend_from_slice(
            format!("HTTP/1.1 {} {}\n", self.status.code(), self.status.text()).as_bytes(),
        );

        for (key, value) in self.headers.iter() {
            response.extend_from_slice(format!("{key}: {value}\n").as_bytes());
        }

        if let Some(body) = self.body.as_ref() {
            response.extend_from_slice(b"\n");
            response.extend_from_slice(body);
        }

        response
    }
}
