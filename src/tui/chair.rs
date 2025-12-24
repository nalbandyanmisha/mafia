use crate::engine::{GameView, SeatView};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

pub fn view(frame: &mut Frame, app: &crate::app::App) {
    use Constraint::{Fill, Length, Min};

    let area = frame.area();
    let [game_area, command_area] = Layout::vertical([Min(3), Length(3)]).areas(area);
    let [table_area, event_area] = Layout::horizontal([Fill(3), Fill(1)]).areas(game_area);

    frame.render_widget(Block::bordered().title("Command Input"), command_area);
    frame.render_widget(Block::bordered().title("Event"), event_area);

    let game_view = app.engine.view();
    draw_command(frame, &command_area, &app.input).unwrap();
    draw_table(frame, &table_area, &game_view, app.current_timer).unwrap();
    draw_event(frame, &event_area, &game_view).unwrap();
}

pub fn draw_player(frame: &mut Frame, area: &Rect, data: &SeatView) -> Result<(), anyhow::Error> {
    let name = if data.name.is_empty() {
        "Empty"
    } else {
        &data.name
    };

    let player_block = Block::default()
        .borders(Borders::ALL)
        .title(name)
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));
    let player_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(player_block.inner(*area));

    frame.render_widget(player_block, *area);

    draw_chair_box(frame, &player_layout[0], data)?;
    draw_stats_box(frame, &player_layout[1], data)?;
    draw_timer_box(frame, &player_layout[2], Some(10))?;

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
    let nomination_content = Paragraph::new("Nominations").alignment(Alignment::Left);
    frame.render_widget(nomination_content, *area);
    Ok(())
}

pub fn draw_timer_box(
    frame: &mut Frame,
    area: &Rect,
    timer: Option<u64>,
) -> Result<(), anyhow::Error> {
    let timer_block = Block::bordered().border_type(BorderType::Rounded);
    let timer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(40),
            Constraint::Length(1),
            Constraint::Percentage(40),
        ])
        .split(timer_block.inner(*area));
    frame.render_widget(timer_block, *area);

    draw_timer(frame, &timer_layout[1], timer)?;

    Ok(())
}

pub fn draw_timer(frame: &mut Frame, area: &Rect, timer: Option<u64>) -> Result<(), anyhow::Error> {
    let timer_str = if let Some(seconds) = timer {
        format!("Time: {seconds}s",)
    } else {
        "No Timer".to_string()
    };
    let timer_content = Paragraph::new(timer_str).alignment(Alignment::Center);
    frame.render_widget(timer_content, *area);
    Ok(())
}

pub fn draw_phase(frame: &mut Frame, area: &Rect, phase: &str) -> Result<(), anyhow::Error> {
    let phase_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green));
    frame.render_widget(&phase_block, *area);

    let phase_content = Paragraph::new(format!("Phase: {phase}")).alignment(Alignment::Center);
    frame.render_widget(phase_content, phase_block.inner(*area));
    Ok(())
}

pub fn draw_round(frame: &mut Frame, area: &Rect, round: usize) -> Result<(), anyhow::Error> {
    let round_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue));
    frame.render_widget(&round_block, *area);

    let round_content = Paragraph::new(format!("Round: {round}")).alignment(Alignment::Center);
    frame.render_widget(round_content, round_block.inner(*area));
    Ok(())
}

pub fn draw_host(
    frame: &mut Frame,
    area: &Rect,
    view: &GameView,
    timer: Option<u64>,
) -> Result<(), anyhow::Error> {
    let host_block = Block::default()
        .borders(Borders::ALL)
        .title("Host")
        .style(Style::default().fg(Color::Yellow));
    let host_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_ratios([(2, 7), (3, 7), (2, 7)]))
        .split(host_block.inner(*area));

    let phase_area = host_layout[0];
    let timer_area = host_layout[1];
    let round_area = host_layout[2];
    frame.render_widget(host_block, *area);

    draw_phase(frame, &phase_area, &view.phase.to_string())?;
    draw_round(frame, &round_area, view.round_id.into())?;

    draw_timer_box(frame, &timer_area, timer)?;

    Ok(())
}

pub fn draw_table(
    frame: &mut Frame,
    area: &Rect,
    view: &GameView,
    timer: Option<u64>,
) -> Result<(), anyhow::Error> {
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

    draw_host(frame, &host_area, view, timer)?;
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
