use rand::{rngs::StdRng, Rng, SeedableRng};
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    // Outer vector is rows (y), inner vector is columns (x)
    // outer  vector stores pointers to heap-allocated inner vectors
    pub grid: Vec<Vec<Cell>>,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    /// path == true means open/path, false means wall (by default cells are walls until carved)
    pub path: bool,
    pub walls: [bool; 4], // top, right, bottom, left (clockwise)
    pub visited: bool,
}

impl Maze {
    /// Create a new maze structure (cells initialized as walls)
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(width);
        
        // X axis is rows
        for x in 0..width {

            let mut row = Vec::with_capacity(height);

            // Y axis is columns
            for y in 0..height {
                row.push(Cell {
                    x,
                    y,
                    path: false,
                    walls: [false; 4],
                    visited: false
                });
            }
            grid.push(row);
        }
        Maze{width, height, grid}
    }

    /// Not really a maze just wanted to see what random generation of walls looks like.
    pub fn basic_generate(&mut self, seed: u64) {

        let mut rng = StdRng::seed_from_u64(seed);

        // Loop through row
        for y in 0..self.height {
            // Loop through column
            for x in 0..self.width {

                let cell = &mut self.grid[y][x];
                cell.path = rng.gen_bool(0.60);

            }
        }
    }

    pub fn dfs_generate(&mut self, seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);

    }
}