use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub enum Color {
    #[default]
    BW,
    COLOR,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub enum Orientation {
    #[default]
    PORTRAIT,
    LANDSCAPE,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub enum Duplex {
    #[default]
    SIMPLEX, // 普通打印
    DUPLEX_LONG_EDGE,  // 长边翻转
    DUPLEX_SHORT_EDGE, // 短边翻转
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub enum Size {
    A3,
    #[default]
    A4,
}

impl From<Size> for i16 {
    fn from(val: Size) -> Self {
        match val {
            Size::A3 => 8,
            Size::A4 => 9,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Job {
    pub id: u32,      // Job ID
    pub name: String, // Job Name (File Name)
    pub color: Color, // B&W / Color
    pub bw_pages: u32,
    pub color_pages: u32,
    pub color_map: String,
    pub number: u32, // How many pages
    pub copies: u32,
    pub paper_size: Size,       // A3/A4
    pub direction: Orientation, // P/L
    pub duplex: Duplex,
}
