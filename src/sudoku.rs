use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct Sudoku {
    pub id: u32,
    pub difficulty: String,
    pub clues: u32,
    pub grid: [[u8; 9]; 9],
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
    pub fn square(&self, x: usize, y: usize) -> [[u8; 3]; 3] {
        let mut square = [[0u8; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                square[i][j] = self.grid[x * 3 + i][y * 3 + j];
            }
        }
        square
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Puzzle #{} — {} ({} clues)", self.id, self.difficulty, self.clues)?;
        let sep = "+-------+-------+-------+";
        for row in 0..9 {
            if row % 3 == 0 { writeln!(f, "{}", sep)?; }
            for col in 0..9 {
                if col % 3 == 0 { write!(f, "| ")?; }
                let cell = self.grid[row][col];
                if cell == 0 { write!(f, ". ")?; } else { write!(f, "{} ", cell)?; }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "{}", sep)?;
        Ok(())
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

impl Valid for [[u8; 3]; 3] {
    fn is_valid(&self) -> bool {
        let set: HashSet<u8> = self.iter().flatten().copied().filter(|&x| x != 0).collect();
        set.len() == 9
    }
}
