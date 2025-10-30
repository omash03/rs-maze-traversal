/*
    TUI Layout Module
    -----------------
    This module handles the layout and rendering of the terminal user interface (TUI)
    using the RataTUI library. It defines the structure of the UI, including buttons,
    maze display area, and informational text.
*/

use ratatui::{Frame};
use ratatui::text::Text;
use ratatui::layout::*;
use ratatui::widgets::*;
use ratatui::style::{Color, Style};

// Buttons
pub fn get_buttons() -> [&'static str; 5] {
    [
        "1: Generate Maze",
        "2: Stack Traversal",
        "3: Queue Traversal",
        "4: Slow Mode",
        "5: Exit"
    ]
}

pub fn draw(
    f: &mut Frame,
    selected: usize,
) {
    // Get the size of the terminal
    let size = f.area();

    let buttons = get_buttons();
    
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

    // Split Button area horizontally into equal parts and create buttons
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
            // Highlight selected button yellow
            Paragraph::new(label)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)))
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
