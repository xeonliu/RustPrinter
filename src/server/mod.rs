use nom::bytes;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct Server {
    listener: TcpListener,
    temp_dir: String,
}

impl Server {
    pub async fn new(port: u32, dir: &str) -> std::io::Result<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        Ok(Self {
            listener,
            temp_dir: dir.to_string(),
        })
    }

    pub async fn run(&self) {
        if let Ok((mut stream, _)) = self.listener.accept().await {
            println!("{:?}", stream);
            let temp_file_path = Path::new(&self.temp_dir).join("temp.bin");
            let mut file = File::create(temp_file_path).expect("Unable to create file");

            let mut buffer = [0; 512];
            let mut data = Vec::new();
            let mut bytes_received = 0;
            loop {
                match stream.read(&mut buffer).await {
                    Ok(0) => {
                        break;
                    }
                    Ok(n) => {
                        file.write(&buffer[0..n]).expect("Failed to write to file");
                        // Append the received data to the Vec
                        data.extend_from_slice(&buffer[0..n]);

                        // 保留 Vec 中的最新 10 个字节
                        if data.len() > 10 {
                            let excess = data.len() - 10;
                            data.drain(0..excess);
                        }

                        bytes_received += n;
                        println!("Received Bytes: {}", bytes_received);

                        // %-12345X
                        if data.ends_with(b"%-12345X") {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from stream: {}", e);
                        break;
                    }
                }
            }
            // Data available here.
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = Server::new(7878, ".".into()).await?;
    // Start Listening & Wait for response
    println!("Start Server");
    server.run().await;
    println!("Processed");
    Ok(())
}
