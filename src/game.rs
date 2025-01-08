pub mod board;


use crate::board_output_handlers::{BoardOutputHandlers, console_output_handler::ConsoleOutputHandler};
use board::Board;
use std::{thread, time, rc::Rc};
use std::cell::{RefCell};

pub struct Game{
    output_observers: RefCell<Vec<Rc<dyn BoardOutputHandlers>>>,
    board: RefCell<Option<Board>>
}
impl Game{
    pub fn new()-> Self{
        Game{output_observers: RefCell::new(vec![Rc::new(ConsoleOutputHandler::new())]), board: RefCell::new(None) }
    }
    pub fn initialize_board(&self, width: usize, height: usize, initial_coords: Vec<(usize, usize)>) -> &Self{
        self.board.replace(Some(Board::from(width, height, initial_coords)));
        self
    }
     pub fn start_game(&self, num_of_steps: usize) {
         let mut board_ref = self.board.borrow_mut();
         let initial_state = board_ref.as_mut().unwrap();

         for i in 0..=num_of_steps {
            initial_state.next_turn();

            print!("Generation: {i}");

            Self::notify_observers(&self.output_observers.borrow(), initial_state.to_displayable_board());

            thread::sleep(time::Duration::from_millis(500));
         }
    }
    pub fn add_output_handlers(&self, observer: Rc<dyn BoardOutputHandlers>) -> &Self{
        self.output_observers.borrow_mut().push(observer);
        self
    }
    fn notify_observers(output_observers: &Vec<Rc<dyn BoardOutputHandlers>>, board: Vec<Vec<bool>>){
        output_observers.iter().for_each(|observer|{ observer.display(&board)})
    }
}
