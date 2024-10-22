use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener};

pub struct Server {
    listener: TcpListener,
    temp_dir: String,
}

impl Server {
    pub async fn new(port: u32, dir: String) -> std::io::Result<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        Ok(Self {
            listener,
            temp_dir: String::new(),
        })
    }

    pub fn save(&self, data: Vec<u8>) {
        let temp_file_path = Path::new(&self.temp_dir).join("temp_data.bin");
        let mut file = File::create(temp_file_path).expect("Unable to create file");
        file.write_all(&data).expect("Unable to write data");
    }

    pub async fn run(&self) {
        if let Ok((mut stream, _)) = self.listener.accept().await {
            println!("{:?}", stream);
            let mut buffer = [0; 512];
            let mut data = Vec::new();
            loop {
                match stream.read(&mut buffer).await {
                    Ok(0) => {
                        break;
                    }
                    Ok(n) => {
                        // Append the received data to the Vec
                        data.extend_from_slice(&buffer[0..n]);
                    }
                    Err(e) => {
                        eprintln!("Failed to read from stream: {}", e);
                        break;
                    }
                }
            }
            // Data available here.
            self.save(data);
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = Server::new(7878,".".into()).await?;
    // Start Listening & Wait for response
    println!("Start Server");
    server.run().await;
    println!("Processed");
    Ok(())
}
