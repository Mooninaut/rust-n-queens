use std::fmt;

#[derive(Debug)]
struct Board {
    size: usize,
    solutions: u64,
    rows: Vec<u8>,
    columns: u64,
    forward_diagonals: u64,
    backward_diagonals: u64,
}

impl Board {
    fn new(size: usize) -> Board {
        Board {
            size,
            solutions: 0,
            rows: Vec::with_capacity(size),
            columns: 0,
            forward_diagonals: 0,
            backward_diagonals: 0,
        }
    }
    // col
    // 0 1 2 3 4...

    // row
    // 0
    // 1
    // ...

    // forward diagonal
    // 0 1 2
    // 1 2 3
    // 2 3 4
    // reverse diagonal
    // 2 3 4
    // 1 2 3
    // 0 1 2
    // forward diagonal = col + row
    // reverse diagonal = (col + max_idx) - row
    // diagonals go from 0 to 2 * max_idx
    // range 0..(2 * size - 1)
    // size = 3
    // 2 * size = 6
    // 2 * size - 1 = 5
    // 0..5 = 0 1 2 3 4

}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = write![f, "["];
        if let Err(_) = result {
            return result
        }

        let height = self.rows.len(); // current length of vector
        let width = self.size;        // size of board
        if height < width {
            for i in 0..height {
                let result = write![f, "{}, ", self.rows[i]];
                if let Err(_) = result {
                    return result;
                }
            }
            for _ in height..(width - 1) {
                let result = write![f, "*, "];
                if let Err(_) = result {
                    return result;
                }
            }
            return write![f, "*]"];
        }
        else {
            for i in 0..(width - 1) {
                let result = write![f, "{}, ", self.rows[i]];
                if let Err(_) = result {
                    return result;
                }
            }
            return write![f, "{}]", self.rows[width - 1]];
        }
    }
}

fn place_queens(size: usize) -> Board {
    let mut board = Board::new(size);
    //let stack_ref_mut = &mut stack_mut;
    place_queens_internal(0, &mut board);
    return board;
}

fn place_queens_internal(row: usize, board: &mut Board) {
    for column_idx in 0..board.size as u8 {
        let column = 1u64 << column_idx;
        board.rows.truncate(row);
        board.rows.push(column_idx);
        let forward_diagonal = column << row;
        let backward_diagonal = (column << board.size) >> row;
        if ((board.columns & column) == 0)
            && ((board.forward_diagonals & forward_diagonal) == 0)
            && ((board.backward_diagonals & backward_diagonal) == 0)
        {
            if row == (board.size - 1) {
                board.solutions += 1;
                if board.solutions.is_power_of_two() {
                    println!("Hooray! {} {}", board, board.solutions);
                }
            }
            else {
                board.columns |= column;
                board.forward_diagonals |= forward_diagonal;
                board.backward_diagonals |= backward_diagonal;
                place_queens_internal(row + 1, board);
                board.columns &= !column;
                board.forward_diagonals &= !forward_diagonal;
                board.backward_diagonals &= ! backward_diagonal;
            }
        }
    }
}

fn main() {
    //println!("Hello, queens!");
    let size = 15;
    let board = place_queens(size);
    println!["Size: {}. Solutions: {}.", size, board.solutions];
}
