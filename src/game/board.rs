pub struct Board {
    pub(crate) board: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    current_generation: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct Cell {
    pub(crate) alive: bool,
    next_alive_state: bool,
    last_checked: usize,
}
impl Cell {
    #[inline]
    pub fn get_alive_state(&mut self, current_gen: usize) -> bool {
        if self.last_checked != current_gen {
            self.alive = self.next_alive_state;
            self.last_checked = current_gen;
        }
        self.alive
    }
}
impl Board {
    pub fn from(width: usize, height: usize, coord_list: Vec<(usize, usize)>) -> Self {
        let mut board_vec = vec![vec![Cell { alive: false, next_alive_state: false, last_checked: 0 }; width]; height];

        for (x, y) in coord_list {
            if x != width && y != height {
                board_vec[x][y].alive = true;

                board_vec[x][y].next_alive_state = true;
            }
        }

        Board { board: board_vec, width, height, current_generation: 0 }
    }
    pub fn next_turn(&mut self) {
        for rows in 0..self.width {
            for columns in 0..self.height {
                let num_of_neighbors = self.get_neighbours_alive((rows, columns));

                let curr_cell = &mut self.board[rows][columns];

                if curr_cell.alive && (num_of_neighbors < 2 || num_of_neighbors > 3) {
                    curr_cell.next_alive_state = false;
                } else if (curr_cell.alive && num_of_neighbors == 2) || num_of_neighbors == 3 {
                    curr_cell.next_alive_state = true;
                }
            }
        }
        self.current_generation += 1;
    }
    fn get_neighbours_alive(&mut self, (x, y): (usize, usize)) -> usize {
        let mut neighbours = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                // print!("x:{i}y:{j}");
                if (j, i) == (0, 0) {
                    continue;
                }
                // if y + i = -1 then to_x equal usize::MAX because it wraps around
                // we swap x and y because to convert 8-axis to array indexes we must swap them
                let to_x = x.wrapping_add(i as usize);
                let to_y = y.wrapping_add(j as usize);

                if to_x == usize::MAX || to_y == usize::MAX || to_x == self.width || to_y == self.height {
                    continue;
                }

                if self.board[to_x][to_y].get_alive_state(self.current_generation) {
                    neighbours += 1;
                }
            }
        }
        // println!("break");
        neighbours
    }
}
