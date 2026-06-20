use std::fs::read_to_string;
mod sudoku;
use sudoku::Sudoku;

fn main() {
    println!("sudoku");
    let puzzles = read_sudoku_file();
    println!("{:#?}", puzzles);
}

fn read_sudoku_file() -> Vec<Sudoku> {
    let file = read_to_string("./data/sudoku_puzzles.csv").expect("Cant read the file");
    file.lines().skip(1).filter_map(Sudoku::from_line).collect()
}
