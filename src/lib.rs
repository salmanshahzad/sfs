//! A simple static file server that can serve files from a directory.
//!
//! # Example
//!
//! Serve files from the `public` directory on port `1024`.
//!
//! ```
//! use std::process;
//!
//! use sfs::Server;
//!
//! fn main() {
//!     let server = Server::new("public");
//!     if let Err(err) = server.and_then(|s| s.listen(1024)) {
//!         eprintln!("{err}");
//!         process::exit(1);
//!     }
//! }
//! ```
use std::{
    ffi::OsStr,
    fs::File,
    io::{ErrorKind, Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    time::Duration,
};

use crate::{
    error::{Result, SfsError},
    http::{Method, Status},
    request::Request,
    response::Response,
};

pub mod error;
mod file;
mod http;
mod request;
mod response;

/// A static file server.
pub struct Server {
    dir: PathBuf,
}

impl Server {
    /// Return a new server that will serve files from `dir`.
    /// Fails with [SfsError::InvalidDirectory] if `dir` is not a directory.
    pub fn new<S>(dir: S) -> Result<Self>
    where
        S: AsRef<OsStr>,
    {
        let path = PathBuf::from(dir.as_ref());
        if path.is_dir() {
            Ok(Self { dir: path })
        } else {
            Err(SfsError::InvalidDirectory)
        }
    }

    /// Start listening on `port`.
    /// Fails with [SfsError::IoError] if the server cannot bind to the specified `port`.
    pub fn listen(&self, port: u16) -> Result<()> {
        let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    stream.set_read_timeout(Some(Duration::from_secs(1)))?;
                    stream.set_write_timeout(Some(Duration::from_secs(1)))?;
                    if let Err(err) = self.handle_request(&mut stream) {
                        let status = match err {
                            SfsError::IoError(err) => match err.kind() {
                                ErrorKind::TimedOut => Status::RequestTimeout,
                                _ => Status::InternalServerError,
                            },
                            _ => Status::InternalServerError,
                        };
                        Server::send_status(&mut stream, status).ok();
                    }
                }
                Err(err) => eprintln!("Could not handle request: {err}"),
            }
        }

        Ok(())
    }

    fn handle_request(&self, stream: &mut TcpStream) -> Result<()> {
        let mut buf = [0; 1024];
        let _ = stream.read(&mut buf)?;

        let req = Request::from_bytes(&buf);
        match req {
            Some(req) => {
                let path = self.get_path(req.resource());
                match path {
                    Some(mut path) => {
                        if !req.resource().ends_with('/') && path.is_dir() {
                            Server::send_dir_redirect(
                                stream,
                                path.strip_prefix(&self.dir).unwrap_or(&path),
                            )?;
                        } else {
                            if path.is_dir() {
                                path = path.join("index.html");
                            }
                            match req.method() {
                                Method::Head => Server::send_file(stream, &path, false)?,
                                Method::Get => Server::send_file(stream, &path, true)?,
                                _ => Server::send_status(stream, Status::MethodNotAllowed)?,
                            }
                        }
                    }
                    None => Server::send_status(stream, Status::NotFound)?,
                }
            }
            None => Server::send_status(stream, Status::BadRequest)?,
        }

        Ok(())
    }

    fn get_path<P>(&self, path: P) -> Option<PathBuf>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let path = path.strip_prefix("/").unwrap_or(path);
        let path = self.dir.join(path);

        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    fn send_dir_redirect<P>(stream: &mut TcpStream, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut response = Response::new(Status::Found);
        response.headers_mut().insert(
            "Location".to_string(),
            format!("/{}/", path.as_ref().display()),
        );
        stream.write_all(&response.as_bytes())?;
        Ok(())
    }

    fn send_file<P>(stream: &mut TcpStream, path: P, send_body: bool) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut response = Response::new(Status::Ok);
        let mut file = File::open(path)?;
        let content_length = file.metadata()?.len() as usize;
        response
            .headers_mut()
            .insert("Content-Length".to_string(), content_length.to_string());
        response.headers_mut().insert(
            "Content-Type".to_string(),
            file::get_content_type(path).to_string(),
        );

        if send_body {
            let mut body = Vec::with_capacity(content_length);
            file.read_to_end(&mut body)?;
            response.set_body(Some(body));
        }

        stream.write_all(&response.as_bytes())?;
        Ok(())
    }

    fn send_status(stream: &mut TcpStream, status: Status) -> Result<()> {
        let response = Response::new(status);
        stream.write_all(&response.as_bytes())?;
        Ok(())
    }
}
