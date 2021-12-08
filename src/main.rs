fn main() {
    let mut puzzle: [[i32; 9]; 9] = [
        [0, 0, 2, 0, 0, 0, 0, 8, 4],
        [0, 0, 1, 6, 0, 0, 0, 0, 7],
        [5, 4, 9, 8, 2, 7, 0, 1, 3],
        [0, 1, 5, 0, 0, 0, 3, 7, 8],
        [7, 0, 3, 0, 0, 5, 4, 0, 9],
        [9, 2, 0, 0, 0, 0, 5, 0, 0],
        [1, 5, 0, 4, 0, 0, 0, 0, 2],
        [0, 0, 0, 7, 0, 3, 0, 9, 6],
        [0, 0, 6, 0, 1, 0, 0, 0, 5],
    ];
    let mut needs_solving = true;
    let mut total_passes = 0;
    print_puzzle(&puzzle);
    while needs_solving {
        needs_solving = false;
        for i in 0..9 {
            for j in 0..9 {
                if puzzle[i][j] == 0 {
                    let possible_values = get_inverse_values(get_existing_values(&puzzle, i, j));
                    if possible_values.len() == 1 {
                        needs_solving = true;
                        puzzle[i][j] = possible_values[0];
                    }
                }
            }
        }
        if needs_solving {
            total_passes = total_passes + 1;
            print_puzzle(&puzzle);
        }
    }
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

    let mut row_boundary: usize = 0;
    let mut col_boundary: usize = 0;

    if row < 3 {
        row_boundary = 0;
    }
    if row >= 3 && row < 6 {
        row_boundary = 3;
    }
    if row >= 6  {
        row_boundary = 6;
    }

    if col < 3 {
        col_boundary = 0;
    }
    if col >= 3 && col < 6 {
        col_boundary = 3;
    }
    if col >= 6  {
        col_boundary = 6;
    }

    for i in 0..3 {
        for j in 0..3 {
            let box_val = puzzle[i+row_boundary][j+col_boundary];
            if box_val >= 1{
                values.push(box_val);
            }
        }
    }


    values.sort_unstable();
    values.dedup();

    return values;
}

pub fn get_inverse_values(values: Vec<i32>) -> Vec<i32>{
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
