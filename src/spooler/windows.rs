use crate::spooler::{Color, Direction, Job, Side, Size, Spooler};

use windows::core::PCWSTR;
use windows::Win32::Foundation;
use windows::Win32::Graphics::Gdi;
use windows::Win32::Graphics::Printing::{GetJobW, OpenPrinterW, JOB_INFO_2W};

#[derive(Debug)]
pub struct WindowsSpooler {
    printer_name: String,
    printer_handle: Foundation::HANDLE,
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

        println!("{:?}", job_info);
        unsafe {
            println!("{:?}", job_info.pDatatype.to_string().unwrap());
        }
        println!("{:?}", job_info);

        let dev_mode = unsafe { *(job_info.pDevMode) };

        unsafe {
            println!("Size: {:?}", dev_mode.Anonymous1.Anonymous1.dmPaperSize);
        }
        println!("Color: {:?}", dev_mode.dmColor);

        return Some(Job {
            id: job_info.JobId,
            name: unsafe { job_info.pDocument.to_string().unwrap() },
            color: Color::BW,
            number: job_info.TotalPages,
            paper_size: Size::A4,
            direction: Direction::VERTICAL,
            side: Side::SINGLE,
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
    let jb = sp.get_job(3);
    println!("{:?}", jb);
}
