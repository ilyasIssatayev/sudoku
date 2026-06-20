use std::collections::HashSet;

#[derive(Debug)]
pub struct Sudoku {
    id: u32,
    difficulty: String,
    clues: u32,
    grid: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(",").collect();
        let id = parts[0].parse::<u32>().ok()?;
        let difficulty = parts[1].to_string();
        let clues = parts[2].parse::<u32>().ok()?;
        let puzzle = parts[3];

        let mut grid = [[0u8; 9]; 9];

        for (i, ch) in puzzle.chars().enumerate() {
            let digit = ch.to_digit(10)? as u8;
            grid[i / 9][i % 9] = digit;
        }
        Some(Sudoku {
            id,
            difficulty,
            clues,
            grid,
        })
    }

    pub fn row(&self, index: usize) -> [u8; 9] {
        self.grid[index]
    }
    pub fn col(&self, index: usize) -> [u8; 9] {
        let mut elements = [0u8; 9];

        for rowIndex in 0..9 {
            let row = self.grid[rowIndex];
            elements[rowIndex] = row[index];
        }

        return elements;
    }
}

trait Valid {
    fn is_valid(&self) -> bool;
}

impl Valid for [u8; 9] {
    fn is_valid(&self) -> bool {
        let set: HashSet<u8> = self.iter().copied().filter(|&x| x != 0).collect();
        set.len() == 9
    }
}
