use crate::engine::{GameView, SeatView};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph},
};

pub fn draw_chair(frame: &mut Frame, area: &Rect, view: &SeatView) -> Result<(), anyhow::Error> {
    let name = if view.name.is_empty() {
        "Empty"
    } else {
        &view.name
    };
    let chair = Block::default()
        .borders(Borders::ALL)
        .title(format!("Chair {}", view.chair.position()))
        .style(Style::default().fg(Color::White));
    frame.render_widget(Paragraph::new(name).block(chair), *area);
    // Implementation for drawing a chair in the TUI
    Ok(())
}

pub fn draw_host(frame: &mut Frame, area: &Rect, view: &GameView) -> Result<(), anyhow::Error> {
    let host = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(50))
        .title("Host")
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(Paragraph::new(view.phase.to_string()).block(host), *area);
    Ok(())
}

pub fn draw_table(frame: &mut Frame, area: &Rect, view: &GameView) -> Result<(), anyhow::Error> {
    let table_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(10))
        .title("Table");
    let table_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(table_block.inner(*area));

    let host_area = table_layout[1];

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50); 2])
        .spacing(10)
        .split(table_layout[0]);

    let left_chairs = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20); 5])
        .split(columns[0]);

    let right_chairs = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20); 5])
        .split(columns[1]);

    for (i, area) in left_chairs.iter().enumerate() {
        draw_chair(frame, area, &view.seats[i + 1])?;
    }

    // Render right side chairs (6–10)
    for (i, area) in right_chairs.iter().enumerate() {
        draw_chair(frame, area, &view.seats[i + 5])?;
    }

    draw_host(frame, &host_area, view)?;
    frame.render_widget(table_block, *area);

    Ok(())
}

pub fn draw_command(frame: &mut Frame, area: &Rect, input: &str) -> Result<(), anyhow::Error> {
    let command_block = Block::default()
        .borders(Borders::ALL)
        .title("Command Input")
        .style(Style::default().fg(Color::Cyan));
    command_block.inner(*area);

    frame.render_widget(Paragraph::new(input).block(command_block), *area);
    Ok(())
}

pub fn draw_event(frame: &mut Frame, area: &Rect, view: &GameView) -> Result<(), anyhow::Error> {
    let event_block = Block::default()
        .borders(Borders::ALL)
        .title("Event Log")
        .style(Style::default().fg(Color::Magenta));
    frame.render_widget(event_block, *area);
    Ok(())
}
