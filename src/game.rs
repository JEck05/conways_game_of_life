pub mod board;

pub use board::Board;

use crate::board_output_handlers::{BoardOutputHandlers, ConsoleOutputHandler};
use std::{thread, time,rc::Rc};

pub struct Game{
    output_observers: Vec<Rc<dyn BoardOutputHandlers>>,
}
impl Game{
    pub fn new()->Self{
        Game{output_observers: vec![Rc::new(ConsoleOutputHandler::new())]}
    }
     pub fn start_game(&mut self, mut initial_state: Board, num_of_steps: usize) {
        for i in 0..=num_of_steps {
            initial_state.next_turn();

            print!("Generation: {i}");

            self.notify_observers(&initial_state.to_displayable_board());

            thread::sleep(time::Duration::from_millis(500));
        }
    }
    pub fn add_output_handlers(& mut self, observer: Rc<dyn BoardOutputHandlers>) -> & mut Self{
        self.output_observers.push(observer);
        self
    }
    fn notify_observers(&self, board: &Vec<Vec<bool>>){
        self.output_observers.iter().for_each(|observer|{ observer.display(board)})
    }
}
