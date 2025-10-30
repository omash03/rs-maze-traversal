use maze_traversal::maze::Maze;


#[test]
fn init_maze() {
    let width = 50;
    let height = 50;
    let mut maze1 = Maze::new(width, height);
    
    
    print_maze(&maze1);
}

fn print_maze(maze: &Maze) {
    for row in &maze.grid {
        for cell in row {
            if cell.path {
                print!("  ");  // two spaces for open path
            } else {
                print!("██");  // solid block for wall
            }
        }
        println!();
    }
}

