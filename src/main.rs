mod Sudoku;

fn main() {
    let mut s_board = Sudoku::SudokuBoard::new();
    s_board.puzzle = [
        [0, 0, 4, 0, 0, 0, 6, 0, 0],
        [2, 7, 0, 0, 0, 0, 0, 9, 0],
        [0, 0, 0, 2, 8, 0, 0, 0, 0],
        [0, 0, 6, 0, 9, 0, 1, 7, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 2, 0, 0, 4],
        [0, 8, 0, 3, 0, 4, 0, 0, 0],
        [9, 5, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
    ];

    let mut total_passes = 0;
    //Calculate all the valid values for each square, using basic Row/Column/Square rules.
    Sudoku::SudokuBoard::print_puzzle(&s_board.puzzle);
    s_board.needs_solving = true;
    while s_board.needs_solving {
        s_board.needs_solving = false;
        s_board.possible_solutions = Sudoku::SudokuBoard::calc_possible_solutions(&s_board);

        //Enumerate the board, applying 2 types of searches;
        //Search 1; Check cell for single valid value
        //Search 2: Check cell for only instance of value in box
        //Search 3: TODO: Check for only-valid columns and boxes and rule out other boxes by that
        for i in 0..9 {
            for j in 0..9 {
                if s_board.puzzle[i][j] == 0 {
                    println!("Working on box; {},{}", i, j);
                    println!(
                        "\tPossible values for this box\n\t\t{:?}",
                        s_board.possible_solutions[i][j]
                    );

                    if s_board.possible_solutions[i][j].len() == 1 {
                        println!(
                            "\tSingle value found, filling in with [{}]",
                            s_board.possible_solutions[i][j][0]
                        );
                        s_board.needs_solving = true;
                        s_board.puzzle[i][j] = s_board.possible_solutions[i][j][0];
                    } else {
                        println!("\tMultiple values found, attempting contextual search...");
                        let possible_values_contextual: Vec<i32> =
                            Sudoku::SudokuBoard::get_box_values(&s_board.possible_solutions, i, j);
                        println!(
                            "\tContextual results\n\t\t{:?}",
                            possible_values_contextual
                        );
                        if possible_values_contextual.len() == 1 {
                            s_board.needs_solving = true;
                            println!(
                                "\tSingle value found, filling in with [{}]",
                                possible_values_contextual[0]
                            );
                            s_board.puzzle[i][j] = possible_values_contextual[0];
                        } else {
                            
                        }
                    }
                }
            }
        }
        if s_board.needs_solving {
            total_passes = total_passes + 1;
            Sudoku::SudokuBoard::print_puzzle(&s_board.puzzle);
        }
    }
    Sudoku::SudokuBoard::print_puzzle(&s_board.puzzle);
    println!("{} total passes.", total_passes);
}
