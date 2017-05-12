extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate chrono;
use chrono::prelude::*;

extern crate bbs;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::TcpStream;

use hyper::server::{Request, Response, Server};
use hyper::status::StatusCode;

use bbs::Message;
use bbs::{SERVER_ADDR, BOT_ADDR, HTML_DATA, HTML_HEADER, HTML_FOOTER};

// Returns val from Ok(val) or sets the response to return an InternalServerError.
macro_rules! try_or_server_err {
    ($expr:expr, $res:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            *($res).status_mut() = StatusCode::InternalServerError;
            return;
        }
    })
}

fn req_handler(mut req: Request, mut res: Response) {
    match req.method {
        hyper::Get => {
            // Read the files [HTML_HEADER, HTML_DATA, HTML_FOOTER] into buf.
            // If HTML_DATA doesn't exist, it should be skipped without failure.
            // Use `try_or_server_err!(expression, res)` instead of `try!(expression)` in
            // order to return an internal server error.
            let mut buf = String::new();
            let mut file = try_or_server_err!(File::open(HTML_HEADER), res);
            try_or_server_err!(file.read_to_string(&mut buf), res);

            File::open(HTML_DATA).as_mut().map(|file| {
                file.read_to_string(&mut buf);
            });

            let mut file = try_or_server_err!(File::open(HTML_FOOTER), res);
            try_or_server_err!(file.read_to_string(&mut buf), res);

            // And return buf as the response.
            *res.status_mut() = StatusCode::Ok;
            res.send(&buf.as_bytes()).unwrap();
        },
        hyper::Post => {
            let mut buf = String::new();
            try_or_server_err!(req.read_to_string(&mut buf), res);

            let message : Message = try_or_server_err!(serde_json::from_str(&buf), res);

           println!("{:?}", message);

            let mut file = try_or_server_err!(OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(HTML_DATA), res);
            let dt: DateTime<Local> = Local::now();
                        
            try_or_server_err!(file.write_all(
                format!("<div class='user'>user {} {}</div><div class='content'> {} </div>", 
                    message.user,
                    dt.format("%Y-%m-%d %H:%M:%S").to_string(),
                    message.text).as_bytes()
                ), res);
            
            if message.user != "bot" {
               let mut tcp = TcpStream::connect(BOT_ADDR).unwrap();
               tcp.write_all(message.text.as_bytes()).unwrap(); 
            }

            *res.status_mut() = StatusCode::Ok;
        },
        _ => *res.status_mut() = StatusCode::ImATeapot,
    }
}

fn main() {
    println!("Listening on {}.", SERVER_ADDR);
    match Server::http(SERVER_ADDR) {
        Ok(server) => match server.handle(req_handler) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        },
        Err(e) => println!("{:?}", e),
    }
}
