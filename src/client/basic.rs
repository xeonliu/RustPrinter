use std::error::Error;
use std::fmt::format;
use reqwest::{cookie, Response, Url};
use super::{consts::BASE_URL, Client};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::cookie::CookieStore;
use log::log;
use tokio::sync::oneshot;
use crate::client::model::{GetAuthTokenResponse, WaitUserInResponse};

impl Client {
    pub fn new() -> Self {
        Self::with_base_url(BASE_URL)
    }

    pub fn with_base_url(base_url: &str) -> Self {
        let jar = Arc::new(cookie::Jar::default());
        let cli = reqwest::Client::builder()
            .cookie_provider(jar.clone())
            .build()
            .unwrap();
        let base_url = Url::parse(base_url).expect("URL PARSE ERROR;");
        Self {cli, jar, base_url}
    }

    pub fn load_cookie(&self, cookie_path: &str) {
        // Load Cookie String from file
        self.jar.add_cookie_str("TODO", &self.base_url);
    }

    pub fn store_cookie(&self, cookie_path: &str) {
        if let Some(value) = self.jar.cookies(&self.base_url) {
            println!("{:?}", value);
        } else {
            println!("No Cookie in Jar")
        }
    }

    pub fn check_cookie(&self) -> bool {
        self.jar.cookies(&self.base_url).is_some()
    }

    pub async fn get_cookie(&self) -> Result<(), Box<dyn Error>>{
        self.cli.get(format!("{}/api/client/Auth/Check",self.base_url)).send().await?;
        Ok(())
    }

    async fn get_token(&self) -> Result<String, Box<dyn Error>> {
        let res = self.cli.get(format!("{}/api/client/Auth/GetAuthToken", self.base_url)).send().await?;
        println!("{:?}", res);
        let json: GetAuthTokenResponse = res.json().await.expect("AUTH JSON PARSE ERROR");
        Ok(json.sz_token)
    }

    pub(crate) async fn login(&self) -> Result<(), Box<dyn Error>>{
        if(!self.check_cookie()) {
            self.get_cookie().await?;
            self.store_cookie("path");
        }

        let token = self.get_token().await?;

        // Generate QR Code Using the token
        let login_url = format!("http://pay.unifound.net/uniwx/s.aspx?c=uniauth_1_{}", token);
        println!("login_url: {}", login_url);

        // spawn another thread for waiting.
        let (tx,rx) = oneshot::channel();

        // Construct another cli.
        let temp_cli = self.cli.clone();

        tokio::spawn(async move {
            loop {
                // Keep Making Requests
                println!("Loop");
                let unix_ms = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time Error").as_millis();
                let res = temp_cli.get(format!("{}/api/client/Auth/WaitUserIn?_={}", BASE_URL, unix_ms)).send().await;

                match res {
                    Ok(res) => {
                        let res = res.json::<WaitUserInResponse>().await.expect("Wrong String");
                        if (res.result.is_some()) {
                            tx.send("Success").unwrap();
                            return;
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        // Wait until logged in.
        println!("Pending...");

        match rx.await {
            Ok(text) => println!("{:?}",text),
            Err(e) => println!("Error")
        }

        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn test_client() {
    let client = Client::new();
    client.login().await.unwrap();
}