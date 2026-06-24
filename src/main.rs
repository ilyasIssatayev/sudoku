use std::fs::read_to_string;
mod display;
mod sudoku;
use display::{print_probabilities, print_sudoku};
use sudoku::Sudoku;

fn main() {
    println!("sudoku");
    let mut puzzles = read_sudoku_file();
    solve_sudoku(puzzles.remove(0))
}

fn solve_sudoku(sudoku: Sudoku) {
    let mut propabilities = get_propabilities(&sudoku);
    // print_probabilities(&propabilities);
    // print_sudoku(&sudoku.grid);
    // return;
    // print!("\n\n start ==== \n");
    for i in 0..9 {
        purge_line_porpabilities(&mut propabilities[i], &sudoku.row(i));
    }
    print!("\n\n rows purged ==== \n");
    print_probabilities(&propabilities);
    print_sudoku(&sudoku.grid);
    for i in 0..9 {
        purge_line_porpabilities(&mut propabilities[i], &sudoku.col(i));
    }
    print!("\n\n cols purged ==== \n");
    print_probabilities(&propabilities);
    print_sudoku(&sudoku.grid);
}

fn read_sudoku_file() -> Vec<Sudoku> {
    let file = read_to_string("./data/sudoku_puzzles.csv").expect("Cant read the file");
    file.lines().skip(1).filter_map(Sudoku::from_line).collect()
}

fn get_propabilities(sudoku: &Sudoku) -> [[Vec<u8>; 9]; 9] {
    let mut propabilities: [[Vec<u8>; 9]; 9] =
        std::array::from_fn(|_| std::array::from_fn(|_| (1..=9).collect()));

    for row in 0..9 {
        for col in 0..9 {
            if sudoku.grid[row][col] != 0 {
                propabilities[row][col] = vec![sudoku.grid[row][col]];
            }
        }
    }
    return propabilities;
}

fn purge_line_porpabilities(propabilities: &mut [Vec<u8>; 9], row: &[u8; 9]) {
    for propability_square in propabilities {
        if propability_square.len() > 1 {
            propability_square.retain(|x| !row.contains(x));
        }
    }
}
