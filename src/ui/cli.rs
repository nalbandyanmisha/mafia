use crate::engine::state::phase::Phase;
use crate::engine::state::round::{Round, RoundId};
use crate::engine::state::table::{Table, chair::Chair};

pub fn draw_table(
    table: &Table,
    phase: Phase,
    round_id: RoundId,
    round: Round,
    current_speaker: Option<Chair>,
) -> Vec<String> {
    let mut view: Vec<String> = vec![];
    match phase {
        Phase::Lobby => {
            view.push(format!("Phase: Lobby, Round: {round_id}"));
            for (chair, player) in table.all_chairs() {
                if player.name().is_empty() {
                    view.push(format!("Chair: {:?} is unoccupied", chair));
                } else {
                    view.push(format!("Chair: {:?}, Player: {}", chair, player.name()));
                }
            }
        }
        Phase::Night => {
            view.push(format!("Phase: Night, Round: {round_id}"));
            for (chair, player) in table.all_chairs() {
                view.push(format!(
                    "Chair: {:?}, Player: {}, Role: {:?}",
                    chair,
                    player.name(),
                    player.role()
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

            for (chair, player) in table.all_chairs() {
                if player.name().is_empty() {
                    view.push(format!("Chair: {:?} is unoccupied", chair));
                } else {
                    view.push(format!(
                        "Chair: {:?}, Player: {}, Status: {:?}, Warnings: {}",
                        chair,
                        player.name(),
                        player.status(),
                        player.warnings()
                    ));
                }
            }
        }
        Phase::Day => {
            let speaker = current_speaker
                .map(|c| c.position().to_string())
                .unwrap_or_else(|| "No one is speaking".into());
            let nominations = round.get_nominations();
            view.push(format!(
                "Phase: Day, Round: {round_id}, Current Speaker: {speaker}",
            ));
            view.push(format!(
                "Nominations this round: {:?}",
                nominations
                    .iter()
                    .map(|chair| format!("{:?}", chair))
                    .collect::<Vec<String>>()
            ));
            for (chair, player) in table.all_chairs() {
                if player.name().is_empty() {
                    view.push(format!("Chair: {:?} is unoccupied", chair));
                } else {
                    view.push(format!(
                        "Chair: {:?}, Player: {}, Status: {:?}, Warnings: {}",
                        chair,
                        player.name(),
                        player.status(),
                        player.warnings()
                    ));
                }
            }
        }
        Phase::Voting => {
            view.push("Phase: Voting".to_string());
            let nominations = round.get_nominations();
            view.push(format!(
                "Nominations this round: {:?}",
                nominations
                    .iter()
                    .map(|chair| format!("{:?}", chair))
                    .collect::<Vec<String>>()
            ));
        }
    }
    view
}
