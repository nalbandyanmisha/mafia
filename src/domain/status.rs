use std::str::FromStr;

/// Player status in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Alive,
    Dead,
    Eliminated,
    Removed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = (*self).into();
        write!(f, "{s}")
    }
}

impl From<Status> for &'static str {
    fn from(s: Status) -> Self {
        match s {
            Status::Alive => "alive",
            Status::Dead => "dead",
            Status::Eliminated => "eliminated",
            Status::Removed => "removed",
        }
    }
}

impl From<Status> for String {
    fn from(s: Status) -> Self {
        <&str>::from(s).to_string()
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "alive" => Ok(Status::Alive),
            "dead" => Ok(Status::Dead),
            "eliminated" => Ok(Status::Eliminated),
            "removed" => Ok(Status::Removed),
            other => Err(format!("Invalid status: {other}")),
        }
    }
}
