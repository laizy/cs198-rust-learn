extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate bbs;

use std::env::args;

use bbs::{UserClient, HTML_ADDR};


fn main() {
    let user = String::from("user1");
    let client = UserClient::new(user, HTML_ADDR.into());
    let (status, text) = client.get_content().unwrap();

    let text = String::from("teahfahhff");
    println!("{}", text);

    for i in 0..100 {
        client.send_msg(&format!("{}:{}", i, text));
    }

}
