use crate::engine::state::phase::Phase;
use crate::engine::state::round::RoundId;
use crate::engine::state::table::Table;

pub fn draw_table(table: &Table, phase: Phase, round: RoundId) -> Vec<String> {
    let mut view: Vec<String> = vec![];
    match phase {
        Phase::Lobby => {
            view.push(format!("Phase: Lobby, Round: {round}"));
            for (chair, player) in &table.chairs_to_players {
                if player.name.is_empty() {
                    view.push(format!("Chair: {:?} is unoccupied", chair.position));
                } else {
                    view.push(format!(
                        "Chair: {:?}, Player: {}",
                        chair.position, player.name,
                    ));
                }
            }
        }
        Phase::Night => {
            view.push(format!("Phase: Night, Round: {round}"));
            for (chair, player) in &table.chairs_to_players {
                view.push(format!(
                    "Chair: {:?}, Player: {}, Role: {:?}",
                    chair.position, player.name, player.role
                ));
            }
            if round == RoundId(0) {
                view.push(
                        "To assigne players their roles, use the 'show' command during the night phase so you can see each player's role privately.".to_string()
                    );
                view.push(
                        "After that give 5 seconds to Sheriff to investigate cityzens and 1 minute to Mafia to choose their strategy.".to_string()
                    );
                view.push(
                        "Once the time is up, proceed to the morning phase using the appropriate command.".to_string()
                    );
            }
        }
        Phase::Morning => {
            view.push(format!("Phase: Morning, Round: {round}"));
            if round == RoundId(0) {
                view.push(
                        "The game has begun! Welcome to the first day of Mafia. As this is a first morning, there are no shooting to announce.".to_string()
                    );
            }

            for (chair, player) in &table.chairs_to_players {
                if player.name.is_empty() {
                    view.push(format!("Chair: {:?} is unoccupied", chair.position));
                } else {
                    view.push(format!(
                        "Chair: {:?}, Player: {}, Status: {:?}, Warnings: {}",
                        chair.position, player.name, player.status, player.warnings
                    ));
                }
            }
        }
        Phase::Day => {
            view.push(format!("Phase: Day, Round: {round}"));
            for (chair, player) in &table.chairs_to_players {
                if player.name.is_empty() {
                    view.push(format!("Chair: {:?} is unoccupied", chair.position));
                } else {
                    view.push(format!(
                        "Chair: {:?}, Player: {}, Status: {:?}, Warnings: {}",
                        chair.position, player.name, player.status, player.warnings
                    ));
                }
            }
        }
        Phase::Voting => {
            view.push("Phase: Voting".to_string());
        }
    }
    view
}
