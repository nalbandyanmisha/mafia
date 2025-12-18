use crate::engine::state::phase::Phase;
use crate::engine::state::round::RoundId;

use crate::engine::GameView;

pub fn draw_table(state: &GameView) -> Vec<String> {
    let mut view: Vec<String> = vec![];
    let round_id = state.round_id;
    let seats = &state.seats;
    match state.phase {
        Phase::Lobby => {
            view.push(format!("Phase: Lobby, Round: {round_id}"));
            for seat in seats {
                if seat.name.is_empty() {
                    view.push(format!("Chair: {:?} is unoccupied", seat.chair));
                } else {
                    view.push(format!("Chair: {:?}, Player: {}", seat.chair, seat.name));
                }
            }
        }
        Phase::Night => {
            view.push(format!("Phase: Night, Round: {round_id}"));
            for seat in seats {
                view.push(format!(
                    "Chair: {:?}, Player: {}, Role: {:?}",
                    seat.chair, seat.name, seat.role
                ));
            }
            if round_id == RoundId(0) {
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
            view.push(format!("Phase: Morning, Round: {round_id}"));
            if round_id == RoundId(0) {
                view.push(
                        "The game has begun! Welcome to the first day of Mafia. As this is a first morning, there are no shooting to announce.".to_string()
                    );
            }

            for seat in seats {
                if seat.name.is_empty() {
                    view.push(format!("Chair: {:?} is unoccupied", seat.chair));
                } else {
                    view.push(format!(
                        "Chair: {:?}, Player: {}, Status: {:?}, Warnings: {}",
                        seat.chair, seat.name, seat.life_status, seat.warnings
                    ));
                }
            }
        }
        Phase::Day => {
            let speaker = state
                .current_speaker
                .map(|c| c.position().to_string())
                .unwrap_or_else(|| "No one is speaking".into());
            let nominations = &state.nominations;
            view.push(format!(
                "Phase: Day, Round: {round_id}, Current Speaker: {speaker}",
            ));
            view.push(format!(
                "Nominations this round: {:?}",
                nominations
                    .iter()
                    .map(|chair| format!("{chair:?}"))
                    .collect::<Vec<String>>()
            ));
            for seat in seats {
                if seat.name.is_empty() {
                    view.push(format!("Chair: {:?} is unoccupied", seat.chair));
                } else {
                    view.push(format!(
                        "Chair: {:?}, Player: {}, Status: {:?}, Warnings: {}",
                        seat.chair, seat.name, seat.life_status, seat.warnings
                    ));
                }
            }
        }
        Phase::Voting => {
            view.push("Phase: Voting".to_string());
            let nominations = &state.nominations;
            view.push(format!(
                "Nominations this round: {:?}",
                nominations
                    .iter()
                    .map(|chair| format!("{chair:?}"))
                    .collect::<Vec<String>>()
            ));
        }
    }
    view
}
