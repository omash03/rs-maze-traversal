// Create a stack (vector and use push/pop)
use crate::maze::Maze;

pub fn traverse(maze: &mut Maze) {

    maze.reset_visited();

    // Enable debug printing
    let debug = false;

    // Stack to store cells visited
    let mut stack = Vec::new();
    let start_cell = maze.start_cell;

    maze.reset_visited();

    // Start from the start cell
    stack.push(start_cell);
    maze.grid[start_cell.0][start_cell.1].visited = true;

    // Begin recursion
    dfs_recursion(maze, start_cell.0, start_cell.1, &mut stack, debug);
}

fn dfs_recursion(maze: &mut Maze, x: usize, y: usize, stack: &mut Vec<(usize, usize)>, debug: bool) {

    // Base Case: Stop recursion once we reach the end cell
    if let Some(&(x, y)) = stack.last() {
        if (x, y) == maze.end_cell {
            if debug {
                println!("Reached the end cell at ({}, {})", x, y);
                stack_print(stack);
            }

            // Mark the path from start to end
            // Iterate through path x,y in stack and mark as path
            for &(px, py) in stack.iter() {
                maze.grid[px][py].path = true;
            }
            return;
        }
    }

    // Check all four directions for possible moves
    for direction in &Maze::DIRECTIONS {
        let (dx, dy, wall_index) = *direction;
        let nx = x as isize + dx;
        let ny = y as isize + dy;

         // Check if neighbor is within bounds
        if nx >= 0 && nx < maze.width as isize && ny >= 0 && ny < maze.height as isize {
            let (nx, ny) = (nx as usize, ny as usize);

            // Check if there is no wall between current cell and neighbor
            if !maze.grid[x][y].walls[wall_index] && !maze.grid[nx][ny].visited {
                // Mark neighbor as visited and push to stack
                maze.grid[nx][ny].visited = true;
                stack.push((nx, ny));

                // Recurse with updated stack
                dfs_recursion(maze, nx, ny, stack, debug);

                // Backtrack: Pop from stack if end not reached
                stack.pop();
            }
        }
    }
}

fn stack_print(stack: &Vec<(usize, usize)>) {
    print!("Solution Path?:");
    for &(x,y) in stack.iter() {
        print!(" ({},{})", x, y);
    }
    println!();
}