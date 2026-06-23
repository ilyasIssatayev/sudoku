use std::fs::read_to_string;
mod display;
mod sudoku;
use display::print_probabilities;
use sudoku::Sudoku;

fn main() {
    println!("sudoku");
    let puzzles = read_sudoku_file();
    let mut propabilities = get_propabilities(&puzzles[0]);
    print_probabilities(&propabilities);
    purge_row_porpabilities(&mut propabilities[0], &puzzles[0].row(1));
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

fn purge_row_porpabilities(propabilities: &mut [Vec<u8>; 9], row: &[u8; 9]) {
    for propability_square in propabilities {
        propability_square.retain(|x| !row.contains(x));
    }
}
