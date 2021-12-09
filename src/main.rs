mod box_access;

#[derive(Default)]
pub struct SudokuBoard {
    puzzle: [[i32; 9]; 9],
    possible_solutions: [[Vec<i32>; 9]; 9],
}

impl SudokuBoard {
    pub fn new() -> SudokuBoard {
        SudokuBoard::default()
    }

    pub fn get_puzzle_row(&self, box_coord: (i32, i32), row: box_access::RowGroup) -> (i32, i32, i32) {
        let row_offset = match row {
            box_access::RowGroup::RowTop => 0,
            box_access::RowGroup::RowMiddle => 1,
            box_access::RowGroup::RowBottom => 2,
        };

        let natural_boundaries =
            get_boundaries_for_cell((box_coord.0 * 3) as usize, (box_coord.1 * 3) as usize);
        let x = natural_boundaries.0 + row_offset;
        return (
            self.puzzle[x][natural_boundaries.1],
            self.puzzle[x][natural_boundaries.1 + 1],
            self.puzzle[x][natural_boundaries.1 + 2],
        );
    }

    pub fn get_puzzle_col(&self, box_coord: (i32, i32), col: box_access::ColGroup) -> (i32, i32, i32) {
        let col_offset = match col {
            box_access::ColGroup::ColLeft => 0,
            box_access::ColGroup::ColMiddle => 1,
            box_access::ColGroup::ColRight => 2,
        };

        let natural_boundaries =
            get_boundaries_for_cell((box_coord.0 * 3) as usize, (box_coord.1 * 3) as usize);
        let y = natural_boundaries.1 + col_offset;
        return (
            self.puzzle[natural_boundaries.0][y],
            self.puzzle[natural_boundaries.0 + 1][y],
            self.puzzle[natural_boundaries.1 + 2][y],
        );
    }

    pub fn get_solution_row(
        &self,
        box_coord: (i32, i32),
        row: box_access::RowGroup,
    ) -> (&Vec<i32>, &Vec<i32>, &Vec<i32>) {
        let row_offset = match row {
            box_access::RowGroup::RowTop => 0,
            box_access::RowGroup::RowMiddle => 1,
            box_access::RowGroup::RowBottom => 2,
        };

        let natural_boundaries =
            get_boundaries_for_cell((box_coord.0 * 3) as usize, (box_coord.1 * 3) as usize);
        let x = natural_boundaries.0 + row_offset;
        return (
            &self.possible_solutions[x][natural_boundaries.1],
            &self.possible_solutions[x][natural_boundaries.1 + 1],
            &self.possible_solutions[x][natural_boundaries.1 + 2],
        );
    }

    pub fn get_solution_col(
        &self,
        box_coord: (i32, i32),
        col: box_access::ColGroup,
    ) -> (&Vec<i32>, &Vec<i32>, &Vec<i32>) {
        let col_offset = match col {
            box_access::ColGroup::ColLeft => 0,
            box_access::ColGroup::ColMiddle => 1,
            box_access::ColGroup::ColRight => 2,
        };

        let natural_boundaries =
            get_boundaries_for_cell((box_coord.0 * 3) as usize, (box_coord.1 * 3) as usize);
        let y = natural_boundaries.1 + col_offset;
        return (
            &self.possible_solutions[natural_boundaries.0][y],
            &self.possible_solutions[natural_boundaries.0 + 1][y],
            &self.possible_solutions[natural_boundaries.1 + 2][y],
        );
    }
}

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
    print_puzzle(&puzzle);
    while needs_solving {
        possible_values = Default::default();
        needs_solving = false;
        for i in 0..9 {
            for j in 0..9 {
                if puzzle[i][j] == 0 {
                    possible_values[i][j] = get_inverse_values(get_existing_values(&puzzle, i, j));
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
                            get_contextual_values(&possible_values, i, j);
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
            print_puzzle(&puzzle);
        }
    }
    print_puzzle(&puzzle);
    println!("{} total passes.", total_passes);
}

fn print_puzzle(puzzle: &[[i32; 9]; 9]) {
    for row in 0..puzzle.len() {
        if row % 3 == 0 {
            print_dash_line();
        }
        let arr_size = puzzle[row].len();
        for item in 0..arr_size {
            if item % 3 == 0 {
                print!("| ");
            }

            let cell_value = puzzle[row][item];
            if cell_value == 0 {
                print!("  ");
            } else {
                print!("{} ", puzzle[row][item]);
            }

            if item == arr_size - 1 {
                println!("|")
            }
        }
    }
    print_dash_line();
}

fn print_dash_line() {
    let mut counter = 0;
    while counter < 25 {
        print!("-");
        counter = counter + 1;
    }
    println!();
}

pub fn get_existing_values(puzzle: &[[i32; 9]; 9], row: usize, col: usize) -> Vec<i32> {
    let mut values = Vec::new();
    for i in 0..9 {
        let row_val = puzzle[row][i];
        let col_val = puzzle[i][col];
        if row_val >= 1 {
            values.push(row_val);
        }
        if col_val >= 1 {
            values.push(col_val);
        }
    }

    let xy_boundary = get_boundaries_for_cell(row, col);

    for i in 0..3 {
        for j in 0..3 {
            let box_val = puzzle[i + xy_boundary.0][j + xy_boundary.1];
            if box_val >= 1 {
                values.push(box_val);
            }
        }
    }

    values.sort_unstable();
    values.dedup();

    return values;
}

pub fn get_inverse_values(values: Vec<i32>) -> Vec<i32> {
    let mut ret_values = Vec::new();

    for i in 1..10 {
        let mut i_found = false;
        for elem in &values {
            if elem == &(i) {
                i_found = true;
            }
        }
        if !i_found {
            ret_values.push(i);
        }
    }
    return ret_values;
}

pub fn get_contextual_values(
    possible_values: &[[Vec<i32>; 9]; 9],
    row: usize,
    col: usize,
) -> Vec<i32> {
    let mut ret_vals: Vec<i32> = Default::default();

    let xy_boundary = get_boundaries_for_cell(row, col);
    println!("\tBox boundary {},{}", xy_boundary.0, xy_boundary.1);

    for elem in &possible_values[row][col] {
        println!("\t\t{} : ", &elem);
        let mut valid_value = true;
        for i in 0..3 {
            for j in 0..3 {
                let box_vals = &possible_values[i + xy_boundary.0][j + xy_boundary.1];
                println!(
                    "\t\t\t{}, {} Valid Values: {:?}",
                    i + xy_boundary.0,
                    j + xy_boundary.1,
                    box_vals
                );
                if box_vals.contains(&elem)
                    && !(i + xy_boundary.0 == row && j + xy_boundary.1 == col)
                {
                    println!("\t\t\tFound in {},{}", i + xy_boundary.0, j + xy_boundary.1);
                    valid_value = false;
                }
                if !valid_value {
                    break;
                }
            }
            if !valid_value {
                break;
            }
        }
        if valid_value {
            println!(" Not found, adding");
            ret_vals.push(*elem);
        }
    }

    ret_vals
}

pub fn get_boundaries_for_cell(row: usize, col: usize) -> (usize, usize) {
    let mut row_boundary: usize = 0;
    let mut col_boundary: usize = 0;

    if row < 3 {
        row_boundary = 0;
    }
    if row >= 3 && row < 6 {
        row_boundary = 3;
    }
    if row >= 6 {
        row_boundary = 6;
    }

    if col < 3 {
        col_boundary = 0;
    }
    if col >= 3 && col < 6 {
        col_boundary = 3;
    }
    if col >= 6 {
        col_boundary = 6;
    }

    return (row_boundary, col_boundary);
}

#[test]
fn test_top_row_00() {
    let mut board: SudokuBoard = SudokuBoard::new();
    board.puzzle = [
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

    let test_row = board.get_puzzle_row((0, 0), box_access::RowGroup::RowTop);
    assert_eq!(test_row.0, 0);
    assert_eq!(test_row.1, 0);
    assert_eq!(test_row.2, 4);
}

#[test]
fn test_mid_row_00() {
    let mut board: SudokuBoard = SudokuBoard::new();
    board.puzzle = [
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

    let test_row = board.get_puzzle_row((0, 0), box_access::RowGroup::RowMiddle);
    assert_eq!(test_row.0, 2);
    assert_eq!(test_row.1, 7);
    assert_eq!(test_row.2, 0);
}

#[test]
fn test_bot_row_00() {
    let mut board: SudokuBoard = SudokuBoard::new();
    board.puzzle = [
        [0, 0, 4, 0, 0, 0, 6, 0, 0],
        [2, 7, 0, 0, 0, 0, 0, 9, 0],
        [1, 2, 3, 2, 8, 0, 0, 0, 0],
        [0, 0, 6, 0, 9, 0, 1, 7, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 2, 0, 0, 4],
        [0, 8, 0, 3, 0, 4, 0, 0, 0],
        [9, 5, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
    ];

    let test_row = board.get_puzzle_row((0, 0), box_access::RowGroup::RowBottom);
    assert_eq!(test_row.0, 1);
    assert_eq!(test_row.1, 2);
    assert_eq!(test_row.2, 3);
}

#[test]
fn test_top_row_11() {
    let mut board: SudokuBoard = SudokuBoard::new();
    board.puzzle = [
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

    let test_row = board.get_puzzle_row((1, 1), box_access::RowGroup::RowTop);
    assert_eq!(test_row.0, 0);
    assert_eq!(test_row.1, 9);
    assert_eq!(test_row.2, 0);
}

#[test]
fn test_mid_row_11() {
    let mut board: SudokuBoard = SudokuBoard::new();
    board.puzzle = [
        [0, 0, 4, 0, 0, 0, 6, 0, 0],
        [2, 7, 0, 0, 0, 0, 0, 9, 0],
        [0, 0, 0, 2, 8, 0, 0, 0, 0],
        [0, 0, 6, 0, 9, 0, 1, 7, 0],
        [4, 0, 0, 0, 5, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 2, 0, 0, 4],
        [0, 8, 0, 3, 0, 4, 0, 0, 0],
        [9, 5, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
    ];

    let test_row = board.get_puzzle_row((1, 1), box_access::RowGroup::RowMiddle);
    assert_eq!(test_row.0, 0);
    assert_eq!(test_row.1, 5);
    assert_eq!(test_row.2, 0);
}

#[test]
fn test_bot_row_11() {
    let mut board: SudokuBoard = SudokuBoard::new();
    board.puzzle = [
        [0, 0, 4, 0, 0, 0, 6, 0, 0],
        [2, 7, 0, 0, 0, 0, 0, 9, 0],
        [1, 2, 3, 2, 8, 0, 0, 0, 0],
        [0, 0, 6, 0, 9, 0, 1, 7, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 2, 0, 0, 4],
        [0, 8, 0, 3, 0, 4, 0, 0, 0],
        [9, 5, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
    ];

    let test_row = board.get_puzzle_row((1, 1), box_access::RowGroup::RowBottom);
    assert_eq!(test_row.0, 0);
    assert_eq!(test_row.1, 7);
    assert_eq!(test_row.2, 2);
}

#[test]
fn test_top_row_22() {
    let mut board: SudokuBoard = SudokuBoard::new();
    board.puzzle = [
        [0, 0, 4, 0, 0, 0, 6, 0, 0],
        [2, 7, 0, 0, 0, 0, 0, 9, 0],
        [0, 0, 0, 2, 8, 0, 0, 0, 0],
        [0, 0, 6, 0, 9, 0, 1, 7, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 2, 0, 0, 4],
        [0, 8, 0, 3, 0, 4, 1, 2, 3],
        [9, 5, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
    ];

    let test_row = board.get_puzzle_row((2,2), box_access::RowGroup::RowTop);
    assert_eq!(test_row.0, 1);
    assert_eq!(test_row.1, 2);
    assert_eq!(test_row.2, 3);
}

#[test]
fn test_mid_row_22() {
    let mut board: SudokuBoard = SudokuBoard::new();
    board.puzzle = [
        [0, 0, 4, 0, 0, 0, 6, 0, 0],
        [2, 7, 0, 0, 0, 0, 0, 9, 0],
        [0, 0, 0, 2, 8, 0, 0, 0, 0],
        [0, 0, 6, 0, 9, 0, 1, 7, 0],
        [4, 0, 0, 0, 5, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 2, 0, 0, 4],
        [0, 8, 0, 3, 0, 4, 0, 0, 0],
        [9, 5, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
    ];

    let test_row = board.get_puzzle_row((2,2), box_access::RowGroup::RowMiddle);
    assert_eq!(test_row.0, 0);
    assert_eq!(test_row.1, 8);
    assert_eq!(test_row.2, 0);
}

#[test]
fn test_bot_row_22() {
    let mut board: SudokuBoard = SudokuBoard::new();
    board.puzzle = [
        [0, 0, 4, 0, 0, 0, 6, 0, 0],
        [2, 7, 0, 0, 0, 0, 0, 9, 0],
        [1, 2, 3, 2, 8, 0, 0, 0, 0],
        [0, 0, 6, 0, 9, 0, 1, 7, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 2, 0, 0, 4],
        [0, 8, 0, 3, 0, 4, 0, 0, 0],
        [9, 5, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 0, 1, 0, 0, 5, 0],
    ];

    let test_row = board.get_puzzle_row((2,2), box_access::RowGroup::RowBottom);
    assert_eq!(test_row.0, 0);
    assert_eq!(test_row.1, 5);
    assert_eq!(test_row.2, 0);
}

#[test]
fn test_top_row_00_solution_access() {
    let mut board: SudokuBoard = SudokuBoard::new();
    board.puzzle = [
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    for i in 0..9 {
        for j in 0..9 {
            if board.puzzle[i][j] == 0 {
                board.possible_solutions[i][j] = get_inverse_values(get_existing_values(&board.puzzle, i, j));
            } else {
                board.possible_solutions[i][j] = Default::default();
            }
        }
    }
    
    let test_row = board.get_solution_row((0,0), box_access::RowGroup::RowTop);
    assert_eq!(test_row.0.len(), 0);
    assert_eq!(test_row.1.len(), 8);
    assert_eq!(test_row.2.len(), 8);
}
