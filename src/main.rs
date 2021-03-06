use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

fn server () {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_text() {
                  match msg.into_text() {
                    Ok(res) => println!("{:?}", res),
                    Err(err) => println!("{:?}", err),
                  };
                  return;
                }
            }
        });
    }
}

fn main () {
  server();
}