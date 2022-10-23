use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:8000";

#[tokio::main]
async fn main() {
    println!("connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        println!(
            "connected to echo server {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        let message = "Hello World";
        let _ = stream.write_all(message.as_bytes()).await;
        println!("sent: {}", message);

        let mut buffer = [0; 1024];
        let _len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {}", message);
    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS)
    }
}
