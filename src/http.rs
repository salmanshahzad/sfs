use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    Options,
    Head,
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OPTIONS" => Ok(Method::Options),
            "HEAD" => Ok(Method::Head),
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PATCH" => Ok(Method::Patch),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Status {
    Ok,
    Found,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    RequestTimeout,
    InternalServerError,
}

impl Status {
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::Found => 302,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::RequestTimeout => 408,
            Self::InternalServerError => 500,
        }
    }

    pub fn text(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Found => "Found",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::RequestTimeout => "Request Timeout",
            Self::InternalServerError => "Internal Server Error",
        }
    }
}
