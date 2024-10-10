mod client;
mod spooler;

use crate::client::Client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} /JOBID:[JOBID] /PRINTER:[PRINTER_NAME]", args[0]);
        return;
    }

    // Check if Windows Usage
    // let job_id: i32 = args[1].split_off(7).parse();

    let client = Client::new();
    client.load_cookie("./test.txt");
    if !client.check_login().await.unwrap() {
        println!("Not Logged in");
        client.login().await.unwrap();
    }

    println!("Logged in");
    let id = client.create_job().await.unwrap();
    println!("jobid: {}", id);

    // Load from Tmp File.
    client.upload_job(id, "./test.txt").await.unwrap();
    client.store_cookie("./test.txt");
}
