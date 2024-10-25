use super::model::{CreateJobRequest, SetJobRequest};
use super::{consts::BASE_URL, Client};
use crate::client::model::{
    CreateJobResponse, GetAuthTokenResponse, StatusCodeResponse, WaitUserInResponse,
};
use crate::job::{Color, Duplex, Job};
use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
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

    pub fn load_cookie(&self, cookie: &str) {
        self.jar.add_cookie_str(&cookie, &self.base_url);
    }

    pub fn output_cookie(&self) -> Option<HeaderValue> {
        self.jar.cookies(&self.base_url)
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
        let json: GetAuthTokenResponse = res.json().await.expect("AUTH JSON PARSE ERROR");
        println!("Login Token: {}", json.sz_token);
        Ok(json.sz_token)
    }

    pub(crate) async fn login(&self) -> Result<(), Box<dyn Error>> {
        let token = self.get_token().await?;

        // Generate QR Code Using the token
        let login_url = format!("http://pay.unifound.net/uniwx/s.aspx?c=uniauth_1_{}", token);
        println!("在微信中打开该链接以登录： {}", login_url);

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

                if let Ok(res) = res {
                    let res = res
                        .json::<WaitUserInResponse>()
                        .await
                        .expect("Wrong String");
                    if res.result.is_some() {
                        tx.send("Success").unwrap();
                        return;
                    }
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

    /* Put it here or in Job Struct? */
    /// Constructs a String
    /// + Black & Simplex "single,collate,NUP1,"
    /// + Color & Vertical "color,vdup,collate,NUP1,"
    pub fn job_to_sz_attribute(job: &Job) -> String {
        let mut attributes: Vec<String> = Vec::new();

        match job.color {
            Color::BW => {
                attributes.push("single".into());
            }
            Color::COLOR => {
                attributes.push("color".into());
            }
        }

        match job.duplex {
            Duplex::SIMPLEX => {}
            Duplex::DUPLEX_SHORT_EDGE => {
                attributes.push("hdup".into());
            }
            Duplex::DUPLEX_LONG_EDGE => {
                attributes.push("vdup".into());
            }
        }

        attributes.push("collate".into());
        attributes.push("NUP1".into());
        attributes.push("".into());

        attributes.join(",")
    }

    pub fn job_to_paper_detail(job: &Job) -> String {
        let paper_id: i16 = job.paper_size.clone().into();
        // TODO: Identify BW_Pages?

        let paper_num = match job.duplex {
            Duplex::SIMPLEX => job.number,
            _ => (job.number + 1) / 2,
        };

        format!(
            "[{{\"dwPaperID\":{},\"dwBWPages\":{},\"dwColorPages\":{},\"dwPaperNum\":{}}}]",
            paper_id, job.bw_pages, job.color_pages, paper_num,
        )
    }

    pub async fn create_job(&self, job: &Job) -> Result<usize, Box<dyn Error>> {
        // Create Job Data.

        // Construct sz_attribute
        let sz_attribe = Self::job_to_sz_attribute(job);

        // Construct sz_paper_detail
        let sz_paper_detail = Self::job_to_paper_detail(job);

        let req_json = CreateJobRequest {
            dw_property: 0,
            sz_job_name: job.name.clone(),
            dw_copies: job.copies.into(),
            sz_attribe,
            sz_paper_detail,
            // TODO: Whether each page is colored or not?
            sz_color_map: match job.color {
                Color::BW => "0".repeat(job.number as usize),
                Color::COLOR => "1".repeat(job.number as usize),
            },
        };

        let res = self
            .cli
            .post(format!("{}/api/client/PrintJob/Create", BASE_URL))
            .json(&req_json)
            .send()
            .await?;

        let res: CreateJobResponse = res.json().await?;
        println!(
            "New job with remote job id {} has been created.",
            res.result.dw_job_id
        );

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
        bin: Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let part = multipart::Part::bytes(bin).file_name("preview.pvg");
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
