mod client;
mod job;
mod parser;
mod server;
mod spooler;

use crate::client::Client;
use flate2::write::GzEncoder;
use flate2::Compression;
use job::Job;
use regex::Regex;
#[cfg(target_os = "windows")]
use spooler::windows::WindowsSpooler;
use spooler::Spooler;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;

enum OS {
    Windows((u32, String)),
    Unix(()), // TODO
    Others,
}

impl OS {
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
        if args.len() < 3 {
            eprintln!(
                "Windows Usage: {} /JOBID:[JOBID] /PRINTER:[PRINTER_NAME]",
                args[0]
            );
            return OS::Others;
        }

        if let Some(os) = OS::parse_windows_args(&args[1], &args[2]) {
            return os;
        }

        OS::Others
    }
}

const CONFIG_DIR: &str = r"C:\upmclient";
const TEMP_DIR: &str = r"C:\upmclient\temp";

#[tokio::main]
async fn main() {
    // TODO: Load Config Files
    // Set Temp File Dir

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Windows Usage: {} /JOBID:[JOBID] /PRINTER:[PRINTER_NAME]",
            args[0]
        );
        return;
    }

    // Check Operating System
    let op_system = OS::parse_args(&args);

    // Parse Job Info
    let job: Job = match op_system {
        #[cfg(target_os = "windows")]
        OS::Windows((job_id, printer_name)) => {
            let sp = WindowsSpooler::new(&printer_name).unwrap();
            sp.get_job(job_id)
        }
        _ => None,
    }
    .expect("Can not find desired Job");

    // Find PCL File.
    let pcl_file_path = format!("{}/{}.tmp", TEMP_DIR, job.id);
    if fs::metadata(&pcl_file_path).is_err() {
        eprintln!("PCL file not found: {}", pcl_file_path);
        return;
    }

    // Compress the PCL File
    let compressed_pcl_file_path = format!("{}/{}.tmp2", TEMP_DIR, job.id);

    gzip_compress(&pcl_file_path, &compressed_pcl_file_path).expect("Compress Failed.");

    let client = Client::new();

    // TODO: File Access should not be the client's Job
    client.load_cookie(&(CONFIG_DIR.to_string() + "/cookies.txt"));
    if !client.check_login().await.unwrap() {
        println!("Not Logged in");
        client.login().await.unwrap();
    }

    println!("Logged in");

    client.store_cookie(&(CONFIG_DIR.to_string() + "/cookies.txt"));

    // Confirmation
    {
        println!("请确认打印任务信息。按回车键确认上传打印文件；按其他任意键退出。");
        println!("目前不支持计算黑白彩色混合时各自的页数，建议在驱动中设置打印色彩为“黑白”。\n若以下数据不正确，请取消打印。");
        println!("{:?}", Client::job_to_paper_detail(&job));
        println!("Press Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }

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
    client.store_cookie(&(CONFIG_DIR.to_string() + "/cookies.txt"));

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
