use std::rc::Rc;
use conways_game_of_life::{self, Game};
use conways_game_of_life::board_output_handlers::{BoardOutputHandlers, text_output_handler::*};
fn main() {
    let _vec = vec![(4,4), (4, 5), (4, 6)];
    //    X
    //      X
    //  X X X
    let vec1 = vec![ (0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    //   x x
    // x     x
    //   x x
    let _vec2 = vec![(0, 3), (0, 4), (2, 3), (2, 4), (1, 2), (1 ,5)];
    // X X X X
    //     X
    let _vec4 = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 2)];

    let _gosper_glider = vec![(7+24, 20+0), (7+22, 20+1), (7+24, 20+1),
    (7+12, 20+2), (7+13, 20+2), (7+20, 20+2), (7+21, 20+2), (7+34, 20+2), (7+35, 20+2),
    (7+11, 20+3), (7+15, 20+3), (7+20, 20+3), (7+21, 20+3), (7+34, 20+3), (7+35, 20+3),
    (7+0, 20+4), (7+1, 20+4), (7+10, 20+4), (7+16, 20+4), (7+20, 20+4), (7+21, 20+4),
    (7+0, 20+5), (7+1, 20+5), (7+10, 20+5), (7+14, 20+5), (7+16, 20+5), (7+17, 20+5), (7+22, 20+5), (7+24, 20+5),
    (7+10, 20+6), (7+16, 20+6), (7+24, 20+6),
    (7+11, 20+7), (7+15, 20+7),
    (7+12, 20+8), (7+13, 20+8)];

    let mut game = Game::new();
    let text_output_handler: Rc<dyn BoardOutputHandlers> = Rc::new(TextOutputHandler::new(String::from("Hello")));

    game.remove_default_output_handler();

    game.set_default_handler(text_output_handler);

    game.initialize_board(10, 10, vec1).start_game(10);

}


