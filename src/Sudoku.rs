pub enum RowGroup {
    RowTop,
    RowMiddle,
    RowBottom,
}

pub enum ColGroup {
    ColLeft,
    ColMiddle,
    ColRight,
}

#[derive(Default)]
pub struct SudokuBoard {
    pub puzzle: [[i32; 9]; 9],
    pub possible_solutions: [[Vec<i32>; 9]; 9],
    pub needs_solving: bool,
}

impl SudokuBoard {
    pub fn new() -> SudokuBoard {
        SudokuBoard::default()
    }

    pub fn from_puzzle(puzzle: [[i32; 9]; 9]) -> SudokuBoard {
        let mut new_board = SudokuBoard::new();
        new_board.puzzle = puzzle;
        new_board.possible_solutions = SudokuBoard::calc_possible_solutions(&new_board);
        new_board
    }

    pub fn calc_possible_solutions(board: &SudokuBoard) -> [[Vec<i32>; 9]; 9] {
        let mut ret_value: [[Vec<i32>; 9]; 9] = Default::default();
        for i in 0..9 {
            for j in 0..9 {
                if board.puzzle[i][j] == 0 {
                    ret_value[i][j] = SudokuBoard::get_inverse_values(
                        SudokuBoard::get_existing_values(&board.puzzle, i, j),
                    );
                } else {
                    ret_value[i][j] = Default::default();
                }
            }
        }
        ret_value
    }

    pub fn get_puzzle_row(
        board: &SudokuBoard,
        box_coord: (i32, i32),
        row: RowGroup,
    ) -> (i32, i32, i32) {
        let row_offset = match row {
            RowGroup::RowTop => 0,
            RowGroup::RowMiddle => 1,
            RowGroup::RowBottom => 2,
        };

        let natural_boundaries = SudokuBoard::get_boundaries_for_cell(
            (box_coord.0 * 3) as usize,
            (box_coord.1 * 3) as usize,
        );
        let x = natural_boundaries.0 + row_offset;
        return (
            board.puzzle[x][natural_boundaries.1],
            board.puzzle[x][natural_boundaries.1 + 1],
            board.puzzle[x][natural_boundaries.1 + 2],
        );
    }

    pub fn get_puzzle_col(&self, box_coord: (i32, i32), col: ColGroup) -> (i32, i32, i32) {
        let col_offset = match col {
            ColGroup::ColLeft => 0,
            ColGroup::ColMiddle => 1,
            ColGroup::ColRight => 2,
        };

        let natural_boundaries = SudokuBoard::get_boundaries_for_cell(
            (box_coord.0 * 3) as usize,
            (box_coord.1 * 3) as usize,
        );
        let y = natural_boundaries.1 + col_offset;
        return (
            self.puzzle[natural_boundaries.0][y],
            self.puzzle[natural_boundaries.0 + 1][y],
            self.puzzle[natural_boundaries.1 + 2][y],
        );
    }

    pub fn get_solution_row(&self, box_coord: (i32, i32), row: RowGroup) -> Vec<&Vec<i32>> {
        let row_offset = match row {
            RowGroup::RowTop => 0,
            RowGroup::RowMiddle => 1,
            RowGroup::RowBottom => 2,
        };

        let natural_boundaries = SudokuBoard::get_boundaries_for_cell(
            (box_coord.0 * 3) as usize,
            (box_coord.1 * 3) as usize,
        );
        let x = natural_boundaries.0 + row_offset;
        let mut ret_vec: Vec<&Vec<i32>> = Default::default();
        ret_vec.push(&self.possible_solutions[x][natural_boundaries.1]);
        ret_vec.push(&self.possible_solutions[x][natural_boundaries.1 + 1]);
        ret_vec.push(&self.possible_solutions[x][natural_boundaries.1 + 2]);

        ret_vec
    }

    pub fn get_solution_col(
        &self,
        box_coord: (i32, i32),
        col: ColGroup,
    ) -> (&Vec<i32>, &Vec<i32>, &Vec<i32>) {
        let col_offset = match col {
            ColGroup::ColLeft => 0,
            ColGroup::ColMiddle => 1,
            ColGroup::ColRight => 2,
        };

        let natural_boundaries = SudokuBoard::get_boundaries_for_cell(
            (box_coord.0 * 3) as usize,
            (box_coord.1 * 3) as usize,
        );
        let y = natural_boundaries.1 + col_offset;
        return (
            &self.possible_solutions[natural_boundaries.0][y],
            &self.possible_solutions[natural_boundaries.0 + 1][y],
            &self.possible_solutions[natural_boundaries.1 + 2][y],
        );
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

    pub fn print_puzzle(puzzle: &[[i32; 9]; 9]) {
        for row in 0..puzzle.len() {
            if row % 3 == 0 {
                SudokuBoard::print_dash_line();
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
        SudokuBoard::print_dash_line();
    }

    pub fn print_dash_line() {
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

        let xy_boundary = SudokuBoard::get_boundaries_for_cell(row, col);

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

    pub fn get_box_values(
        possible_values: &[[Vec<i32>; 9]; 9],
        row: usize,
        col: usize,
    ) -> Vec<i32> {
        let mut ret_vals: Vec<i32> = Default::default();

        let xy_boundary = SudokuBoard::get_boundaries_for_cell(row, col);
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

    pub fn remove_possible_value_from_cell(
        board: &SudokuBoard,
        possible_value: i32,
        row: usize,
        col: usize,
    ) -> Vec<i32> {
        let mut new_vec: Vec<i32> = Default::default();
        for value in 0..9 {
            if board.possible_solutions[row][col].contains(&value) && value != possible_value {
                new_vec.push(value);
            }
        }

        new_vec
    }
}

#[test]
pub fn test_top_row_00() {
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

    let test_row = SudokuBoard::get_puzzle_row(&board, (0, 0), RowGroup::RowTop);
    assert_eq!(test_row.0, 0);
    assert_eq!(test_row.1, 0);
    assert_eq!(test_row.2, 4);
}

#[test]
pub fn test_mid_row_00() {
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

    let test_row = SudokuBoard::get_puzzle_row(&board, (0, 0), RowGroup::RowMiddle);
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

    let test_row = SudokuBoard::get_puzzle_row(&board, (0, 0), RowGroup::RowBottom);
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

    let test_row = SudokuBoard::get_puzzle_row(&board, (1, 1), RowGroup::RowTop);
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

    let test_row = SudokuBoard::get_puzzle_row(&board, (1, 1), RowGroup::RowMiddle);
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

    let test_row = SudokuBoard::get_puzzle_row(&board, (1, 1), RowGroup::RowBottom);
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

    let test_row = SudokuBoard::get_puzzle_row(&board, (2, 2), RowGroup::RowTop);
    assert_eq!(test_row.0, 1);
    assert_eq!(test_row.1, 2);
    assert_eq!(test_row.2, 3);
}

#[test]
fn test_mid_row_22() {
    let mut board = SudokuBoard::new();
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

    let test_row = SudokuBoard::get_puzzle_row(&board, (2, 2), RowGroup::RowMiddle);
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

    let test_row = SudokuBoard::get_puzzle_row(&board, (2, 2), RowGroup::RowBottom);
    assert_eq!(test_row.0, 0);
    assert_eq!(test_row.1, 5);
    assert_eq!(test_row.2, 0);
}

#[test]
fn test_top_row_00_solution_access() {
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
    for i in 0..9 {
        for j in 0..9 {
            if board.puzzle[i][j] == 0 {
                board.possible_solutions[i][j] = SudokuBoard::get_inverse_values(
                    SudokuBoard::get_existing_values(&board.puzzle, i, j),
                );
            } else {
                board.possible_solutions[i][j] = Default::default();
            }
        }
    }

    let test_row = board.get_solution_row((0, 0), RowGroup::RowTop);
    assert_eq!(test_row[0].len(), 4);
    assert_eq!(test_row[1].len(), 3);
    assert_eq!(test_row[2].len(), 0);
}

#[test]
fn test_removal_from_cell() {
    let mut board: SudokuBoard = SudokuBoard::from_puzzle([
        [0, 0, 4, 0, 0, 0, 6, 0, 0],
        [2, 7, 0, 0, 0, 0, 0, 9, 0],
        [0, 0, 0, 2, 8, 0, 0, 0, 0],
        [0, 0, 6, 0, 9, 0, 1, 7, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 2, 0, 0, 4],
        [0, 8, 0, 3, 0, 4, 0, 0, 0],
        [9, 5, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
    ]);

    assert_eq!(board.possible_solutions[0][0].contains(&1), true);
    assert_eq!(board.possible_solutions[0][0].contains(&3), true);
    assert_eq!(board.possible_solutions[0][0].contains(&5), true);
    assert_eq!(board.possible_solutions[0][0].contains(&8), true);

    board.possible_solutions[0][0] = SudokuBoard::remove_possible_value_from_cell(&board, 1, 0, 0);
    assert_eq!(board.possible_solutions[0][0].contains(&1), false);
    assert_eq!(board.possible_solutions[0][0].contains(&3), true);
    assert_eq!(board.possible_solutions[0][0].contains(&5), true);
    assert_eq!(board.possible_solutions[0][0].contains(&8), true);
}
