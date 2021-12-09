mod Sudoku;

fn main() {
    let mut puzzle: [[i32; 9]; 9] = [
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

    let mut possible_values: [[Vec<i32>; 9]; 9];
    let mut needs_solving = true;
    let mut total_passes = 0;
    //Calculate all the valid values for each square, using basic Row/Column/Square rules.
    Sudoku::SudokuBoard::print_puzzle(&puzzle);
    while needs_solving {
        possible_values = Default::default();
        needs_solving = false;
        for i in 0..9 {
            for j in 0..9 {
                if puzzle[i][j] == 0 {
                    possible_values[i][j] = Sudoku::SudokuBoard::get_inverse_values(Sudoku::SudokuBoard::get_existing_values(&puzzle, i, j));
                } else {
                    possible_values[i][j] = Default::default();
                }
            }
        }

        //Enumerate the board, applying 2 types of searches;
        //Search 1; Check cell for single valid value
        //Search 2: Check cell for only instance of value in box
        //Search 3: TODO: Check for only-valid columns and boxes and rule out other boxes by that
        for i in 0..9 {
            for j in 0..9 {
                if puzzle[i][j] == 0 {
                    println!("Working on box; {},{}", i, j);
                    println!(
                        "\tPossible values for this box\n\t\t{:?}",
                        possible_values[i][j]
                    );

                    if possible_values[i][j].len() == 1 {
                        println!(
                            "\tSingle value found, filling in with [{}]",
                            possible_values[i][j][0]
                        );
                        needs_solving = true;
                        puzzle[i][j] = possible_values[i][j][0];
                    } else {
                        println!("\tMultiple values found, attempting contextual search...");
                        let possible_values_contextual: Vec<i32> =
                            Sudoku::SudokuBoard::get_contextual_values(&possible_values, i, j);
                        println!(
                            "\t\tContextual results\n\t\t\t{:?}",
                            possible_values_contextual
                        );
                        if possible_values_contextual.len() == 1 {
                            needs_solving = true;
                            println!(
                                "\tSingle value found, filling in with [{}]",
                                possible_values_contextual[0]
                            );
                            puzzle[i][j] = possible_values_contextual[0];
                        }
                    }
                }
            }
        }
        if needs_solving {
            total_passes = total_passes + 1;
            Sudoku::SudokuBoard::print_puzzle(&puzzle);
        }
    }
    Sudoku::SudokuBoard::print_puzzle(&puzzle);
    println!("{} total passes.", total_passes);
}
