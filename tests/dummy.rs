use maze_traversal::maze::Maze;


#[test]
fn simple_gen() {
    let width = 50;
    let height = 50;
    let mut maze1 = Maze::new(width, height);
    
    maze1.basic_generate(450); // fixed seed for reproducibility
    
    print_maze(&maze1);
}

#[test]
fn dfs_maze(){
    let width = 21;
    let height = 21;
    let mut maze2 = Maze::new(width, height);
    
    maze2.dfs_generate(12345); // fixed seed for reproducibility
    
    print_maze(&maze2);
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

