#[derive(Debug)]
pub enum Event {
    EngineUpdated,
    TimerStarted(u64),
    TimerTick(u64),
    TimerEnded,
    InputChar(char),
    InputBackspace,
    InputEnter,
    Error(String),
    QuitRequested,
}
