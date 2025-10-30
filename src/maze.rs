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
        let grid = Vec::with_capacity(height);
        
        for y in 0..height {
            let mut row = Vec::with_capacity(width);

            for x in 0..width {
                row.push(Cell {
                    x,
                    y,
                    path: false,
                    walls: [true; 4],
                    visited: false
                });
            }
        }
        Maze{width, height, grid}
    }

    /// Generate a maze using randomized DFS (recursive backtracker implemented with a stack).
    /// After generation, `path` will be true for carved cells.
    pub fn generate(&mut self, seed: u64) {
        // TODO
    }
}