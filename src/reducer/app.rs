use crate::engine::Engine;
use crate::app::AppStatus;
use crate::action::Action;
use crate::action::app::AppAction;
use crate::engine::EngineEvent;
use crate::effect::Effect;
use crate::engine::commands::Command;

pub struct AppState {
    pub engine: Engine,
    pub status: AppStatus,
}

impl AppState {
    pub fn new() -> Self {
        AppState{
            engine: Engine::new(),
            status: AppStatus::Running
        }
    }
}

pub fn reduce(mut state: AppState, action: Action) -> AppState {
    let mut effects = Vec::new();
    match action {
        Action::App(app) => handle_app(&mut state, app),
        Action::Engine(engine_action) => {
            // TODO: the idea is instead of Command use EngineAction
            // because currently role assigning is the Effect (application related Enum)
            // we should have assignment of role as an ordinary Action to have it in the Events
            // I'll refactor "apply" function later.
            let engine_events = state.engine.apply(Command::Start);
            handle_engine_result(&mut state, engine_events, &mut effects)
        }
    }
    state
}

fn handle_app(state: &mut AppState, app: AppAction) {
    match app {
        AppAction::QuitRequested => {
            state.status = AppStatus::Quit;
        }
    }
}

fn handle_engine_result(state: &mut AppState, result: anyhow::Result<Vec<EngineEvent>>, effects: &mut Vec<Effect>) {

}