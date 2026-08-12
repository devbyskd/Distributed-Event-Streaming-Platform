use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, error, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await?;
    info!("Event Stream Broker listening on: {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process_client(socket).await;
        });
    }
}

async fn process_client(mut socket: TcpStream) {
    let mut buf = vec![0; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                info!("Client connection closed.");
                return;
            }
            Ok(n) => {
                info!("Received {} bytes. Appending to distributed log...", n);
                // In production, persist to disk/Raft consensus log here
                if let Err(e) = socket.write_all(b"ACK\n").await {
                    error!("Failed to write to socket; err = {:?}", e);
                    return;
                }
            }
            Err(e) => {
                error!("Failed to read from socket; err = {:?}", e);
                return;
            }
        }
    }
}
