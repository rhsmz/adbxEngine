use bevy::prelude::*;

/// エディタアプリケーションのメインリソース
#[derive(Resource, Reflect)]
#[reflect(Resource)]
#[allow(dead_code)]
pub struct EditorApp {
    pub current_scene: Option<String>,
    pub selected_entity: Option<Entity>,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            current_scene: None,
            selected_entity: None,
        }
    }
}
