use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

fn calculate_host_area(table_area: Rect) -> Rect {
    // Calculate host/control panel size
    let host_w = table_area.width / 3;
    let host_h = table_area.height / 3;

    // Calculate top-left corner to center it in table_area
    let host_x = table_area.x + (table_area.width - host_w) / 2;
    let host_y = table_area.y + (table_area.height - host_h) / 2;

    Rect {
        x: host_x,
        y: host_y,
        width: host_w,
        height: host_h,
    }
}

/// Compute positions for `n` player cards around a rectangular host
fn calculate_players_areas(
    table_area: Rect,
    host_area: Rect,
    n_players: usize,
    player_area_w: u16,
    player_area_h: u16,
) -> Vec<Rect> {
    let mut areas = Vec::with_capacity(n_players);

    // Center of the host block
    let host_center_x = host_area.x + host_area.width / 2;
    let host_center_y = host_area.y + host_area.height / 2;

    // Circle radius: distance from host center to place cards around
    // We can take the maximum half-dimension + some padding
    let radius_x = host_area.width / 2 + player_area_w + 2;
    let radius_y = host_area.height / 2 + player_area_h + 1;

    // Angle increment for n players around host
    let angle_step = 360.0 / n_players as f64;

    for i in 0..n_players {
        let angle_deg = i as f64 * angle_step;
        let angle_rad = angle_deg.to_radians();

        // Calculate card center position
        let cx = host_center_x as f64 + radius_x as f64 * angle_rad.cos();
        let cy = host_center_y as f64 + radius_y as f64 * angle_rad.sin();

        // Convert center to top-left corner of card
        let x = (cx - player_area_w as f64 / 2.0).round() as u16;
        let y = (cy - player_area_h as f64 / 2.0).round() as u16;

        // Clamp to table area
        let x = x
            .max(table_area.x)
            .min(table_area.x + table_area.width - player_area_w);
        let y = y
            .max(table_area.y)
            .min(table_area.y + table_area.height - player_area_h);

        areas.push(Rect {
            x,
            y,
            width: player_area_w,
            height: player_area_h,
        });
    }

    areas
}

fn draw_host(frame: &mut Frame, host_area: Rect) {
    // Draw the host/control panel block
    let host_block = Block::default()
        .borders(Borders::ALL)
        .title("Control Panel")
        .style(Style::default().fg(Color::Yellow));

    frame.render_widget(host_block, host_area);
}

fn draw_player(frame: &mut Frame, area: Rect) {
    // Build the block with rounded borders
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::White));

    frame.render_widget(block, area);
}

fn draw_players_around_host(frame: &mut Frame, players_areas: &[Rect]) {
    for area in players_areas.iter() {
        draw_player(frame, *area);
    }
}

pub fn draw_table(frame: &mut Frame, table_area: Rect, n_players: usize) {
    let player_area_w = table_area.width / 6;
    let player_area_h = table_area.height / 6;
    let host_area = calculate_host_area(table_area);
    let players_areas = calculate_players_areas(
        table_area,
        host_area,
        n_players,
        player_area_w,
        player_area_h,
    );
    draw_host(frame, host_area);
    draw_players_around_host(frame, &players_areas);
}
