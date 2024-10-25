mod client;
mod config;
mod job;
mod parser;
mod server;
mod spooler;

use crate::client::Client;
use config::{load_cookie, save_cookie, temp_dir};
use core::panic;
use flate2::write::GzEncoder;
use flate2::Compression;
use job::Job;
use parser::ghostpcl::GhostPCL;
use parser::PCLParser;
use regex::Regex;
use server::Server;
#[cfg(target_os = "windows")]
use spooler::windows::WindowsSpooler;
use spooler::Spooler;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;

#[derive(Clone, Debug)]
enum OS {
    #[cfg(target_os = "windows")]
    Windows((u32, String)),
    Others,
}

impl OS {
    #[cfg(target_os = "windows")]
    fn parse_windows_args(arg_1: &str, arg_2: &str) -> Option<OS> {
        // Check for Windows
        let jobid_re = Regex::new(r"/JOBID:(\d+)").unwrap();
        let printer_re = Regex::new(r"/PRINTER:(\w+)").unwrap();

        let caps = jobid_re.captures(arg_1)?;
        let job_id = caps.get(1).map(|m| m.as_str().parse::<u32>().unwrap())?;

        let caps = printer_re.captures(arg_2)?;
        let printer_name = caps.get(1).map(|m| m.as_str().to_string())?;

        Some(OS::Windows((job_id, printer_name)))
    }

    fn parse_args(args: &[String]) -> Self {
        if args.len() == 1 {
            println!("Working as server...");
            return OS::Others;
        }

        if args.len() == 3 {
            #[cfg(target_os = "windows")]
            if let Some(os) = OS::parse_windows_args(&args[1], &args[2]) {
                return os;
            }
        }

        eprintln!("Server Usage: {}", args[0]);
        eprintln!(
            "Windows Patch Usage: {} /JOBID:[JOBID] /PRINTER:[PRINTER_NAME]",
            args[0]
        );

        panic!("Invalid args");
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Check Operating System
    let op_system = OS::parse_args(&args);

    // Get PCL File & Job Info

    // Set Temp File Dir
    let temp_dir = match op_system {
        #[cfg(target_os = "windows")]
        OS::Windows((_, _)) => String::from(r"C:\upmclient\temp"),
        OS::Others => String::from(temp_dir().to_str().unwrap()),
    };

    #[allow(unused_assignments)]
    let mut job = Job::default();
    #[allow(unused_assignments)]
    let mut pcl_file_path = String::new();

    match op_system {
        #[cfg(target_os = "windows")]
        OS::Windows((job_id, printer_name)) => {
            let sp = WindowsSpooler::new(&printer_name).unwrap();
            job = sp.get_job(job_id).expect("No Job!");
            pcl_file_path = format!("{}/{}.tmp", temp_dir, job.id);
        }

        OS::Others => {
            // Waiting for socket sonnection
            let server = Server::new(6981, &temp_dir)
                .await
                .expect("Cannot start Server");
            // Save PCL File
            server.run().await;
            // PCL File Saved Here
            pcl_file_path = temp_dir.clone() + "/temp.bin";
            let parser = GhostPCL::new(&temp_dir).unwrap();
            job = parser.get_job(&pcl_file_path).expect("No Job");
        }
    };

    if fs::metadata(&pcl_file_path).is_err() {
        eprintln!("PCL file not found: {}", pcl_file_path);
        return;
    }

    // PCL & Job Info Completed At this Point.

    // Compress the PCL File
    let compressed_pcl_file_path = format!("{}/{}.tmp2", temp_dir, job.id);

    gzip_compress(&pcl_file_path, &compressed_pcl_file_path).expect("Compress Failed.");

    let client = Client::new();

    if let Some(cookie) = load_cookie() {
        client.load_cookie(&cookie);
    }

    if !client.check_login().await.unwrap() {
        println!("Not Logged in");
        client.login().await.unwrap();
    }

    println!("Logged in");

    if let Some(cookie) = client.output_cookie() {
        save_cookie(cookie);
    }

    // Confirmation
    {
        println!("请确认打印任务信息。按回车键确认上传打印文件；按其他任意键退出。");
        println!("{:?}", &job);
        println!("{:?}", Client::job_to_paper_detail(&job));
        println!("Press Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }

    // Remote job id
    let id = client.create_job(&job).await.unwrap();
    println!("Remote jobId: {}", id);

    // Load from Tmp File.
    client
        .upload_job(id, &compressed_pcl_file_path)
        .await
        .unwrap();

    let embedded_file: &[u8] = include_bytes!("../resources/preview.pvg");

    client
        .upload_preview(id, Vec::from(embedded_file))
        .await
        .unwrap();

    client.set_job(id).await.unwrap();

    if let Some(cookie) = client.output_cookie() {
        save_cookie(cookie);
    }

    // Wait for user input to exit
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

/// Compress input to output in gzip format
fn gzip_compress(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;
    let mut encoder = GzEncoder::new(&mut output_file, Compression::default());
    std::io::copy(&mut input_file, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}
