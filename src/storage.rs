use chrono::Local;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

/// Returns ~/.local/share/mafia (or platform equivalent)
fn app_data_dir() -> PathBuf {
    let proj = ProjectDirs::from("org", "misha", "mafia")
        .expect("Failed to determine project directories");

    let dir = proj.data_dir();

    fs::create_dir_all(dir).expect("Failed to create application data directory");

    dir.to_path_buf()
}

/// Returns a unique timestamped save file path
///
/// Example:
/// game_2026-01-28_21-14-03.json
pub fn timestamped_save_path() -> PathBuf {
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let filename = format!("game_{timestamp}.json");

    app_data_dir().join(filename)
}
