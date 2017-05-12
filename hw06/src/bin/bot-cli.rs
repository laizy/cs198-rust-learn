extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate bbs;

use std::io::Read;
use std::net::{TcpListener, Shutdown, TcpStream};

use std::error::Error;

use hyper::Client;
use hyper::header::UserAgent;

use bbs::{BOT_ADDR, HTML_ADDR};
use bbs::UserClient;

use std::io::Write;

fn main() {
    let client = hyper::client::Client::new();
    let url = hyper::Url::parse_with_params(
        "https://www.random.org/integers/",
        &[("num", "1"), ("min", "1"), ("max", "10"),
         ("format", "plain"), ("rnd", "new"), ("col", "1"), ("base", "10"),
         ]).expect("url error");
    println!("{}", url);
    let mut res = client.get(url).send().expect("request error");
    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("read error");
    println!("{}", buf);
}
