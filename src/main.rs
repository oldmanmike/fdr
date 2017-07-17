// Copyright (c) 2017 Michael Carpenter
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate clap;
extern crate fdr;
extern crate futures;
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate websocket;

use clap::{Arg, App};
use fdr::CDPTarget;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::io::{Read, stdin};
use websocket::{Message, OwnedMessage};
use websocket::client::ClientBuilder;
use websocket::receiver;
use websocket::sender;

fn main() {
    let matches = App::new("Flight Data Recorder")
        .version("0.0.1")
        .author("Michael Carpenter <mcarpenter.dev@gmail.com>")
        .about("A Swiss Army knife for frontend exploratory testing")
        .arg(Arg::with_name("d").short("d").help("Run as a daemon"))
        .arg(Arg::with_name("verbose").long("--verbose").short("v").help("Log everything"))
        .arg(Arg::with_name("silent").long("--silent").short("s").help("Log nothing"))
        .arg(Arg::with_name("browser")
                 .long("--browser")
                 .short("b")
                 .multiple(true)
                 .help("The address and port of the desired browser"))
        .get_matches();

    println!("Welcome to the Flight Data Recorder!");
    let mut resp = reqwest::get("http://localhost:9222/json").unwrap();
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
    let mut targets: Vec<CDPTarget> = serde_json::from_str(&content).unwrap();
    println!("Got: {:?}", targets);

    let target = targets.remove(0);
    println!("Selecting Target {:?}", target);

    let ws = target.web_socket_debugger_url;
    println!("Connecting to {:?}", ws);

    let client = ClientBuilder::new(ws.unwrap().as_str())
        .unwrap()
        .add_protocol("rust-websocket")
        .connect_insecure()
        .unwrap();

    println!("Successfully connected");

    let (mut receiver, mut sender): (receiver::Reader<TcpStream>, sender::Writer<TcpStream>) =
        client.split().unwrap();

    let (tx, rx): (mpsc::Sender<OwnedMessage>, mpsc::Receiver<OwnedMessage>) = channel();

    let tx_1 = tx.clone();

    let send_loop = thread::spawn(move || run_send_loop(sender, rx));
    let receive_loop = thread::spawn(move || run_receive_loop(receiver, tx_1));

    tx.send(OwnedMessage::Text("{\"id\": 4, \"method\": \"DOM.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 17, \"method\": \"Runtime.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 15, \"method\": \"Page.enable\"}".to_string()));

    tx.send(OwnedMessage::Text("{\"id\": -2147483647, \"method\": \"Animation.enable\"}"
                                   .to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 1, \"method\": \"ApplicationCache.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 2, \"method\": \"Console.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 3, \"method\": \"CSS.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 5, \"method\": \"DOMStorage.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 6, \"method\": \"Database.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 7, \"method\": \"Debugger.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 8, \"method\": \"HeapProfiler.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 9, \"method\": \"IndexedDB.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 10, \"method\": \"Inspector.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 11, \"method\": \"LayerTree.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 12, \"method\": \"Log.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 13, \"method\": \"Network.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 16, \"method\": \"Profiler.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 18, \"method\": \"Security.enable\"}".to_string()));
    tx.send(OwnedMessage::Text("{\"id\": 19, \"method\": \"ServiceWorker.enable\"}".to_string()));

    let mut counter: i32 = 0;

    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        let message = match trimmed {
            "/close" => {
                let _ = tx.send(OwnedMessage::Close(None));
                break;
            }
            "/ping" => OwnedMessage::Ping(b"PING".to_vec()),
            _ => OwnedMessage::Text(trimmed.to_string()),
        };

        match tx.send(message) {
            Ok(()) => (),
            Err(e) => {
                println!("Main Loop: {:?}", e);
                break;
            }
        }
        counter = counter + 1;
    }

    println!("Waiting for child threads to exit");

    let _ = send_loop.join();
    let _ = receive_loop.join();

    println!("Exited");
}

fn run_send_loop(mut sender: sender::Writer<TcpStream>, rx: mpsc::Receiver<OwnedMessage>) {
    loop {
        let message: OwnedMessage = match rx.recv() {
            Ok(m) => m,
            Err(e) => {
                println!("Send Loop: {:?}", e);
                return;
            }
        };
        match message {
            OwnedMessage::Close(_) => {
                let _ = sender.send_message(&message);
                return;
            }
            _ => (),
        };
        match sender.send_message(&message) {
            Ok(()) => (),
            Err(e) => {
                println!("Send Loop: {:?}", e);
                let _ = sender.send_message(&Message::close());
                return;
            }
        }
    }
}

fn run_receive_loop(mut receiver: receiver::Reader<TcpStream>, tx: mpsc::Sender<OwnedMessage>) {
    loop {
        for message in receiver.incoming_messages() {
            let message = match message {
                Ok(m) => m,
                Err(e) => {
                    println!("Receive Loop: {:?}", e);
                    let _ = tx.send(OwnedMessage::Close(None));
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    let _ = tx.send(OwnedMessage::Close(None));
                    return;
                }
                OwnedMessage::Ping(data) => {
                    match tx.send(OwnedMessage::Pong(data)) {
                        Ok(()) => (),
                        Err(e) => {
                            println!("Receive Loop: {:?}", e);
                            return;
                        }
                    }
                }
                _ => println!("Receive Loop: {:?}", message),
            }
        }
    }
}
