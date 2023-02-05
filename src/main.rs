use std::{env, process};

use sfs::Server;

fn main() {
    let mut dir = String::from(".");
    let mut port = String::from("1024");
    let mut print_help = false;
    let mut print_version = false;

    let mut args_iter = env::args().peekable();
    while let Some(arg) = args_iter.next() {
        if arg == "-h" || arg == "--help" {
            print_help = true;
        }
        if arg == "-V" || arg == "--version" {
            print_version = true;
        }
        if arg == "-d" || arg == "--directory" {
            if let Some(d) = args_iter.next() {
                dir = d;
            }
        }
        if arg == "-p" || arg == "--port" {
            if let Some(p) = args_iter.next() {
                port = p;
            }
        }
    }

    if print_help {
        print_usage();
    } else if print_version {
        println!("0.1.0");
    } else {
        let port = match port.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                eprintln!("Invalid port");
                process::exit(1);
            }
        };

        let server = Server::new(dir);
        if let Err(err) = server.and_then(|s| s.listen(port)) {
            eprintln!("{err}");
            process::exit(1);
        }
    }
}

fn print_usage() {
    let output = r#"
sfs 0.1.0
Salman Shahzad
A simple static file server.

USAGE:
    sfs [FLAGS] [OPTIONS]

FLAGS:
    -h, --help
        Prints help information

    -V, --version
        Prints version information

OPTIONS:
    -d, --directory <path>
        The directory to serve
        Default: .

    -p, --port <port>
        The port on which the server should listen
        Default: 1024
"#;
    println!("{}", output.trim());
}
