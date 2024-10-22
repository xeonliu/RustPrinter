use RustPrinter::Job;

#[cfg(target_os = "windows")]
pub mod windows;

pub trait Spooler {
    fn get_job(&self, id: u32) -> Option<Job>;
    fn delete_job(&self);
}
