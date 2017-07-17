// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate tokio_io;
extern crate websocket;

use serde::{Serialize, Deserialize, Deserializer};
use std::io;
use std::sync::mpsc::{Sender, Receiver};
use websocket::{Message, OwnedMessage};

pub mod remote_debugger;
pub mod server;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDPTarget {
    pub description: String,
    pub devtools_frontend_url: Option<String>,
    pub favicon_url: Option<String>,
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub some_type: String,
    pub url: String,
    pub web_socket_debugger_url: Option<String>,
}

struct ChromeDriver {
    counter: i32,
    tx: Sender<OwnedMessage>,
    rx: Receiver<OwnedMessage>,
}

impl ChromeDriver {
    fn packetize<'a, S, D>(self, method: String, params: S) -> io::Result<D>
        where S: Serialize,
              D: Deserializer<'a>
    {
        let payload = serde_json::to_string(&params).unwrap();
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
