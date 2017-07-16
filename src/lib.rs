extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate websocket;

use serde::{Serialize, Deserialize, Deserializer};
use std::io;
use std::sync::mpsc::{Sender, Receiver};
use websocket::{Message, OwnedMessage};

pub mod remote_debugger;

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
    fn packetize<'a, S: Serialize, R: Deserializer<'a>>(self,
                                                        method: String,
                                                        params: S)
                                                        -> io::Result<R> {
        let payload = serde_json::to_string(&params).unwrap();
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
