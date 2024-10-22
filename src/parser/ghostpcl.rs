#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::PCLParser;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

struct GhostPCL {}

impl PCLParser for GhostPCL {
    fn get_job(input: &str) -> Option<RustPrinter::Job> {
        todo!()
    }

    fn get_job_from_raw(content: &[u8]) -> Option<RustPrinter::Job> {
        todo!()
    }
}
