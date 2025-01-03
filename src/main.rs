use conways_game_of_life::game::{self,Board};


fn main() {
    //    X
    //      X
    //  X X X
    let _vec = vec![(4,4), (4, 5), (4, 6)];
    //   x x
    // x     x
    //   x x
    let _vec2 = vec![(0, 3), (0, 4), (2, 3), (2, 4), (1, 2), (1 ,5)];
    // X X X X
    //     X
    let _vec4 = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 2)];

    let gosper_glider = vec![(7+24, 20+0), (7+22, 20+1), (7+24, 20+1),
(7+12, 20+2), (7+13, 20+2), (7+20, 20+2), (7+21, 20+2), (7+34, 20+2), (7+35, 20+2),
(7+11, 20+3), (7+15, 20+3), (7+20, 20+3), (7+21, 20+3), (7+34, 20+3), (7+35, 20+3),
(7+0, 20+4), (7+1, 20+4), (7+10, 20+4), (7+16, 20+4), (7+20, 20+4), (7+21, 20+4),
(7+0, 20+5), (7+1, 20+5), (7+10, 20+5), (7+14, 20+5), (7+16, 20+5), (7+17, 20+5), (7+22, 20+5), (7+24, 20+5),
(7+10, 20+6), (7+16, 20+6), (7+24, 20+6),
(7+11, 20+7), (7+15, 20+7),
(7+12, 20+8), (7+13, 20+8)];

    let board = Board::from(50, 50, gosper_glider);


    game::start_game(board, 1000);
}




