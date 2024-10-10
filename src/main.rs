mod client;
mod spooler;

use crate::client::Client;
use regex::Regex;
use spooler::{windows::WindowsSpooler, Job, Spooler};
use std::env;

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

        return Some(OS::Windows((job_id, printer_name)));
    }

    fn parse_args(args: &Vec<String>) -> Self {
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

        return OS::Others;
    }
}

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

    // TODO: Find PCL File.

    // Parse Job Info
    let job: Job = match op_system {
        OS::Windows((job_id, printer_name)) => {
            let sp = WindowsSpooler::new(&printer_name).unwrap();
            sp.get_job(job_id)
        }
        _ => None,
    }
    .expect("Can not find desired Job");

    let client = Client::new();

    // TODO: File Access should not be the client's Job
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
    client.set_job(id).await.unwrap();
    client.store_cookie("./test.txt");
}
