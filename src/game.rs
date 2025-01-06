pub mod board;


use crate::board_output_handlers::{BoardOutputHandlers, console_output_handler::ConsoleOutputHandler};
use board::Board;
use std::{thread, time, rc::Rc};

pub struct Game{
    output_observers: Vec<Rc<dyn BoardOutputHandlers>>,
    board: Option<Board>
}
impl Game{
    pub fn new()-> Self{
        Game{output_observers: vec![Rc::new(ConsoleOutputHandler::new())], board: None }
    }
    pub fn initialize_board(&mut self, width: usize, height: usize, initial_coords: Vec<(usize, usize)>) -> &mut Self{
        self.board = Some(Board::from(width, height, initial_coords));
        self
    }
     pub fn start_game(&mut self, num_of_steps: usize) {
        let initial_state = match &mut self.board {
            Some(board) => board,
            None => panic!("Uninitialized Starting Board")
        };
        for i in 0..=num_of_steps {
            initial_state.next_turn();

            print!("Generation: {i}");

            Self::notify_observers(&self.output_observers, initial_state.to_displayable_board());

            thread::sleep(time::Duration::from_millis(500));
        }
    }
    pub fn add_output_handlers(& mut self, observer: Rc<dyn BoardOutputHandlers>) -> & mut Self{
        self.output_observers.push(observer);
        self
    }
    fn notify_observers(output_observers: &Vec<Rc<dyn BoardOutputHandlers>>, board: Vec<Vec<bool>>){
        output_observers.iter().for_each(|observer|{ observer.display(&board)})
    }
}
