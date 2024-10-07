pub mod windows;

#[derive(Debug)]
enum Color {
    BW,
    COLOR,
}

#[derive(Debug)]
enum Direction {
    VERTICAL,
    HORIZONTAL,
}

#[derive(Debug)]
enum Edge {
    LONG,
    SHORT,
}

#[derive(Debug)]
enum Side {
    SINGLE,
    BOTH(Edge),
}

#[derive(Debug)]
enum Size {
    A3,
    A4,
}

#[derive(Debug)]
pub struct Job {
    id: u32,              // Job ID
    name: String,         // Job Name (File Name)
    color: Color,         // B&W / Color
    number: u32,          // How many prints
    paper_size: Size,     // A3/A4
    direction: Direction, // V/H
    side: Side,           // Single / Both
}

pub trait Spooler {
    fn get_job(&self, id: u32) -> Option<Job>;
    fn delete_job(&self);
}
