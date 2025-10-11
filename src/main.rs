/*
    Author: Owen Sheffer
    Maze Traversal Project
    CIT360
*/

use crossterm;
use crossterm::event::{Event, KeyCode, KeyEvent};

use ratatui::{self, Frame};
use ratatui::text::Text;
use ratatui::layout::*;
use ratatui::widgets::*;

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

    // Setup logging to file
    let log_file = File::create("out.log")?;
    CombinedLogger::init(vec![
        WriteLogger::new(LevelFilter::Debug, Config::default(), log_file),
    ])?;

    // Buttons
    let buttons = [
        "1: Generate Maze",
        "2: Stack Traversal",
        "3: Queue Traversal",
        "4: Show Debug",
        "5: Exit",
    ];

    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let _term_guard = TermRestore; // Ensure terminal is restored on exit
    let mut terminal = ratatui::init();
    let mut selected: usize = 0;

    // Clear terminal before entering TUI mode
    terminal.clear()?;


    loop {
        // Draw UI with current selection
        terminal.draw(|f| draw(f, &buttons, selected))?;

        // Handle input
        if crossterm::event::poll(std::time::Duration::from_millis(500))? {
            match crossterm::event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        info!("Exiting on 'q' or Esc");
                        break;
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let idx = (c as u8 - b'1') as usize;
                        if idx < buttons.len() {
                            selected = idx;
                            info!("Triggered: {} ({})", buttons[selected], selected + 1);

                            // check by index (5th button is Exit) or use contains("Exit")
                            if idx == 4 {
                                break;
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

fn draw(f: &mut Frame, buttons: &[&str], selected: usize) {
    // Get the size of the terminal
    let size = f.area();


    // Compute button area
    let button_width = size.width;
    let button_height = size.height / 8;
    let button_x = size.x + (size.width - button_width) / 2;
    let button_y = size.height.saturating_sub(button_height);

    let button_area = Rect {
        x: button_x,
        y: button_y,
        width: button_width,
        height: button_height,
    };

    // Compute content area for maze inside the border (inset by 1 on all sides)
    let content_x = size.x + 1;
    let content_y= size.y + 1;
    let content_width: u16 = size.width.saturating_sub(2);
    let content_height: u16 = size.height.saturating_sub(2);

    let maze_area = Rect {
        x: content_x,
        y: content_y,
        width: content_width,
        height: content_height.saturating_sub(button_height),
    };


    // Rendering

    // Draw an outer border block with title
    let block = Block::default()
        .title("Maze Traversal")
        .borders(Borders::ALL);
    f.render_widget(block, size);


    // Container for maze
    let maze_block = Block::default()
        .borders(Borders::ALL);

    f.render_widget(maze_block, maze_area);
    // End maze container

    
    let buttons_block = Block::default()
        .title("Controls")
        .borders(Borders::ALL);
    f.render_widget(buttons_block, button_area);

    // Split Button area horizontally into equal parts for each button
    let mut constraints = Vec::new();
    for _ in 0..buttons.len() {
        constraints.push(Constraint::Percentage(100 / buttons.len() as u16));
    }
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .margin(0)
        .split(button_area);

    // Render each button inside its chunk
    for (i, &label) in buttons.iter().enumerate() {
        let button = if i == selected {
            // Highlight selected button
            Paragraph::new(label)
                .block(Block::default().borders(Borders::ALL).border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)))
                .alignment(Alignment::Center)
        } else {
            Paragraph::new(label)
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center)
        };
        f.render_widget(button, chunks[i]);
    }

    let text = Text::from("Press ESC key to exit...");
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center);
    let area = Rect {
        x: size.x,
        y: size.y + size.height / 2 - 1,
        width: size.width,
        height: 3,
    };
    f.render_widget(paragraph, area);
}
