mod editor_app;
mod editor_main;
mod ui;
mod systems;
mod project;
mod communication;
mod error;
mod settings;

use crate::editor_main::app_initialization::initialize_app;
use crate::editor_main::system_registration::register_systems;

fn main() {
    let mut app = initialize_app();
    register_systems(&mut app);
    app.run();
}
