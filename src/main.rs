mod client;

use crate::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    client.load_cookie("./test.txt");
    if !client.check_login().await.unwrap() {
        println!("Not Logged in");
        client.login().await.unwrap();
    }
    println!("Logged in");
    client.store_cookie("./test.txt");
}
