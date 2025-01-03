
use crate::game::{Cell, Board};
use std::{thread, time, process::Command};
trait BoardOutputHandlers {
    fn display(&self, board: &Vec<Vec<Cell>>);
}
pub struct ConsoleOutputHandler;
impl ConsoleOutputHandler {
    fn new() -> Self {
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

pub fn start_game(mut initial_state: Board, num_of_steps: usize) {
    let console_handler = ConsoleOutputHandler::new();
    for i in 0..=num_of_steps {
        print!("Generation: {i}");
        initial_state.next_turn();
        console_handler.display(&initial_state.board);

        thread::sleep(time::Duration::from_millis(100));
    }
}

