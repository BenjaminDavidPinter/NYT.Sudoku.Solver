use std::clone::Clone;

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

#[derive(Default, Clone)]
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

    pub fn get_solution_col(&self, box_coord: (i32, i32), col: ColGroup) -> Vec<&Vec<i32>> {
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
        let mut ret_vec: Vec<&Vec<i32>> = Default::default();
        ret_vec.push(&self.possible_solutions[natural_boundaries.0][y]);
        ret_vec.push(&self.possible_solutions[natural_boundaries.0 + 1][y]);
        ret_vec.push(&self.possible_solutions[natural_boundaries.0 + 2][y]);

        ret_vec
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
        //println!("\tBox boundary {},{}", xy_boundary.0, xy_boundary.1);

        for elem in &possible_values[row][col] {
            //println!("\t\t{} : ", &elem);
            let mut valid_value = true;
            for i in 0..3 {
                for j in 0..3 {
                    let box_vals = &possible_values[i + xy_boundary.0][j + xy_boundary.1];
                    //println!(
                    //"\t\t\t{}, {} Valid Values: {:?}",
                    //i + xy_boundary.0,
                    // j + xy_boundary.1,
                    // box_vals
                    //);
                    if box_vals.contains(&elem)
                        && !(i + xy_boundary.0 == row && j + xy_boundary.1 == col)
                    {
                        //println!("\t\t\tFound in {},{}", i + xy_boundary.0, j + xy_boundary.1);
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
                //println!(" Not found, adding");
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
        for value in 0..10 {
            if board.possible_solutions[row][col].contains(&value) && value != possible_value {
                new_vec.push(value);
            }
        }

        new_vec
    }

    pub fn solve_deterministic(s_board: &mut SudokuBoard) {
        let mut total_passes = 0;
        //Calculate all the valid values for each square, using basic Row/Column/Square rules.
        s_board.needs_solving = true;
        while s_board.needs_solving {
            s_board.needs_solving = false;
            s_board.possible_solutions = SudokuBoard::calc_possible_solutions(&s_board);
            //println!("\tPerforming single-row/single-column exclusive possible value search");
            //Get exlcusive values for rows 1/2/3 and columns 1/2/3, 6 total, per cube, 9 total cubes
            //9*6 total runs, or 54 total
            let mut removable_vals: Vec<(usize, usize, i32)> = Default::default();
            for cube_x in 0..3 {
                //3 wide
                for cube_y in 0..3 {
                    //println!("\t\tPerforming search on cube {}, {}", cube_x, cube_y);
                    //3 high
                    let top_row =
                        SudokuBoard::get_solution_row(&s_board, (cube_x, cube_y), RowGroup::RowTop);
                    let middle_row = SudokuBoard::get_solution_row(
                        &s_board,
                        (cube_x, cube_y),
                        RowGroup::RowMiddle,
                    );
                    let bottom_row = SudokuBoard::get_solution_row(
                        &s_board,
                        (cube_x, cube_y),
                        RowGroup::RowBottom,
                    );
                    let left_col = SudokuBoard::get_solution_col(
                        &s_board,
                        (cube_x, cube_y),
                        ColGroup::ColLeft,
                    );
                    let mid_col = SudokuBoard::get_solution_col(
                        &s_board,
                        (cube_x, cube_y),
                        ColGroup::ColMiddle,
                    );
                    let right_col = SudokuBoard::get_solution_col(
                        &s_board,
                        (cube_x, cube_y),
                        ColGroup::ColRight,
                    );
                    //println!(
                    //"\t\t\t{:?}\n\t\t\t{:?}\n\t\t\t{:?}",
                    // top_row, middle_row, bottom_row
                    //);
                    //Top Row
                    let mut top_found_in_cube;
                    let mut t_row: Vec<&i32> = top_row.into_iter().flatten().collect();
                    t_row.sort_unstable();
                    t_row.dedup();
                    for possible_value in &t_row {
                        top_found_in_cube = false;
                        let possible_value = **possible_value as i32;
                        for cell_index in 0..3 {
                            if middle_row[cell_index].contains(&possible_value)
                                || bottom_row[cell_index].contains(&possible_value)
                            {
                                top_found_in_cube = true;
                                break;
                            }
                        }
                        if !top_found_in_cube {
                            //Top row contains a value that is only valid in the top row
                            //  remove this value from other top rows in the puzzle
                            //println!("\t\t{}: ", possible_value);
                            //println!("\t\t\tFound {} in row TOP of cube {},{}, but not in rest of cube. This value can be removed from adjacent rows",
                            //                             possible_value,
                            //                              cube_x,
                            //                               cube_y);
                            for i in 0..9 {
                                let x = (cube_x * 3) as usize;
                                let end_y = ((cube_y * 3) + 3) as usize;
                                let start_y = (cube_y * 3) as usize;
                                if (i >= 0 && i < start_y) || (i >= end_y && i <= 8) {
                                    removable_vals.push((x as usize, i as usize, possible_value));
                                }
                            }
                        } else {
                            top_found_in_cube = false;
                        }
                    }
                    //Mid row
                    let mut mid_found_in_cube;
                    let mut m_row: Vec<&i32> = middle_row.into_iter().flatten().collect();
                    m_row.sort_unstable();
                    m_row.dedup();
                    for possible_value in &m_row {
                        mid_found_in_cube = false;
                        let possible_value = **possible_value as i32;
                        for cell_index in 0..3 {
                            if t_row.contains(&&possible_value)
                                || bottom_row[cell_index].contains(&possible_value)
                            {
                                mid_found_in_cube = true;
                                break;
                            }
                        }
                        if !mid_found_in_cube {
                            //Top row contains a value that is only valid in the top row
                            //  remove this value from other top rows in the puzzle
                            //println!("\t\t{}: ", possible_value);
                            //println!("\t\t\tFound {} in row MID of cube {},{}, but not in rest of cube. This value can be removed from adjacent rows",
                            //                                 possible_value,
                            //                                cube_x,
                            //                                cube_y);
                            for i in 0..9 {
                                let x = ((cube_x * 3) + 1) as usize;
                                let end_y = ((cube_y * 3) + 3) as usize;
                                let start_y = (cube_y * 3) as usize;
                                if (i >= 0 && i < start_y) || (i >= end_y && i <= 8) {
                                    removable_vals.push((x as usize, i as usize, possible_value));
                                }
                            }
                        } else {
                            mid_found_in_cube = false;
                        }
                    }
                    //Bot row
                    let mut bot_found_in_cube;
                    let mut b_row: Vec<&i32> = bottom_row.into_iter().flatten().collect();
                    b_row.sort_unstable();
                    b_row.dedup();
                    for possible_value in &b_row {
                        bot_found_in_cube = false;
                        let possible_value = **possible_value as i32;
                        for cell_index in 0..3 {
                            if t_row.contains(&&possible_value) || m_row.contains(&&possible_value)
                            {
                                bot_found_in_cube = true;
                                break;
                            }
                        }
                        if !bot_found_in_cube {
                            //Top row contains a value that is only valid in the top row
                            //  remove this value from other top rows in the puzzle
                            //println!("\t\t{}: ", possible_value);
                            //println!("\t\t\tFound {} in row BOT of cube {},{}, but not in rest of cube. This value can be removed from adjacent rows",
                            //                                possible_value,
                            //                                cube_x,
                            //                                cube_y);
                            for i in 0..9 {
                                let x = ((cube_x * 3) + 2) as usize;
                                let end_y = ((cube_y * 3) + 3) as usize;
                                let start_y = (cube_y * 3) as usize;
                                if (i >= 0 && i < start_y) || (i >= end_y && i <= 8) {
                                    removable_vals.push((x as usize, i as usize, possible_value));
                                }
                            }
                        } else {
                            bot_found_in_cube = false;
                        }
                    }
                    //left col
                    let mut left_found_in_cube;
                    let mut l_col: Vec<&i32> = left_col.into_iter().flatten().collect();
                    l_col.sort_unstable();
                    l_col.dedup();
                    for possible_value in &l_col {
                        let possible_value = **possible_value;
                        left_found_in_cube = false;
                        for cell_index in 0..3 {
                            if right_col[cell_index].contains(&possible_value)
                                || mid_col[cell_index].contains(&possible_value)
                            {
                                left_found_in_cube = true;
                                break;
                            }
                        }
                        if !left_found_in_cube {
                            //Top row contains a value that is only valid in the top row
                            //  remove this value from other top rows in the puzzle
                            //println!("\t\t{}: ", possible_value);
                            //println!("\t\t\tFound {} in col LEFT of cube {},{}, but not in rest of cube. This value can be removed from adjacent rows",
                            //                                possible_value,
                            //                                cube_x,
                            //                                cube_y);
                            for i in 0..9 {
                                let y = (cube_y * 3) as usize;
                                let end_x = ((cube_x * 3) + 3) as usize;
                                let start_x = (cube_x * 3) as usize;
                                if (i >= 0 && i < start_x) || (i >= end_x && i <= 8) {
                                    removable_vals.push((i as usize, y as usize, possible_value));
                                }
                            }
                        } else {
                            left_found_in_cube = false;
                        }
                    }
                    //right col
                    let mut right_found_in_cube;
                    let mut r_col: Vec<&i32> = right_col.into_iter().flatten().collect();
                    r_col.sort_unstable();
                    r_col.dedup();
                    for possible_value in &r_col {
                        let possible_value = **possible_value;
                        right_found_in_cube = false;
                        for cell_index in 0..3 {
                            if l_col.contains(&&possible_value)
                                || mid_col[cell_index].contains(&possible_value)
                            {
                                right_found_in_cube = true;
                                break;
                            }
                        }
                        if !right_found_in_cube {
                            //Top row contains a value that is only valid in the top row
                            //  remove this value from other top rows in the puzzle
                            //println!("\t\t{}: ", possible_value);
                            //println!("\t\t\tFound {} in col RIGHT of cube {},{}, but not in rest of cube. This value can be removed from adjacent rows",
                            //                               possible_value,
                            //                               cube_x,
                            //                                cube_y);
                            //cube_x = 1
                            //cube_y = 0
                            //y == 2
                            for i in 0..9 {
                                let y = ((cube_y * 3) + 2) as usize;
                                let end_x = ((cube_x * 3) + 3) as usize;
                                let start_x = (cube_x * 3) as usize;
                                if (i >= 0 && i < start_x) || (i >= end_x && i <= 8) {
                                    removable_vals.push((i as usize, y as usize, possible_value));
                                }
                            }
                        } else {
                            right_found_in_cube = false;
                        }
                    }
                    //mid col
                    let mut midc_found_in_cube;
                    let mut m_col: Vec<&i32> = mid_col.into_iter().flatten().collect();
                    m_col.sort_unstable();
                    m_col.dedup();
                    for possible_value in &m_col {
                        let possible_value = **possible_value;
                        midc_found_in_cube = false;
                        for cell_index in 0..3 {
                            if l_col.contains(&&possible_value) || r_col.contains(&&possible_value)
                            {
                                midc_found_in_cube = true;
                                break;
                            }
                        }
                        if !midc_found_in_cube {
                            //Top row contains a value that is only valid in the top row
                            //  remove this value from other top rows in the puzzle
                            //println!("\t\t{}: ", possible_value);
                            //println!("\t\t\tFound {} in col MID of cube {},{}, but not in rest of cube. This value can be removed from adjacent rows",
                            //                               possible_value,
                            //                                cube_x,
                            //                                cube_y);
                            //cube_x = 1
                            //cube_y = 0
                            //y == 2
                            for i in 0..9 {
                                let y = ((cube_y * 3) + 1) as usize;
                                let end_x = ((cube_x * 3) + 3) as usize;
                                let start_x = (cube_x * 3) as usize;
                                if (i >= 0 && i < start_x) || (i >= end_x && i <= 8) {
                                    removable_vals.push((i as usize, y as usize, possible_value));
                                }
                            }
                        } else {
                            midc_found_in_cube = false;
                        }
                    }
                }
            }
            //println!("\t\tRemoving possible values from rows where exclusion limits them;");
            for i in 0..removable_vals.len() {
                //println!(
                // "\t\t\tRemoving {} from {},{}",
                //  removable_vals[i].2, removable_vals[i].0, removable_vals[i].1
                //);
                s_board.possible_solutions[removable_vals[i].0][removable_vals[i].1] =
                    SudokuBoard::remove_possible_value_from_cell(
                        &s_board,
                        removable_vals[i].2,
                        removable_vals[i].0,
                        removable_vals[i].1,
                    );
            }
            //Enumerate the board, applying 2 types of searches;
            //Search 1; Check cell for single valid value
            //Search 2: Check cell for only instance of value in box
            //Search 3: TODO: Check for only-valid columns and boxes and rule out other boxes by that
            for i in 0..9 {
                for j in 0..9 {
                    if s_board.puzzle[i][j] == 0 {
                        //println!("Working on box; {},{}", i, j);
                        //println!(
                        // "\tPossible values for this box\n\t\t{:?}",
                        //   s_board.possible_solutions[i][j]
                        //);
                        if s_board.possible_solutions[i][j].len() == 1 {
                            //println!(
                            //  "\tSingle value found, filling in with [{}]",
                            //  s_board.possible_solutions[i][j][0]
                            //);
                            s_board.needs_solving = true;
                            s_board.puzzle[i][j] = s_board.possible_solutions[i][j][0];
                        } else {
                            //println!("\tMultiple values found, attempting contextual search...");
                            let possible_values_contextual: Vec<i32> =
                                SudokuBoard::get_box_values(&s_board.possible_solutions, i, j);
                            //println!(
                            // "\t\tContextual results\n\t\t\t{:?}",
                            // possible_values_contextual
                            //);
                            if possible_values_contextual.len() == 1 {
                                s_board.needs_solving = true;
                                //println!(
                                //  "\tSingle value found, filling in with [{}]",
                                //    possible_values_contextual[0]
                                //);
                                s_board.puzzle[i][j] = possible_values_contextual[0];
                            }
                        }
                    }
                }
            }
            if s_board.needs_solving {
                total_passes = total_passes + 1;
            }
        }
    }

    pub fn solve_greedy(
        s_board: &SudokuBoard,
        greedy_number: i32,
        greed_level: usize,
    ) -> SudokuBoard {
        let mut solved = false;
        for i in 0..9 {
            for j in 0..9 {
                if s_board.possible_solutions[i][j].len() <= greed_level
                    && !solved
                    && s_board.possible_solutions[i][j].contains(&greedy_number)
                {
                    let mut test_board = s_board.clone();
                    test_board.puzzle[i][j] = greedy_number;
                    SudokuBoard::solve_deterministic(&mut test_board);
                    if SudokuBoard::populated(&test_board) {
                        solved = true;
                        return test_board;
                    }
                }
            }
        }
        return s_board.clone();
    }

    pub fn populated(s_board: &SudokuBoard) -> bool {
        for row in s_board.puzzle {
            if row.contains(&0) {
                return false;
            }
        }
        return true;
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
fn test_left_col_00_solution_access() {
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

    let test_row = board.get_solution_col((0, 0), ColGroup::ColLeft);
    assert_eq!(test_row[0].len(), 4);
    assert_eq!(test_row[1].len(), 0);
    assert_eq!(test_row[2].len(), 4);
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
