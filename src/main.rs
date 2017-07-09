extern crate futures;
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

use futures::{Future, Stream};
use reqwest::Client;
use serde_json::Value;
use std::io::{self, Read, Write};
use tokio_core::reactor::Core;

#[derive(Debug, Deserialize)]
struct Something {
    description: String,
    devtoolsFrontendUrl: Option<String>,
    faviconUrl: Option<String>,
    id: String,
    title: String,
    #[serde(rename = "type")]
    some_type: String,
    url: String,
    webSocketDebuggerUrl: Option<String>,
}

fn main() {
    println!("Welcome to the Flight Data Recorder!");
    let mut resp = reqwest::get("http://localhost:9222/json").unwrap();
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
    let json: Vec<Something> = serde_json::from_str(&content).unwrap();
    println!("Got: {:?}", json);
}
