/// Maze Initialization and Generation

use rand::prelude::*;
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub start_cell: (usize, usize),
    pub end_cell: (usize, usize),
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
    pub path: bool, // part of solution path
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
                    path: false,
                    walls: [true; 4],
                    visited: false
                });
            }
            grid.push(row);
        }
        Maze{width, height, grid, start_cell: (0,0), end_cell: (0,0)}
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
        self.start_cell = (start_x, start_y);

        self.dfs_recurse(start_x, start_y, &mut rng, &mut cells_visited);
    }


    // Directions to move to neighboring cells: (dx, dy, wall_index)
    // Maze starts at top-left corner (0,0) so -1 is up/left, +1 is down/right
    pub const DIRECTIONS: [(isize, isize, usize); 4] = [
        (0isize, -1isize, 0), // Top
        (1isize, 0isize, 1),  // Right
        (0isize, 1isize, 2),  // Bottom
        (-1isize, 0isize, 3)  // Left
    ];

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
            self.end_cell = (x, y);
        }

        // Unvisited neighbors stack
        let mut neighbors = Vec::new();

        // Check all four directions from current cell to store unvisited neighbors
        for &(dx, dy, dir) in &Self::DIRECTIONS {

            // neighbor x and y coordinates
            // E.g. x=2,y=2 and dx=0,dy=-1 (top) => nx=2,ny=1
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

        // For each unvisited neighbor
        for (nx, ny, dir) in neighbors {

            // Base case: if neighbor is unvisited
            if !self.grid[nx][ny].visited {
                // Remove walls between current cell and neighbor
                self.grid[x][y].walls[dir] = false; // remove wall in current cell

                // Adding 2 and taking the remainder of 4 gives the opposite wall direction
                // 0+2=2%4=2 (top->bottom), 1+2=3%4=3 (right->left), etc.
                self.grid[nx][ny].walls[(dir + 2) % 4] = false; // remove opposite wall in neighbor cell

                // Recurse into neighbor
                self.dfs_recurse(nx, ny, rng, cells_visited);
                // After recursion returns, loop continues to next neighbor
                // This only happens if that path is exhausted (backtracking)
            }
        }
    }

    // Helper function so we can run multiple traversals on the same maze
    pub fn reset_visited(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.grid[x][y].visited = false;
                self.grid[x][y].path = false;
            }
        }
    }

    // Helper function to print the maze
    pub fn print_maze(maze: &Maze) {

    // Make walls 4 wide in console
    // Need to handle top walls, left walls, right walls, bottom walls

    for y in 0..maze.height {
        // Print the top walls
        for x in 0..maze.width {
            if maze.grid[x][y].walls[0] {
                print!("+---");
            } else {
                print!("+   ");
            }
        }
        println!("+");

        // Print the left walls and cell content
        // Iterate through each cell in the row left to right
        for x in 0..maze.width {
            if maze.grid[x][y].walls[3] {
                print!("|");
            } else {
                print!(" ");
            }

            if maze.grid[x][y].start {
                print!(" S ");
            } else if maze.grid[x][y].end {
                print!(" E ");
            } else if maze.grid[x][y].path {
                print!(" X ");
            } else {
                print!("   ");
            }
        }

        // Print the rightmost wall
        if maze.grid[maze.width - 1][y].walls[1] {
            println!("|");
        } else {
            println!(" ");
        }
    }

    // Print the bottom walls of the last row
    for x in 0..maze.width {
        if maze.grid[x][maze.height - 1].walls[2] {
            print!("+---");
        } else {
            print!("+   ");
        }
    }
    println!("+");

    }
}