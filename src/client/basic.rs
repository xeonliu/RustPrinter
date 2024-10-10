use super::model::{CreateJobRequest, SetJobRequest};
use super::{consts::BASE_URL, Client};
use crate::client::model::{
    CreateJobResponse, GetAuthTokenResponse, StatusCodeResponse, WaitUserInResponse,
};
use reqwest::cookie::CookieStore;
use reqwest::{cookie, multipart, Url};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::oneshot;

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
        Self { cli, jar, base_url }
    }

    pub fn load_cookie(&self, cookie_path: &str) {
        // Load Cookie String from file
        if let Ok(mut file) = File::open(cookie_path) {
            let mut cookie_string = String::new();
            file.read_to_string(&mut cookie_string)
                .expect("Unable to read data");
            self.jar.add_cookie_str(&cookie_string, &self.base_url);
        }

        File::create(cookie_path).expect("Unable to create file");
    }

    pub fn store_cookie(&self, cookie_path: &str) {
        if let Some(cookie) = self.jar.cookies(&self.base_url) {
            let mut file = File::create(cookie_path).expect("Unable to open file");
            file.write_all(cookie.as_bytes())
                .expect("Unable to write data");
            println!("{:?}", cookie);
        } else {
            println!("No Cookie in Jar")
        }
    }

    pub fn check_cookie(&self) -> bool {
        self.jar.cookies(&self.base_url).is_some()
    }

    pub async fn get_cookie(&self) -> Result<(), Box<dyn Error>> {
        self.cli
            .get(format!("{}/api/client/Auth/Check", self.base_url))
            .send()
            .await?;
        Ok(())
    }

    pub async fn check_login(&self) -> Result<bool, Box<dyn Error>> {
        let res = self
            .cli
            .get(format!("{}/api/client/Auth/Check", self.base_url))
            .send()
            .await?;
        let res: StatusCodeResponse = res.json().await?;
        Ok(res.code == 0)
    }

    async fn get_token(&self) -> Result<String, Box<dyn Error>> {
        let res = self
            .cli
            .get(format!("{}/api/client/Auth/GetAuthToken", self.base_url))
            .send()
            .await?;
        println!("{:?}", res);
        let json: GetAuthTokenResponse = res.json().await.expect("AUTH JSON PARSE ERROR");
        Ok(json.sz_token)
    }

    pub(crate) async fn login(&self) -> Result<(), Box<dyn Error>> {
        if !self.check_cookie() {
            self.get_cookie().await?;
            self.store_cookie("./test.txt");
        }

        let token = self.get_token().await?;

        // Generate QR Code Using the token
        let login_url = format!("http://pay.unifound.net/uniwx/s.aspx?c=uniauth_1_{}", token);
        println!("login_url: {}", login_url);

        // spawn another thread for waiting.
        let (tx, rx) = oneshot::channel();

        // Construct another cli.
        let temp_cli = self.cli.clone();

        tokio::spawn(async move {
            loop {
                // Keep Making Requests
                println!("Loop");
                let unix_ms = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time Error")
                    .as_millis();
                let res = temp_cli
                    .get(format!(
                        "{}/api/client/Auth/WaitUserIn?_={}",
                        BASE_URL, unix_ms
                    ))
                    .send()
                    .await;

                match res {
                    Ok(res) => {
                        let res = res
                            .json::<WaitUserInResponse>()
                            .await
                            .expect("Wrong String");
                        if res.result.is_some() {
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
            Ok(text) => println!("{:?}", text),
            Err(e) => println!("{:?}", e),
        }

        Ok(())
    }

    pub async fn create_job(&self) -> Result<usize, Box<dyn Error>> {
        // Create Job Data.

        let req_json = CreateJobRequest {
            dw_property: 0,
            sz_job_name: "test".into(),
            dw_copies: 1,
            sz_attribe: "".into(),
            sz_paper_detail:
                "[{\"dwPaperID\":9,\"dwBWPages\":1,\"dwColorPages\":0,\"dwPaperNum\":1}]".into(),
            sz_color_map: "0".into(),
        };

        let res = self
            .cli
            .post(format!("{}/api/client/PrintJob/Create", BASE_URL))
            .json(&req_json)
            .send()
            .await?;

        let res: CreateJobResponse = res.json().await?;
        println!("{:?}", res);

        Ok(res.result.dw_job_id)
    }

    /// Upload Compressed PCL File to Server.
    ///
    /// `filepath` indicates the path of gzipped PCL File
    ///
    pub async fn upload_job(&self, dw_jobid: usize, filepath: &str) -> Result<(), Box<dyn Error>> {
        let part = multipart::Part::file(filepath).await?.file_name("raw.dat");
        let form = multipart::Form::new().part("szFile", part);

        let res = self
            .cli
            .post(format!(
                "{}/api/client/PrintJob/Upload?dwJobId={}",
                BASE_URL, dw_jobid
            ))
            .multipart(form)
            .header("User-Agent", "UPMClient 1.0")
            .send()
            .await?;

        let res = res.json::<StatusCodeResponse>().await?;
        println!("{:?}", res);
        match res.code {
            0 => Ok(()),
            _ => Err("Upload job failed".into()),
        }
    }

    /// Upload PVG Preview File to Server.
    ///
    /// The PVG File consists of PNG Files for each Page.
    #[allow(unused)]
    pub async fn upload_preview(
        &self,
        dw_jobid: usize,
        filepath: &str,
    ) -> Result<(), Box<dyn Error>> {
        let part = multipart::Part::file(filepath)
            .await?
            .file_name("preview.pvg");
        let form = multipart::Form::new().part("szFile", part);

        let res = self
            .cli
            .post(format!(
                "{}/api/client/PrintJob/UploadPreview?dwJobId={}",
                BASE_URL, dw_jobid
            ))
            .multipart(form)
            .header("User-Agent", "UPMClient 1.0")
            .send()
            .await?;

        let res = res.json::<StatusCodeResponse>().await?;
        println!("{:?}", res);
        match res.code {
            0 => Ok(()),
            _ => Err("Upload Preview failed".into()),
        }
    }

    pub async fn set_job(&self, dw_job_id: usize) -> Result<(), Box<dyn Error>> {
        if let Some(osession_id) = self.jar.cookies(&self.base_url) {
            let session_str: String = osession_id.to_str()?.to_owned();
            // OSESSIONID=
            let osession_id = String::from(&session_str[11..]);

            let req_json = SetJobRequest {
                dw_job_id,
                dw_status: 1,
                osession_id,
            };

            let res = self
                .cli
                .post(format!("{}/api/client/PrintJob/Set", BASE_URL))
                .json(&req_json)
                .send()
                .await?;

            let res: StatusCodeResponse = res.json().await?;
            println!("{:?}", res);
            return Ok(());
        }
        Err("Set Job Failed".into())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn test_client() {
    let client = Client::new();
    client.load_cookie("./test.txt");
    // if !client.check_login().await.unwrap() {
    //     println!("Not Logged in");
    //     client.login().await.unwrap();
    // }
    client.store_cookie("./test.txt");
}
