# Static File Server

An extremely simple static file server written in Rust with zero dependencies.

## Install

```
cargo install --git https://github.com/salmanshahzad/sfs
```

## Usage

### As application

```
sfs -d <directory> -p <port>
```

### As library

```rust
use std::process;

use sfs::Server;

fn main() {
    let server = Server::new("public");
    if let Err(err) = server.and_then(|s| s.listen(1024)) {
        eprintln!("{err}");
        process::exit(1);
    }
}
```
