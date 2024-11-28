use app::{FlowFast, FlowFastState};
use relm4::RelmApp;

pub mod app;
pub mod components;
pub mod utils;

fn main() {
    let app = RelmApp::new("com.ljslobster.FlowFast");

    app.run::<FlowFast>(FlowFastState::Idle);
}
