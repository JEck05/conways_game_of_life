mod board;


use crate::board_output_handlers::{BoardOutputHandlers, console_output_handler::ConsoleOutputHandler};
use board::Board;
use std::{thread, time, rc::Rc};

pub struct Game{
    output_observers: Vec<Rc<dyn BoardOutputHandlers>>,
    board: Option<Board>,
    default_handler: Rc<dyn BoardOutputHandlers>,
}
impl Game{
    pub fn new()-> Self{
        let default_handler: Rc<dyn BoardOutputHandlers> =Rc::new(ConsoleOutputHandler::new());
        Game{output_observers: vec![], board: None , default_handler}
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

            println!();
            thread::sleep(time::Duration::from_millis(500));
        }
    }
    pub fn set_default_handler(&mut self, output_observer: Rc<dyn BoardOutputHandlers>){
        self.output_observers.push(Rc::clone(&output_observer));
        self.remove_default_output_handler();
        self.default_handler = output_observer;
    }
    pub fn remove_default_output_handler(&mut self) {
       self.remove_output_handlers(&Rc::clone(&self.default_handler))
    }
    pub fn add_output_handlers(& mut self, observer: Rc<dyn BoardOutputHandlers>){
        self.output_observers.push(observer);
    }
    pub fn remove_output_handlers(&mut self, observer: &Rc<dyn BoardOutputHandlers>){
        self.output_observers.retain(|observers| {
            !Rc::ptr_eq(observers, &observer)
        });
    }
    fn notify_observers(output_observers: &Vec<Rc<dyn BoardOutputHandlers>>, board: Vec<Vec<bool>>){
        output_observers.iter().for_each(|observer|{ observer.display(&board)})
    }
}