pub mod windows;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    BW,
    COLOR,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    PORTRAIT,
    LANDSCAPE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Duplex {
    SIMPLEX,    // 普通打印
    HORIZONTAL, // 短边翻转
    VERTICAL,   // 长边翻转
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    A3,
    A4,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub id: u32,      // Job ID
    pub name: String, // Job Name (File Name)
    pub color: Color, // B&W / Color
    pub number: u32,  // How many pages
    pub copies: u32,
    pub paper_size: Size,     // A3/A4
    pub direction: Direction, // P/L
    pub duplex: Duplex,
}

pub trait Spooler {
    fn get_job(&self, id: u32) -> Option<Job>;
    fn delete_job(&self);
}
