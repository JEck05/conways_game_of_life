pub mod board;


use crate::board_output_handlers::{BoardOutputHandlers, console_output_handler::ConsoleOutputHandler};
use board::Board;
use std::{thread, time, rc::Rc};
use std::cell::{RefCell};

pub struct Game{
    output_observers: RefCell<Vec<Rc<dyn BoardOutputHandlers>>>,
    board: RefCell<Option<Board>>,
    default_handler: RefCell<Rc<dyn BoardOutputHandlers>>,
}
impl Game {
    pub fn new() -> Self {
        let default_handler: RefCell<Rc<dyn BoardOutputHandlers>> = RefCell::new(Rc::new(ConsoleOutputHandler::new()));

        Game { output_observers: RefCell::new(vec![]), board: RefCell::new(None), default_handler }
    }
    pub fn initialize_board(&self, width: usize, height: usize, initial_coords: Vec<(usize, usize)>) -> &Self {
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

            println!();
            thread::sleep(time::Duration::from_millis(500));
        }
    }
    pub fn set_default_handler(&self, output_observer: Rc<dyn BoardOutputHandlers>) {
        self.output_observers.borrow_mut().push(Rc::clone(&output_observer));
        self.remove_default_output_handler();
        self.default_handler.replace( output_observer);
    }
    pub fn remove_default_output_handler(&self) {
        self.remove_output_handlers(&Rc::clone(&self.default_handler.borrow()))
    }
    pub fn add_output_handlers(&self, observer: Rc<dyn BoardOutputHandlers>) {
        self.output_observers.borrow_mut().push(observer);
    }
    pub fn remove_output_handlers(&self, observer: &Rc<dyn BoardOutputHandlers>) {
        self.output_observers.borrow_mut().retain(|observers| {
            !Rc::ptr_eq(observers, &observer)
        });
    }
    fn notify_observers(output_observers: &Vec<Rc<dyn BoardOutputHandlers>>, board: Vec<Vec<bool>>) {
        output_observers.iter().for_each(|observer| { observer.display(&board)})
    }

}
