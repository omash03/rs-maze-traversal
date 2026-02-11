/// Breadth-First Search using the vector deque data structure

use crate::maze::Maze;
use std::collections::VecDeque;

pub fn traverse(maze: &mut Maze) {
    let debug = false;
    
    let start_cell = maze.start_cell;
    let end_cell = maze.end_cell;
    
    maze.reset_visited();
    
    // Queue stores cells to explore
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    
    // 2D vector to track parent of each cell
    // Option enum used to store the parent of each cell
    // Initialize with None for all cells since there are no parents yet
    let mut parent: Vec<Vec<Option<(usize, usize)>>> = 
        // Init create 2d Vec 
        vec![vec![None; maze.height]; maze.width];
    
    queue.push_back(start_cell);
    // Mark start cell as visited
    maze.grid[start_cell.0][start_cell.1].visited = true;
    
    // BFS loop
    while let Some((x, y)) = queue.pop_front() {
        if debug {
            println!("Visiting: ({}, {})", x, y);
        }
        
        // Check if we reached the end
        if (x, y) == end_cell {
            if debug {
                println!("Reached end cell at ({}, {})", x, y);
            }
            
            // Reconstruct path from end to start
            let mut current = end_cell;
            while current != start_cell {
                maze.grid[current.0][current.1].path = true;
                current = parent[current.0][current.1].unwrap();
            }
            maze.grid[start_cell.0][start_cell.1].path = true;
            return;
        }
        
        // Explore all four directions ()
        for direction in &Maze::DIRECTIONS {
            let (dx, dy, wall_index) = *direction;
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            
            // Check bounds
            if nx >= 0 && nx < maze.width as isize && ny >= 0 && ny < maze.height as isize {
                let (nx, ny) = (nx as usize, ny as usize);
                
                // Check if no wall and not visited
                if !maze.grid[x][y].walls[wall_index] && !maze.grid[nx][ny].visited {
                    maze.grid[nx][ny].visited = true;
                    parent[nx][ny] = Some((x, y));
                    queue.push_back((nx, ny));
                }
            }
        }
    }
    
    if debug {
        println!("No path found!");
    }
}