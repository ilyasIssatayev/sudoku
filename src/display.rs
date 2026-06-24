pub fn print_sudoku(grid: &[[u8; 9]; 9]) {
    let sep = "+-------+-------+-------+";
    for row in 0..9 {
        if row % 3 == 0 { println!("{}", sep); }
        for col in 0..9 {
            if col % 3 == 0 { print!("| "); }
            let cell = grid[row][col];
            if cell == 0 { print!(". "); } else { print!("{} ", cell); }
        }
        println!("|");
    }
    println!("{}", sep);
}

pub fn print_probabilities(probabilities: &[[Vec<u8>; 9]; 9]) {
    let h_box_sep = "+-------------------+   +-------------------+   +-------------------+";
    let h_cell_sep = "|------+------+------|   |------+------+------|   |------+------+------|";

    for outer_row in 0..9 {
        if outer_row % 3 == 0 {
            if outer_row != 0 { println!(); }
            println!("{}", h_box_sep);
        } else {
            println!("{}", h_cell_sep);
        }

        for inner_row in 0..3 {
            for outer_col in 0..9 {
                if outer_col == 0 {
                    print!("| ");
                } else if outer_col % 3 == 0 {
                    print!("   | ");
                } else {
                    print!(" ");
                }

                let candidates = &probabilities[outer_row][outer_col];
                for inner_col in 0..3 {
                    let digit = (inner_row * 3 + inner_col + 1) as u8;
                    if candidates.contains(&digit) {
                        print!("{} ", digit);
                    } else {
                        print!(". ");
                    }
                }
            }
            println!("|");
        }
    }

    println!();
    println!("{}", h_box_sep);
}
