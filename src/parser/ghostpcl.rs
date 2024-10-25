#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{
    ffi::{CStr, CString},
    io,
    os::raw::c_void,
    ptr,
};

use super::{nompcl::parse_pcl, PCLParser};
use crate::{
    config::ensure_directory,
    job::{Color, Job},
};
use chrono::Local;
use image::{DynamicImage, GenericImageView, Pixel};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn is_color(img: &DynamicImage) -> bool {
    // let image_path = format!("{}/1.png", self.temp_dir);
    // let img =

    for pixel in img.pixels() {
        let channels = pixel.2.channels();
        if channels[0] != channels[1] || channels[1] != channels[2] {
            return true;
        }
    }
    false
}

#[derive(Debug)]
enum GhostPCLError {
    GsapiExitError,
}

fn pcl2png(input: &str, temp_dir: &str) -> Result<(), GhostPCLError> {
    // TODO: Check if input exists.

    let mut minst: *mut c_void = ptr::null_mut();

    static arg0: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b" \0") };
    static arg1: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"-dNOPAUSE\0") };
    static arg2: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"-dBATCH\0") };
    static arg3: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"-dSAFER\0") };
    static arg4: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"-sDEVICE=png16\0") };
    static arg5: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"-r100\0") };
    // Store CString instances in variables
    let arg6 = CString::new(format!("-sOutputFile={}/%d.png", temp_dir)).unwrap();
    let arg7 = CString::new(input).unwrap();

    let argv: [*const i8; 8] = [
        arg0.as_ptr(),
        arg1.as_ptr(),
        arg2.as_ptr(),
        arg3.as_ptr(),
        arg4.as_ptr(),
        arg5.as_ptr(),
        arg6.as_ptr(),
        arg7.as_ptr(),
    ];

    unsafe {
        if gsapi_new_instance(&mut minst, ptr::null_mut()) < 0 {
            panic!("Error Creating gsapi instance");
        }
        if gsapi_set_arg_encoding(minst, PL_ARG_ENCODING_UTF8.try_into().unwrap()) == 0 {
            gsapi_init_with_args(
                minst,
                argv.len().try_into().unwrap(),
                argv.as_ptr() as *mut *mut i8,
            );
        }

        // WARN: Doesn't seem to work on Windows?
        if gsapi_exit(minst) != 0 {
            // Failed to parse PCL File.
            gsapi_delete_instance(minst);
            return Err(GhostPCLError::GsapiExitError);
        }
        gsapi_delete_instance(minst);
    }
    Ok(())
}

pub struct GhostPCL {
    temp_dir: String,
}

impl GhostPCL {
    pub fn new(dir: &str) -> io::Result<Self> {
        ensure_directory(dir);
        Ok(Self {
            temp_dir: String::from(dir),
        })
    }
}

impl PCLParser for GhostPCL {
    fn get_job(&self, input: &str) -> Option<Job> {
        // TODO: Get Timestamp as name
        if let Err(e) = pcl2png(input, &self.temp_dir) {
            eprintln!("{:?}", e);
            return None;
        }
        // Grep temp images in temp dir
        let mut images = vec![];
        for entry in std::fs::read_dir(&self.temp_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("png") {
                let img = image::open(&path).expect("Failed to open image");
                images.push(img);
                std::fs::remove_file(&path).expect("Failed to delete image file");
            }
        }
        // Count Page Number
        let number: u32 = images.len().try_into().unwrap();
        // Check their color
        let color_map: Vec<bool> = images.iter().map(is_color).collect();
        let color_pages: u32 = color_map.iter().filter(|&&c| c).count().try_into().unwrap();
        let color_map: String = color_map
            .iter()
            .map(|c| match c {
                true => "1",
                false => "0",
            })
            .collect();
        let bw_pages = number - color_pages;

        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();

        let job = parse_pcl(input).unwrap();

        Some(Job {
            id: 0,
            name: timestamp,
            color: match color_pages {
                0 => Color::BW,
                _ => Color::COLOR,
            },
            bw_pages,
            color_pages,
            color_map,
            number,
            copies: job.copies,
            paper_size: job.paper_size,
            direction: job.direction,
            duplex: job.duplex,
        })
    }

    fn get_job_from_raw(&self, content: &[u8]) -> Option<Job> {
        todo!()
    }
}

#[test]
fn test_png() {
    let parser = GhostPCL::new(".").unwrap();
    // parser.get_job("/home/liu/Desktop/RustPrinter/temp_data.bin");
    println!("{:?}", parser.get_job(r"C:\upmclient\temp\8.tmp"));
}
