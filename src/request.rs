use crate::http::Method;

const SPACE: u8 = 32;

#[derive(Debug)]
pub struct Request {
    method: Method,
    resource: String,
}

impl Request {
    pub fn from_bytes(buf: &[u8]) -> Option<Request> {
        let mut split = buf.split(|b| *b == SPACE);
        let method = split
            .next()
            .map(String::from_utf8_lossy)?
            .parse::<Method>()
            .ok()?;
        let resource = split
            .next()
            .map(|chunk| String::from_utf8(chunk.to_vec()))?
            .ok()?;
        Some(Self { method, resource })
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }
}
