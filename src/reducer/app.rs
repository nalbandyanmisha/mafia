use crate::engine::Engine;
use crate::app::AppStatus;
use crate::action::Action;
use crate::action::app::AppAction;
use crate::action::engine::EngineAction;

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

pub fn reduce(mut state: AppState, action: Action) -> (AppState) {
    match action {
        Action::App(app) => handle_app(&mut state, app),
        Action::Engine(engine) => handle_engine(&mut state, engine)
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

fn handle_engine(state: &mut AppState, engine: EngineAction) {
    match engine {
        EngineAction::Join{ name } => {
            // TODO: do something
        }
    }
}