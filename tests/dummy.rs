use maze_traversal::maze::Maze;

#[test]
fn dfs_maze(){
    let width = 21;
    let height = 21;
    let mut maze2 = Maze::new(width, height);
    
    maze2.dfs_gen(); // fixed seed for reproducibility
    
    print_maze(&maze2);
}

pub fn print_maze(maze: &Maze) {

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

