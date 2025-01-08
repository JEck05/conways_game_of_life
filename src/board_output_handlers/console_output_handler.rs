use std::process::Command;
use crate::board_output_handlers::BoardOutputHandlers;

pub struct ConsoleOutputHandler;

impl ConsoleOutputHandler {
    pub fn new() -> Self {
        ConsoleOutputHandler
    }
}
impl BoardOutputHandlers for ConsoleOutputHandler {
    fn display(&self, board: &Vec<Vec<bool>>) {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
        println!();
        for rows in board {
            for cells in rows {
                if cells == &true {
                    print!(" X ");
                }else {
                    print!(" _ ");
                }
            }
            println!();
        }
    }
}