/*
    Author: Owen Sheffer
    Maze Traversal Project
    CIT360
*/

use maze_traversal::maze;
use std::io::Write;
use std::error::Error;
use std::fs::File;

// Result is a type for returning and propagating errors
// Box is a smart pointer for heap allocation
// dyn Error is a trait object for error handling
fn main() {

}

pub fn print_maze(maze: &maze::Maze) {

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