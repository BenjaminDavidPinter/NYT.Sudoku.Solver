mod Sudoku;

fn main() {
    let mut s_board = Sudoku::SudokuBoard::from_puzzle([
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
                            "\t\tContextual results\n\t\t\t{:?}",
                            possible_values_contextual
                        );
                        if possible_values_contextual.len() == 1 {
                            s_board.needs_solving = true;
                            println!(
                                "\tSingle value found, filling in with [{}]",
                                possible_values_contextual[0]
                            );
                            s_board.puzzle[i][j] = possible_values_contextual[0];
                        }
                    }
                }
            }
        }

        println!("\tNo valid context values found, performing single-row/single-column exclusive possible value search");
        //Get exlcusive values for rows 1/2/3 and columns 1/2/3, 6 total, per cube, 9 total cubes
        //9*6 total runs, or 54 total
        let mut top_removable_values: Vec<(usize, usize, i32)> = Default::default();
        let mut mid_removable_values: Vec<(usize, usize, i32)> = Default::default();
        let mut bot_removable_values: Vec<(usize, usize, i32)> = Default::default();
        for cube_x in 0..3 {
            //3 wide
            for cube_y in 0..3 {
                println!("\t\tPerforming search on cube {}, {}", cube_x, cube_y);
                //3 high
                let top_row = Sudoku::SudokuBoard::get_solution_row(
                    &s_board,
                    (cube_x, cube_y),
                    Sudoku::RowGroup::RowTop,
                );
                let middle_row = Sudoku::SudokuBoard::get_solution_row(
                    &s_board,
                    (cube_x, cube_y),
                    Sudoku::RowGroup::RowMiddle,
                );
                let bottom_row = Sudoku::SudokuBoard::get_solution_row(
                    &s_board,
                    (cube_x, cube_y),
                    Sudoku::RowGroup::RowBottom,
                );

                println!(
                    "\t\t\t{:?}\n\t\t\t{:?}\n\t\t\t{:?}",
                    top_row, middle_row, bottom_row
                );

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
                        println!("\t\t{}: ", possible_value);
                        println!("\t\t\tFound {} in row TOP of cube {},{}, but not in rest of cube. This value can be removed from adjacent rows",
                                                         possible_value,
                                                         cube_x,
                                                         cube_y);
                        for i in 0..9 {
                            let x = (cube_x * 3) as usize;
                            let end_y = ((cube_y * 3) + 3) as usize;
                            let start_y = ((cube_y * 3)) as usize;
                            if (i >= 0 && i < start_y) || (i >= end_y && i <= 8) {
                                bot_removable_values.push((
                                    x as usize,
                                    i as usize,
                                    possible_value,
                                ));
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
                        println!("\t\t{}: ", possible_value);
                        println!("\t\t\tFound {} in row MID of cube {},{}, but not in rest of cube. This value can be removed from adjacent rows",
                                                         possible_value,
                                                         cube_x,
                                                         cube_y);
                        for i in 0..9 {
                            let x = ((cube_x * 3) + 1) as usize;
                            let end_y = ((cube_y * 3) + 3) as usize;
                            let start_y = ((cube_y * 3)) as usize;
                            if (i >= 0 && i < start_y) || (i >= end_y && i <= 8) {
                                bot_removable_values.push((
                                    x as usize,
                                    i as usize,
                                    possible_value,
                                ));
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
                        if t_row.contains(&&possible_value)
                            || m_row.contains(&&possible_value)
                        {
                            bot_found_in_cube = true;
                            break;
                        }
                    }
                    if !bot_found_in_cube {
                        //Top row contains a value that is only valid in the top row
                        //  remove this value from other top rows in the puzzle
                        println!("\t\t{}: ", possible_value);
                        println!("\t\t\tFound {} in row BOT of cube {},{}, but not in rest of cube. This value can be removed from adjacent rows",
                                                         possible_value,
                                                         cube_x,
                                                         cube_y);
                        for i in 0..9 {
                            let x = ((cube_x * 3) + 2) as usize;
                            let end_y = ((cube_y * 3) + 3) as usize;
                            let start_y = ((cube_y * 3)) as usize;
                            if (i >= 0 && i < start_y) || (i >= end_y && i <= 8) {
                                bot_removable_values.push((
                                    x as usize,
                                    i as usize,
                                    possible_value,
                                ));
                            }
                        }
                    } else {
                        bot_found_in_cube = false;
                    }
                }
            }
        }
        println!("\t\tRemoving possible values from rows where exclusion limits them;");
        for i in 0..top_removable_values.len() {
            s_board.possible_solutions[top_removable_values[i].0][top_removable_values[i].1] =
                Sudoku::SudokuBoard::remove_possible_value_from_cell(
                    &s_board,
                    top_removable_values[i].2,
                    top_removable_values[i].0,
                    top_removable_values[i].1,
                );
        }

        for i in 0..mid_removable_values.len() {
            s_board.possible_solutions[mid_removable_values[i].0][mid_removable_values[i].1] =
                Sudoku::SudokuBoard::remove_possible_value_from_cell(
                    &s_board,
                    mid_removable_values[i].2,
                    mid_removable_values[i].0,
                    mid_removable_values[i].1,
                );
        }

        for i in 0..bot_removable_values.len() {
            s_board.possible_solutions[bot_removable_values[i].0][bot_removable_values[i].1] =
                Sudoku::SudokuBoard::remove_possible_value_from_cell(
                    &s_board,
                    bot_removable_values[i].2,
                    bot_removable_values[i].0,
                    bot_removable_values[i].1,
                );
        }

        if s_board.needs_solving {
            total_passes = total_passes + 1;
            Sudoku::SudokuBoard::print_puzzle(&s_board.puzzle);
        }
    }
    Sudoku::SudokuBoard::print_puzzle(&s_board.puzzle);
    println!("{} total passes.", total_passes);
}
