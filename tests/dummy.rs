use maze_traversal::maze::Maze as maze;
use maze_traversal::dfs_traversal;

#[test]
fn dfs_trav(){

    let width = 25;
    let height = 25;
    let mut maze2 = maze::new(width, height);
    maze2.dfs_gen();
    dfs_traversal::traverse(&mut maze2);

    maze::print_maze(&maze2);
}

#[test]
fn bfs_traversal() {

    let width = 25;
    let height = 25;
    let mut maze2 = maze::new(width, height);
    
    maze2.dfs_gen(); // fixed seed for reproducibility
    
    maze_traversal::bfs_traversal::traverse(&mut maze2);
    
    maze::print_maze(&maze2);
}

#[test]
fn dfs_maze(){
    let width = 25;
    let height = 25;
    let mut maze2 = maze::new(width, height);
    
    maze2.dfs_gen(); // fixed seed for reproducibility
    
    maze::print_maze(&maze2);
}