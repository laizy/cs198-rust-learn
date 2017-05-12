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

macro_rules! try_or_server_err {
    ($expr:expr, $stream:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            $stream.shutdown(Shutdown::Both);
            return;
        }
    })
}


fn main() {
    // Create a bot user.
    // TODO
    let client = UserClient::new("bot".into(), HTML_ADDR.into());

    // Start TcpListener.
    let listener = TcpListener::bind(BOT_ADDR).unwrap();

    let handle_client = |mut stream: TcpStream| {
        print!("ahhf");
        let mut buf = String::new();
        try_or_server_err!(stream.read_to_string(&mut buf), stream);
        let words: Vec<_> = buf.trim().split_whitespace().collect();
        if words.len() > 0 && words[0] == "choose" {
            let n = 1;
            client.send_msg(words[1]);
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => println!("{}", e.description()),        
        }

    }


    // Listen for incoming TCP connections.
    // For each connection, accept it and read all data from the stream.
    // If it's a 'choose' message, post to the BBS with a response (via the above bot user).
    // TODO
}
