use crate::client::Client;
use crate::job::{Color, Duplex, Job, Orientation, Size};
use crate::spooler::Spooler;

use serde::de::IntoDeserializer;
use windows::core::PCWSTR;
use windows::Win32::Foundation;
use windows::Win32::Graphics::Gdi::{self, DMDUP_HORIZONTAL, DMDUP_SIMPLEX, DMDUP_VERTICAL};
use windows::Win32::Graphics::Printing::{GetJobW, OpenPrinterW, JOB_INFO_2W};

#[derive(Debug)]
pub struct WindowsSpooler {
    printer_name: String,
    printer_handle: Foundation::HANDLE,
}

impl Orientation {}

impl TryFrom<i16> for Size {
    type Error = &'static str;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            8 => Ok(Self::A3),
            9 => Ok(Self::A4),
            _ => Err("Size Error"),
        }
    }
}

impl TryFrom<i16> for Color {
    type Error = &'static str;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::BW),
            2 => Ok(Self::COLOR),
            _ => Err("Size Error"),
        }
    }
}

impl TryFrom<i16> for Orientation {
    type Error = &'static str;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::PORTRAIT),
            2 => Ok(Self::LANDSCAPE),
            _ => Err("Size Error"),
        }
    }
}

impl TryFrom<i16> for Duplex {
    type Error = &'static str;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match Gdi::DEVMODE_DUPLEX(value) {
            DMDUP_SIMPLEX => Ok(Self::SIMPLEX),
            DMDUP_HORIZONTAL => Ok(Self::DUPLEX_SHORT_EDGE),
            DMDUP_VERTICAL => Ok(Self::DUPLEX_LONG_EDGE),
            _ => Err("Duplex Error"),
        }
    }
}

impl WindowsSpooler {
    pub fn new(name: &str) -> windows::core::Result<Self> {
        let printer_name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        let printer_name: PCWSTR = PCWSTR::from_raw(printer_name.as_ptr());
        let mut printer_handle = Foundation::HANDLE(core::ptr::null_mut());
        unsafe {
            OpenPrinterW(printer_name, &mut printer_handle, None)?;
            println!("Printer Handle: {:?}", printer_handle);
        }

        Ok(WindowsSpooler {
            printer_name: name.into(),
            printer_handle,
        })
    }
}

impl Spooler for WindowsSpooler {
    fn get_job(&self, id: u32) -> Option<Job> {
        let mut pcb_needed: u32 = 0;

        unsafe {
            // Level 2 JOB INFO
            if GetJobW(self.printer_handle, id, 2, None, &mut pcb_needed).into() {
                return None;
            }
        }

        // Create C Buf
        let mut buffer = vec![0u8; pcb_needed as usize];

        unsafe {
            if GetJobW(
                self.printer_handle,
                id,
                2,
                Some(&mut buffer),
                &mut pcb_needed,
            ) == Foundation::FALSE
            {
                return None;
            }
        }
        let job_info: JOB_INFO_2W = unsafe { *(buffer.as_ptr() as *const JOB_INFO_2W) };

        let name = unsafe { job_info.pDocument.to_string().unwrap() };
        println!("Name: {:?}", name);

        let number = job_info.TotalPages;
        println!("Total Pages: {:?}", number);

        let pages_printed = job_info.PagesPrinted;
        println!("Pages Printed: {:?}", pages_printed);

        let dev_mode = unsafe { *(job_info.pDevMode) };

        // TODO: Check BitMap

        let paper_size: Size = unsafe {
            dev_mode
                .Anonymous1
                .Anonymous1
                .dmPaperSize
                .try_into()
                .expect("Paper Size Not Supported")
        };
        println!("Size: {:?}", paper_size);

        let direction: Orientation = unsafe {
            dev_mode
                .Anonymous1
                .Anonymous1
                .dmOrientation
                .try_into()
                .expect("Orientation not supported")
        };
        println!("Orientation: {:?}", direction);

        let duplex: Duplex = dev_mode.dmDuplex.0.try_into().unwrap();
        println!("Duplex: {:?}", duplex);

        let color: Color = dev_mode
            .dmColor
            .0
            .try_into()
            .expect("Color Mode Not Supported");

        println!("Color: {:?}", color);

        // TODO: Identify BW_Pages?
        let copies: u32 = unsafe { dev_mode.Anonymous1.Anonymous1.dmCopies }
            .try_into()
            .expect("Error on copies");
        println!("Copies: {:?}", copies);

        let bw_pages = match color {
            Color::BW => number,
            _ => 0,
        };

        let color_pages = match color {
            Color::COLOR => number,
            _ => 0,
        };

        // TODO: Whether each page is colored or not?
        let color_map = match color {
            Color::BW => "0".repeat(number as usize),
            Color::COLOR => "1".repeat(number as usize),
        };

        return Some(Job {
            id: job_info.JobId,
            name,
            color,
            bw_pages,
            color_pages,
            color_map,
            number,
            paper_size,
            direction,
            duplex,
            copies,
        });
    }

    fn delete_job(&self) {
        todo!()
    }
}

#[test]
fn test() {
    let sp = WindowsSpooler::new("联创打印管理系统").unwrap();
    println!("{:?}", sp);
    let jb = sp.get_job(6);
    println!("{:?}", jb);

    if let Some(job) = jb {
        println!("{:?}", Client::job_to_sz_attribute(&job));
        println!("{:?}", Client::job_to_paper_detail(&job));
    }
}
