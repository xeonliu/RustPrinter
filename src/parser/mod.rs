use RustPrinter::Job;

pub mod ghostpcl;

pub trait PCLParser {
    fn get_job(&self, path: &str) -> Option<Job>;
    fn get_job_from_raw(&self, content: &[u8]) -> Option<Job>;
}
