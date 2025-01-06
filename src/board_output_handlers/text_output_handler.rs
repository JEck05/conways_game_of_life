use crate::board_output_handlers::BoardOutputHandlers;
use std::fs::OpenOptions;
use std::io::Write;

pub struct TextOutputHandler {
    file_name: String
}

impl TextOutputHandler{
    pub fn new(file_name: String) -> Self{
        TextOutputHandler{ file_name }
    }
}
impl BoardOutputHandlers for TextOutputHandler {
    fn display(&self, board: &Vec<Vec<bool>>) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_name)
            .unwrap();
        for row in board {
            let line: String = row.iter()
                .map(|&b| if b { " X " } else { " _ " })
                .collect();
            writeln!(file, " {} ", line).unwrap();
        }
    }
}