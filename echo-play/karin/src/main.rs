use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

const KARIN_SERVER_ADDRESS: &str = "127.0.0.1:8001";
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    println!("karin starting {}", KARIN_SERVER_ADDRESS);

    let listener = TcpListener::bind(KARIN_SERVER_ADDRESS).unwrap();

    println!("karin listening {}", KARIN_SERVER_ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("received: {}", message);

    // thread::sleep(Duration::from_millis(delay));

    let _ = stream.write_all(message.as_bytes());
    println!("sent: {}", message);
}
