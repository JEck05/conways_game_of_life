pub mod board;


use crate::board_output_handlers::{BoardOutputHandlers, console_output_handler::ConsoleOutputHandler};
use board::Board;
use std::{thread, time, rc::Rc};
use std::cell::{RefCell};
use std::rc::Weak;

pub struct Game{
    output_observers: RefCell<Vec<Weak<dyn BoardOutputHandlers>>>,
    board: RefCell<Option<Board>>,
    default_handler: Rc<dyn BoardOutputHandlers>,
}
impl Game {
    pub fn new() -> Self {
        let default_handler: Rc<dyn BoardOutputHandlers> = Rc::new(ConsoleOutputHandler::new());
        Game { output_observers: RefCell::new(vec![Rc::downgrade(&default_handler)]), board: RefCell::new(None), default_handler}
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
    pub fn set_default_handler(&mut self, output_observer: Rc<dyn BoardOutputHandlers>) -> &mut Self{
        self.output_observers.borrow_mut().push(Rc::downgrade(&output_observer));
        self.remove_default_output_handler();
        self.default_handler = output_observer;
        self
    }
    pub fn remove_default_output_handler(&mut self) -> &Self{
        self.remove_output_handlers(&Rc::clone(&self.default_handler))
    }
    pub fn add_output_handlers(&self, observer: Rc<dyn BoardOutputHandlers>) -> &Self {
        self.output_observers.borrow_mut().push(Rc::downgrade(&observer));
        self
    }
    pub fn remove_output_handlers(&self, observer: &Rc<dyn BoardOutputHandlers>) -> &Self{
        self.output_observers.borrow_mut().retain(|observers| {
            !Weak::ptr_eq(observers, &Rc::downgrade(&observer))
        });
        self
    }
    fn notify_observers(output_observers: &Vec<Weak<dyn BoardOutputHandlers>>, board: Vec<Vec<bool>>) {
        output_observers
            .iter()
            .for_each(
                |observer| {
                    match observer.upgrade() {
                        Some(value) =>  value.display(&board),
                        None=> (),
                    }
                }
            )
    }

}
