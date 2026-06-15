pub mod app;
pub mod engine;

#[derive(Debug, Clone)]
pub enum Action {
    App(app::AppAction),
    Engine(engine::EngineAction)
}