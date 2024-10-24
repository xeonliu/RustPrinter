use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Debug)]
pub enum Orientation {
    PORTRAIT,
    LANDSCAPE,
    REVERSE_PORTRAIT,
    REVERSE_LANDSCAPE,
}

fn parse_orientation(input: &str) -> Result<Orientation, &'static str> {
    let re = Regex::new(r"\x1B&l([01])[Oo]").unwrap();
    if let Some(caps) = re.captures(input) {
        let orientation = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
        match orientation {
            0 => Ok(Orientation::PORTRAIT),
            1 => Ok(Orientation::LANDSCAPE),
            2 => Ok(Orientation::REVERSE_PORTRAIT),
            3 => Ok(Orientation::REVERSE_LANDSCAPE),
            _ => Err("Invalid orientation value"),
        }
    } else {
        Err("No match found")
    }
}

#[derive(Debug)]
pub enum PageSize {
    A4,
    A3,
    // Add more page sizes as needed
}

fn parse_page_size(input: &str) -> Result<PageSize, &'static str> {
    let re = Regex::new(r"\x1B&l(\d+)[Aa]").unwrap();
    if let Some(caps) = re.captures(input) {
        let page_size = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
        match page_size {
            26 => Ok(PageSize::A4),
            27 => Ok(PageSize::A3),
            _ => Err("Invalid page size value"),
        }
    } else {
        Err("No match found")
    }
}

#[derive(Debug)]
pub enum DuplexMode {
    SIMPLEX,
    DUPLEX_LONG_EDGE,
    DUPLEX_SHORT_EDGE,
}

fn parse_duplex_binding(input: &str) -> Result<DuplexMode, &'static str> {
    let re = Regex::new(r"\x1B&l([012])[Ss]").unwrap();
    if let Some(caps) = re.captures(input) {
        let duplex_binding = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
        match duplex_binding {
            0 => Ok(DuplexMode::SIMPLEX),
            1 => Ok(DuplexMode::DUPLEX_LONG_EDGE),
            2 => Ok(DuplexMode::DUPLEX_SHORT_EDGE),
            _ => Err("Invalid duplex binding value"),
        }
    } else {
        Err("No match found")
    }
}

fn parse_pcl(input_file: &str) -> io::Result<()> {
    // 读取文件内容
    let mut file = File::open(input_file)?;
    let mut buf_reader = BufReader::new(file);

    // 读取文件内容到 Vec<u8>
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;

    // 剔除非 ASCII 字符
    let ascii_contents = contents
        .into_iter()
        .filter(|&x| x.is_ascii())
        .collect::<Vec<u8>>();

    let text = String::from_utf8(ascii_contents).unwrap();

    // 编译正则表达式来匹配模式 \x1B*b(某个整数)W，并且捕获整数 x
    let re = Regex::new(r"\x1B\*b(\d+)W").unwrap();

    // 使用正则表达式替换匹配的内容，以及紧跟的 x 个字符
    let buffer = re.replace_all(&text, |caps: &regex::Captures| {
        let x: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        "".repeat(x)
    });

    // 解析文件内容
    match parse_orientation(&buffer) {
        Ok(direction) => println!("Parsed direction: {:?}", direction),
        Err(e) => println!("Error parsing file: {}", e),
    }

    match parse_page_size(&buffer) {
        Ok(page_size) => println!("Parsed page size: {:?}", page_size),
        Err(e) => println!("Error parsing page size: {}", e),
    }

    match parse_duplex_binding(&buffer) {
        Ok(duplex_mode) => println!("Parsed duplex mode: {:?}", duplex_mode),
        Err(e) => println!("Error parsing duplex mode: {}", e),
    }

    let pages: Vec<&str> = buffer.split("\x0C\x1B").collect();

    println!("Page Number: {}", pages.len());

    Ok(())
}

#[test]
fn test() {
    let input_file = r"C:\upmclient\temp\17.tmp";
    if let Err(e) = parse_pcl(input_file) {
        println!("Error: {}", e);
    }
}
