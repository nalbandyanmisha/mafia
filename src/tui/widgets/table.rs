use ratatui::{Frame, layout::Rect};

use crate::snapshot::{SeatData, TableData};
use crate::tui::widgets::host::draw_host;

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

fn center_of(rect: &Rect) -> (f64, f64) {
    let cx = rect.x as f64 + rect.width as f64 / 2.0;
    let cy = rect.y as f64 + rect.height as f64 / 2.0;
    (cx, cy)
}

fn angle_from_host(host: &Rect, player: &Rect) -> f64 {
    let (host_cx, host_cy) = center_of(host);

    // Find the corner of player closest to host center
    let corners = [
        (player.x as f64, player.y as f64),
        (player.x as f64 + player.width as f64, player.y as f64),
        (player.x as f64, player.y as f64 + player.height as f64),
        (
            player.x as f64 + player.width as f64,
            player.y as f64 + player.height as f64,
        ),
    ];

    let closest = corners
        .iter()
        .min_by(|a, b| {
            let da = (a.0 - host_cx).powi(2) + (a.1 - host_cy).powi(2);
            let db = (b.0 - host_cx).powi(2) + (b.1 - host_cy).powi(2);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap();

    let dx = closest.0 - host_cx;
    let dy = closest.1 - host_cy;

    // atan2 returns [-pi, pi], 0 is to the right (east), +y is down
    let mut angle = dy.atan2(dx);

    // Shift reference so 0 is left-bottom
    angle -= -std::f64::consts::FRAC_PI_4; // left-bottom is -pi/4 from east
    if angle < 0.0 {
        angle += 2.0 * std::f64::consts::PI;
    }

    angle
}
fn sort_player_areas_clockwise(host: &Rect, players: &mut [Rect]) {
    players.sort_by(|a, b| {
        angle_from_host(host, a)
            .partial_cmp(&angle_from_host(host, b))
            .unwrap()
    });
}

fn draw_chairs_around_host(
    frame: &mut Frame,
    players_areas: &[Rect],
    seats: &[SeatData],
) -> Result<(), anyhow::Error> {
    for (i, area) in players_areas.iter().enumerate() {
        crate::tui::widgets::chair::draw_chair(frame, *area, &seats[i].clone())?;
    }
    Ok(())
}

pub fn draw_table(
    frame: &mut Frame,
    table_area: Rect,
    n_players: usize,
    table: &TableData,
) -> Result<(), anyhow::Error> {
    let player_area_w = table_area.width / 6;
    let player_area_h = table_area.height / 6;
    let host_area = calculate_host_area(table_area);
    let mut players_areas = calculate_players_areas(
        table_area,
        host_area,
        n_players,
        player_area_w,
        player_area_h,
    );
    sort_player_areas_clockwise(&host_area, &mut players_areas);
    players_areas.rotate_right(6); // Adjust so player 1 is at bottom-left
    draw_host(frame, host_area);
    draw_chairs_around_host(frame, &players_areas, &table.seats).unwrap();
    Ok(())
}
