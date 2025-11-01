use std::cell;

/// Maze Initialization and Generation

use rand::prelude::*;
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
    pub start: bool,
    pub end: bool,
    pub path: [bool; 4], // top, right, bottom, left (clockwise)
    pub walls: [bool; 4], // top, right, bottom, left (clockwise)
    pub visited: bool,
}

impl Maze {
    /// Create a new maze structure (cells initialized with walls)
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
                    start: false,
                    end: false,
                    path: [false; 4],
                    walls: [true; 4],
                    visited: false
                });
            }
            grid.push(row);
        }
        Maze{width, height, grid}
    }

    /// DFS Maze Generation
    pub fn dfs_gen(&mut self) {

        let mut rng = rand::thread_rng();
        let mut cells_visited = 0;

        // Recursive backtracking
        // Choose a random starting point
        let start_x = rng.gen_range(0..self.width);
        let start_y = rng.gen_range(0..self.height);
        self.grid[start_x][start_y].start = true;

        self.dfs_recurse(start_x, start_y, &mut rng, &mut cells_visited);
    }

    /// DFS Recursive Algorithm
    /// This is probably not very efficient but i can understand it lol
    /// 
    fn dfs_recurse(&mut self, x: usize, y: usize, rng: &mut ThreadRng, cells_visited: &mut usize) {
        
        // Mark current cell as visited
        self.grid[x][y].visited = true;
        *cells_visited += 1;

        // Check if this should be the end cell before processing neighbors
        if *cells_visited == self.width * self.height {
            // Mark as end if all cells have been visited
            self.grid[x][y].end = true;
        }

        // Unvisited neighbors stack
        let mut neighbors = Vec::new();

        // Direction for unvisited neighbors
        let directions = [
            (0isize, -1isize, 0), // Top
            (1isize, 0isize, 1),  // Right
            (0isize, 1isize, 2),  // Bottom
            (-1isize, 0isize, 3)  // Left
        ];

        // Check all four directions from current cell to store unvisited neighbors
        for &(dx, dy, dir) in &directions {

            // neighbor x and y coordinates
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            // check if neighbor is within bounds and unvisited
            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                let neighbor = &self.grid[nx as usize][ny as usize];
                if !neighbor.visited {
                    neighbors.push((nx as usize, ny as usize, dir));
                }
            }
        }

        // Choose a random unvisited neighbor
        neighbors.shuffle(rng);

        // For each unvisited neighbor (but only carve one path at a time)
        for (nx, ny, dir) in neighbors {
            if !self.grid[nx][ny].visited {
                // Remove walls between current cell and neighbor
                self.grid[x][y].walls[dir] = false; // remove wall in current cell
                self.grid[nx][ny].walls[(dir + 2) % 4] = false; // remove opposite wall in neighbor cell

                // Recurse into neighbor
                self.dfs_recurse(nx, ny, rng, cells_visited);
                // After recursion returns, loop continues to next neighbor
                // This only happens if that path is exhausted (backtracking)
            }
        }
    }
}