
pub mod board_output_handlers;
pub use board_output_handlers::*;

pub mod board;
pub use board::*;

use std::{thread, time};

pub fn start_game(mut initial_state: Board, num_of_steps: usize) {
    let console_handler = ConsoleOutputHandler::new();
    for i in 0..=num_of_steps {
        initial_state.next_turn();
        print!("Generation: {i}");
        console_handler.display(&initial_state.board);

        thread::sleep(time::Duration::from_millis(500));
    }
}
