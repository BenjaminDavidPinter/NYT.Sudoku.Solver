mod Sudoku;

fn main() {
    let mut s_board = Sudoku::SudokuBoard::from_puzzle([
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
    Sudoku::SudokuBoard::print_puzzle(&s_board.puzzle);

    Sudoku::SudokuBoard::solve_deterministic(&mut s_board);
    
    if Sudoku::SudokuBoard::populated(&s_board) {
        println!("\n\nCompleted\n\n");
    } else {
        //Generate futures
        let new_board = s_board.clone();
        for i in 1..10 {
            println!();
            println!("Greedy {}s...", i);
            let attempted_puzzle = Sudoku::SudokuBoard::solve_greedy(&s_board, i, 3 as usize);
            if Sudoku::SudokuBoard::populated(&attempted_puzzle) {
                println!("\n\nCompleted\n\n");
                Sudoku::SudokuBoard::print_puzzle(&attempted_puzzle.puzzle);        
                break;
            }
        }
    }
}
