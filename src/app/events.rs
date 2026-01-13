use serde::Serialize;
use std::fmt;

use crate::engine::Event as EngineEvent;

#[derive(Debug, Clone, Serialize)]
pub enum Event {
    Engine(EngineEvent),
    End,
    TimerStarted(u64),
    TimerTick(u64),
    TimerEnded,
    InputChar(char),
    InputBackspace,
    InputEnter,
    Error(String),
    QuitRequested,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::Engine(event) => write!(f, "{event}"),
            Event::TimerStarted(s) => write!(f, "Timer started: {s}s"),
            Event::TimerTick(s) => write!(f, "Timer: {s}s"),
            Event::TimerEnded => write!(f, "Timer ended"),
            Event::Error(e) => write!(f, "Error: {e}"),
            Event::QuitRequested => write!(f, "Quit requested"),
            Event::InputChar(c) => write!(f, "Input: {c}"),
            Event::InputBackspace => write!(f, "Input: Backspace"),
            Event::InputEnter => write!(f, "Input: Enter"),
            Event::End => write!(f, "End game"),
        }
    }
}
