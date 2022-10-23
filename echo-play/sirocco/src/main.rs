use std::{
    env::args,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    let delay = args()
        .nth(1)
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or_default();
    println!("{}", delay);

    println!("sirocco starting {}", SIROCCO_SERVER_ADDRESS);

    let listener = TcpListener::bind(SIROCCO_SERVER_ADDRESS).unwrap();

    println!("sirocco listening {}", SIROCCO_SERVER_ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, delay);
    }
}

fn handle_connection(mut stream: TcpStream, delay: u64) {
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("received: {}", message);

    thread::sleep(Duration::from_millis(delay));

    let _ = stream.write_all(message.as_bytes());
    println!("sent: {}", message);
}
