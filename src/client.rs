use std::thread;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::io::stdin;
use websocket::{Message, OwnedMessage};
use websocket::client::ClientBuilder;
use websocket::receiver;
use websocket::sender;

pub fn attach_browser(ws: &str) {
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
