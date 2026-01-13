mod communication;
mod editor_app;
mod editor_main;
mod error;
mod project;
mod settings;
mod systems;
mod ui;

use crate::editor_main::app_initialization::initialize_app;
use crate::editor_main::system_registration::register_systems;

fn main() {
    let mut app = initialize_app();
    register_systems(&mut app);
    app.run();
}
