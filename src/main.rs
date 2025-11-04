/*
    Author: Owen Sheffer
    Maze Traversal Project
    CIT360
*/

use maze_traversal::{
    maze as maze,
    dfs_traversal as dfs_traversal,
    bfs_traversal as bfs_traversal
};
use std::io;

// Result is a type for returning and propagating errors
// Box is a smart pointer for heap allocation
// dyn Error is a trait object for error handling
fn main() {

    println!("Welcome to the maze generation program!");
    let _continue_input = get_input("Press Enter to continue.");
    
    // Outer loop to check allow user to generate more mazes
    loop {
        
        // Inner loop to handle all maze generation and traversal user control
        loop {
            let (size_x, size_y) = get_area();
            let mut maze = maze::Maze::new(size_x, size_y);
            maze.dfs_gen();

            println!();
            println!("Maze Generated!");
            maze::Maze::print_maze(&maze);

            let traversal = get_input("How would you like to traverse? (DFS or BFS)");
            
            // Ask use which traversal they would like to use to find a path from start to finish
            match traversal.as_str() {
                "BFS" | "bfs" => {
                    exec_bfs(&mut maze);

                    let swap_method = get_input("Try DFS now? (Yes/No)");
                    match parse_response(&swap_method) {
                        
                        UserResponse::Yes => {
                            exec_dfs(&mut maze);
                        }
                        UserResponse::No => {
                            break;
                        }
                        _ => {
                           println!("Invalid input. Please enter 'Yes'or 'No'");
                        }
                    }
                    break;
                }
                "DFS" | "dfs" => {
                    exec_dfs(&mut maze);

                    let swap_method = get_input("Try BFS now? (Yes/No)");
                    match parse_response(&swap_method) {
                        
                        UserResponse::Yes => {
                            exec_bfs(&mut maze);
                        }
                        UserResponse::No => {
                            break;
                        }
                        _ => {
                           println!("Invalid input. Please enter 'Yes'or 'No'");
                        }
                    }
                    break;
                }
                _ => {
                    println!("Invalid input. Please enter 'DFS' or 'BFS'");
                }
            }
        }

        let restart = get_input("Generate another maze? (Yes/No)");
        match restart.as_str() {

            "Yes" | "yes" | "Y" | "y" => {
                println!();
                continue
            }
            _ => {
                println!("Exiting program.");
                break;
            }
        }
    }
}

// Simple rusty way to parse yes no input
enum UserResponse {
    Yes,
    No,
    Invalid,
}
fn parse_response(input: &str) -> UserResponse {
    match input {
        "Yes" | "yes" | "Y" | "y" => UserResponse::Yes,
        "No" | "no" | "N" | "n" => UserResponse::No,
        _ => UserResponse::Invalid,
    }
}

fn get_input(prompt: &str) -> String {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Failed to read input. Try again.");
            continue;
        }

        return input.trim().to_string();
    }
}

/// Function to get maze size and prevent extraneous input
fn get_area() -> (usize, usize) {

    // Default to size of 50 but user should always be prompted for input
    let mut x = 20;
    let mut y= 20;

    loop {
        println!("Enter the width for the maze between 10-50:");

        let mut input = String::new();

        // Handle errors with result enum returned by read_line()
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Failed to read input. Try again.");
            continue;
        }
        
        match input.trim().parse::<usize>() {
            Ok(num) if num >= 10 && num <= 50 => x = num,
            _ => println!("Invalid input. Please enter a number between 10 and 50."),
        }
        break;
    }

    loop {
        println!("Enter the height for the maze between 10-50:");

        let mut input = String::new();

        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Failed to read input. Try again.");
            continue;
        }

        match input.trim().parse::<usize>() {
            Ok(num) if num >= 10 && num <=50 => y = num,
            _ => println!("Invalid input. Please enter a number between 10 and 50."),
        }
        break;
    }

    return (x,y);
}

fn exec_dfs(maze: &mut maze::Maze) {
    println!();
    println!("DFS Traversal");
    dfs_traversal::traverse(maze);
    maze::Maze::print_maze(maze);
}

fn exec_bfs(maze: &mut maze::Maze) {
    println!();
    println!("BFS Traversal");
    bfs_traversal::traverse(maze);
    maze::Maze::print_maze(&maze);
}