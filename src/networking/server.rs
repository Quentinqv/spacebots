use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

pub fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                let websocket = accept(stream);
                match websocket {
                    Ok(websocket) => {
                        spawn(move || {
                            let mut websocket = websocket;
                            loop {
                                let msg = websocket.read();
                                match msg {
                                    Ok(msg) => {
                                        if msg.is_binary() || msg.is_text() {
                                            websocket.send(msg).unwrap();
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("WebSocket error: {:?}", e);
                                        break;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => eprintln!("Error accepting WebSocket connection: {:?}", e),
                }
            }
            Err(e) => eprintln!("Error accepting connection: {:?}", e),
        }
    }
}
