use crate::game::{Cell};
use std::{process::Command};
pub trait BoardOutputHandlers {
    fn display(&self, board: &Vec<Vec<Cell>>);
}
pub struct ConsoleOutputHandler;
impl ConsoleOutputHandler {
    pub fn new() -> Self {
        ConsoleOutputHandler
    }
}
impl BoardOutputHandlers for ConsoleOutputHandler {
    fn display(&self, board: &Vec<Vec<Cell>>) {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
        println!();
        for rows in board {
            for cells in rows {
                if cells.alive {
                    print!(" X ");
                } else {
                    print!(" _ ");
                }
            }
            println!();
        }
    }
}



