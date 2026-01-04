use crate::tui::layout::{chair, host};
use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct TableLayout {
    pub host: host::HostLayout,
    pub chairs: Vec<chair::ChairLayout>,
}

pub fn table(main: Rect, chair_count: usize) -> TableLayout {
    let host = host::host(main);

    let chair_w = main.width / 6;
    let chair_h = main.height / 6;

    let mut chairs = calculate_chairs(main, host.area, chair_count, chair_w, chair_h);

    sort_clockwise(&host.area, &mut chairs);
    chairs.rotate_right(6);

    let chairs = chairs.into_iter().map(chair::ChairLayout::new).collect();
    TableLayout { host, chairs }
}

fn calculate_chairs(table: Rect, host: Rect, n: usize, w: u16, h: u16) -> Vec<Rect> {
    let mut areas = Vec::with_capacity(n);

    // Center of the host block
    let host_center_x = host.x + host.width / 2;
    let host_center_y = host.y + host.height / 2;

    // Circle radius: distance from host center to place cards around
    // We can take the maximum half-dimension + some padding
    let radius_x = host.width / 2 + w + 2;
    let radius_y = host.height / 2 + h + 1;

    // Angle increment for n players around host
    let angle_step = 360.0 / n as f64;

    for i in 0..n {
        let angle_deg = i as f64 * angle_step;
        let angle_rad = angle_deg.to_radians();

        // Calculate card center position
        let cx = host_center_x as f64 + radius_x as f64 * angle_rad.cos();
        let cy = host_center_y as f64 + radius_y as f64 * angle_rad.sin();

        // Convert center to top-left corner of card
        let x = (cx - w as f64 / 2.0).round() as u16;
        let y = (cy - h as f64 / 2.0).round() as u16;

        // Clamp to table area
        let x = x.max(table.x).min(table.x + table.width - w);
        let y = y.max(table.y).min(table.y + table.height - h);

        areas.push(Rect {
            x,
            y,
            width: w,
            height: h,
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
fn sort_clockwise(host: &Rect, chairs: &mut [Rect]) {
    chairs.sort_by(|a, b| {
        angle_from_host(host, a)
            .partial_cmp(&angle_from_host(host, b))
            .unwrap()
    });
}
