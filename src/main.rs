mod actions;
mod app;
mod engine;
mod tui;

use std::time::Duration;
use tui::chair::{draw_command, draw_event, draw_table};

use app::{App, AppStatus};
use ratatui::{
    Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    widgets::Block,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut app = App::new();

    // println!("Lattice points on circle of radius {radius}: {points:?}");

    while app.status == AppStatus::Running {
        terminal.draw(|f| view(&app, f))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.on_key(key.code);
            }
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

fn view(app: &App, frame: &mut Frame) {
    use Constraint::{Fill, Length, Min};

    let area = frame.area();
    let [game_area, command_area] = Layout::vertical([Min(3), Length(3)]).areas(area);
    let [table_area, event_area] = Layout::horizontal([Fill(3), Fill(1)]).areas(game_area);

    frame.render_widget(Block::bordered().title("Command Input"), command_area);
    frame.render_widget(Block::bordered().title("Event"), event_area);

    let game_view = app.engine.view();
    draw_command(frame, &command_area, &app.input).unwrap();
    draw_table(frame, &table_area, &game_view).unwrap();
    draw_event(frame, &event_area, &game_view).unwrap();
}
