# Rust ESP-IDF Websocket Example

Websocket is an experimental feature of the `esp-idf-svc` crate but I cant find anything about it in they repo so I made this example. It's much a like with the [mqtt example](https://github.com/ivmarkov/rust-esp32-std-demo). This project was bootstrap with `cargo-generate` using the [esp-idf-template](https://github.com/esp-rs/esp-idf-template). Im using the esp32c3 rust board.

## Websocket Server

A simple websocket server example to test:

``` rust
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

fn main() {
    let server = TcpListener::bind("YOUR_PC_IP:9001").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let the_client = stream.unwrap();
            println!("Client: {}", the_client.local_addr().unwrap());
            let mut websocket = accept(the_client).unwrap();

            websocket
                .write_message(tungstenite::Message::Text("Hello client".to_string()))
                .unwrap();

            loop {
                if websocket.can_read() {
                    let msg = websocket.read_message().unwrap();

                    if msg.is_binary() || msg.is_text() {
                        println!("Message received: {}", msg.clone().into_text().unwrap());
                    }
                }
            }
        });
    }
}
```