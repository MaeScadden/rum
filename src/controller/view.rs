use std::{
    fs::File,
    io::{self, BufReader},
};

use ropey::Rope;

#[derive(Default)]
pub struct View {
    pub rope: Rope,
    col: usize,
    row: usize,
}

impl View {
    pub fn new(file_path: String) -> io::Result<Self> {
        Ok(Self {
            rope: Rope::from_reader(BufReader::new(File::open(file_path)?))?,

            col: 0,
            row: 0,
        })
    }
}
