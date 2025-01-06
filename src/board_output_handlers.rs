pub mod console_output_handler;
pub mod text_output_handler;


pub trait BoardOutputHandlers {
    fn display(&self, board: &Vec<Vec<bool>>);
}
