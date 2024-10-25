use crate::job::{Duplex, Job, Orientation, Size};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

fn parse_orientation(input: &str) -> Result<Orientation, &'static str> {
    let re = Regex::new(r"\x1B&l([01])[Oo]").unwrap();
    if let Some(caps) = re.captures(input) {
        let orientation = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
        match orientation {
            0 => Ok(Orientation::PORTRAIT),
            1 => Ok(Orientation::LANDSCAPE),
            _ => Err("Invalid orientation value"),
        }
    } else {
        Err("No match found")
    }
}

fn parse_paper_size(input: &str) -> Result<Size, &'static str> {
    let re = Regex::new(r"\x1B&l(\d+)[Aa]").unwrap();
    if let Some(caps) = re.captures(input) {
        let page_size = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
        match page_size {
            26 => Ok(Size::A4),
            27 => Ok(Size::A3),
            _ => Err("Invalid page size value"),
        }
    } else {
        Err("No match found")
    }
}

fn parse_duplex_binding(input: &str) -> Result<Duplex, &'static str> {
    let re = Regex::new(r"\x1B&l([012])[Ss]").unwrap();
    if let Some(caps) = re.captures(input) {
        let duplex_binding = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
        match duplex_binding {
            0 => Ok(Duplex::SIMPLEX),
            1 => Ok(Duplex::DUPLEX_LONG_EDGE),
            2 => Ok(Duplex::DUPLEX_SHORT_EDGE),
            _ => Err("Invalid duplex binding value"),
        }
    } else {
        Err("No match found")
    }
}

fn parse_copies(input: &str) -> Result<u32, &'static str> {
    let re = Regex::new(r"\x1B&l(\d+)[Xx]").unwrap();
    if let Some(caps) = re.captures(input) {
        let copies: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        Ok(copies)
    } else {
        Err("No match found")
    }
}

pub fn parse_pcl(input_file: &str) -> Result<Job, Box<dyn Error>> {
    // 读取文件内容
    let file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);

    // 读取文件内容到 Vec<u8>
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    // 剔除非 ASCII 字符
    let ascii_contents = contents
        .into_iter()
        .filter(|&x| x.is_ascii())
        .collect::<Vec<u8>>();

    let buffer = String::from_utf8(ascii_contents).unwrap();

    Ok(Job {
        paper_size: parse_paper_size(&buffer)?,
        direction: parse_orientation(&buffer)?,
        duplex: parse_duplex_binding(&buffer)?,
        copies: parse_copies(&buffer)?,
        ..Default::default()
    })
}

#[test]
fn test() {
    let input_file = r"C:\upmclient\temp\17.tmp";
    if let Err(e) = parse_pcl(input_file) {
        println!("Error: {}", e);
    }
}
