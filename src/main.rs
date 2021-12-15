mod sudoku;

fn main() {
    let mut s_board = sudoku::SudokuBoard::from_puzzle([
        [0, 3, 0, 8, 0, 7, 0, 0, 5],
        [0, 0, 0, 0, 0, 5, 0, 0, 3],
        [0, 0, 0, 6, 0, 0, 1, 0, 0],
        [6, 0, 0, 4, 0, 0, 2, 0, 0],
        [2, 0, 0, 0, 0, 0, 4, 8, 9],
        [0, 8, 0, 0, 0, 0, 0, 3, 0],
        [0, 0, 2, 7, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 6, 0, 0, 0],
        [0, 9, 7, 0, 0, 0, 0, 4, 2],
    ]);
    sudoku::SudokuBoard::print_puzzle(&s_board.puzzle);

    sudoku::SudokuBoard::solve_deterministic(&mut s_board);

    if sudoku::SudokuBoard::populated(&s_board) && sudoku::SudokuBoard::validate_board(&s_board) {
        println!("\n\nCompleted\n\n");
    } else {
        for i in 1..10 {
            let attempted_puzzle = sudoku::SudokuBoard::solve_greedy(&s_board, i, 3 as usize);
            if sudoku::SudokuBoard::populated(&attempted_puzzle)
                && sudoku::SudokuBoard::validate_board(&attempted_puzzle)
            {
                println!("\n\nCompleted & Validated\n");
                sudoku::SudokuBoard::print_puzzle(&attempted_puzzle.puzzle);
                break;
            }
        }
    }
}
