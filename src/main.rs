/*
    Author: Owen Sheffer
    Maze Traversal Project
    CIT360
*/

mod tui_layout; 
mod dfs_traversal;
mod bfs_traversal;
use maze_traversal::maze;
use std::io::Write;
use tui_layout::draw;

// Cross platform terminal backend used by RataTUI
use crossterm;
use crossterm::event::{Event, KeyCode};

// https://docs.rs/ratatui/latest/ratatui/
use ratatui;

// Logging since we can't capture stdout/stderr in TUI mode
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::fs::File;
use log::{info, warn, error, debug};

/*
    Ensure terminal is restored on panic or normal exit
    This will be done by implementing the Drop trait for the TermRestore struct
    Drop trait is called when the struct goes out of scope (RAII)
*/
struct TermRestore;
impl Drop for TermRestore {
    fn drop(&mut self) {
        // _ as name since we don't care about the result
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::cursor::Show
        );
    }
}

// Result is a type for returning and propagating errors
// Box is a smart pointer for heap allocation
// dyn Error is a trait object for error handling
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let buttons = tui_layout::get_buttons();

    // Setup logging to file
    let log_file = File::create("out.log")?;
    CombinedLogger::init(vec![
        WriteLogger::new(LevelFilter::Debug, Config::default(), log_file),
    ])?;

    // Setup terminal and ensure it is restored on exit
    crossterm::terminal::enable_raw_mode()?;
    let _term_guard = TermRestore; // Keep in scope for duration of execution
    let mut terminal = ratatui::init();
    let mut selected: usize = 0;

    // Clear terminal before entering TUI mode
    terminal.clear()?;

    // Rendering loop
    loop {
        // Draw UI with current selection
        terminal.draw(|f| draw(f, selected))?;

        // Handle input (only check every 500ms to reduce CPU usage)
        if crossterm::event::poll(std::time::Duration::from_millis(500))? {
            match crossterm::event::read()? {
                Event::Key(key) => match key.code {

                    // Escape sequence for TUI
                    KeyCode::Char('q') | KeyCode::Esc => {
                        info!("Exiting on 'q' or Esc");
                        break;
                    }

                    // Number keys to select buttons
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let idx = (c as u8 - b'1') as usize;
                        if idx < buttons.len() {
                            selected = idx;
                            info!("Triggered: {} ({})", buttons[selected], selected + 1);

                            // check by index to trigger functions
                            match idx {
                                // 1: Generate Maze -> write a simple maze text file
                                0 => {
                                    // TODO
                                }
                                // 2: Stack Traversal (DFS)
                                //1 => dfs_traversal::run_dfs(),
                                // 3: Queue Traversal (BFS)
                                //2 => bfs_traversal::run_bfs(),
                                4 => {} //show_slowed
                                5 => break,
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

    }
    Ok(())
}