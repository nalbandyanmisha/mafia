use crate::engine::{GameView, SeatView};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

pub fn draw_player(frame: &mut Frame, area: &Rect, data: &SeatView) -> Result<(), anyhow::Error> {
    let name = if data.name.is_empty() {
        "Empty"
    } else {
        &data.name
    };

    let player_block = Block::default()
        .borders(Borders::ALL)
        .title(name)
        .style(Style::default().fg(Color::White));
    let player_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(player_block.inner(*area));

    frame.render_widget(player_block, *area);

    draw_chair_box(frame, &player_layout[0], data)?;
    draw_stats_box(frame, &player_layout[1], data)?;

    // Implementation for drawing a player in the TUI
    Ok(())
}

pub fn draw_chair_box(
    frame: &mut Frame,
    area: &Rect,
    view: &SeatView,
) -> Result<(), anyhow::Error> {
    let position_block = Block::bordered().border_type(BorderType::Rounded);
    let position_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Percentage(50),
        ])
        .split(*area);
    frame.render_widget(position_block, *area);

    draw_chair_content(frame, &position_layout[1], view)?;

    Ok(())
}

pub fn draw_chair_content(
    frame: &mut Frame,
    area: &Rect,
    view: &SeatView,
) -> Result<(), anyhow::Error> {
    let position_content =
        Paragraph::new(format!("{}", view.chair.position())).alignment(Alignment::Center);
    frame.render_widget(position_content, *area);
    Ok(())
}

pub fn draw_stats_box(
    frame: &mut Frame,
    area: &Rect,
    view: &SeatView,
) -> Result<(), anyhow::Error> {
    let stats_block = Block::bordered().border_type(BorderType::Rounded);
    let stats_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(stats_block.inner(*area));
    frame.render_widget(stats_block, *area);

    draw_warnings(frame, &stats_layout[0], view)?;
    draw_role(frame, &stats_layout[1], view)?;
    draw_life_status(frame, &stats_layout[2], view)?;
    draw_nomination(frame, &stats_layout[3], view)?;
    Ok(())
}

pub fn draw_warnings(frame: &mut Frame, area: &Rect, view: &SeatView) -> Result<(), anyhow::Error> {
    let warnings_content =
        Paragraph::new(format!("Warnings: {}", view.warnings)).alignment(Alignment::Left);
    frame.render_widget(warnings_content, *area);
    Ok(())
}

pub fn draw_role(frame: &mut Frame, area: &Rect, view: &SeatView) -> Result<(), anyhow::Error> {
    let role_content = Paragraph::new(format!("Role: {}", view.role)).alignment(Alignment::Left);
    frame.render_widget(role_content, *area);
    Ok(())
}

pub fn draw_life_status(
    frame: &mut Frame,
    area: &Rect,
    view: &SeatView,
) -> Result<(), anyhow::Error> {
    let life_content =
        Paragraph::new(format!("Status: {}", view.life_status)).alignment(Alignment::Left);
    frame.render_widget(life_content, *area);
    Ok(())
}

pub fn draw_nomination(
    frame: &mut Frame,
    area: &Rect,
    view: &SeatView,
) -> Result<(), anyhow::Error> {
    let nomination_content = Paragraph::new(format!("Nominations: ")).alignment(Alignment::Left);
    frame.render_widget(nomination_content, *area);
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
        draw_player(frame, area, &view.seats[i])?;
    }

    // Render right side chairs (6–10)
    for (i, area) in right_chairs.iter().enumerate() {
        draw_player(frame, area, &view.seats[i + 5])?;
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
