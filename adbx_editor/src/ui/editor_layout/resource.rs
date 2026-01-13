use bevy::prelude::*;

/// エディタのレイアウト管理
#[derive(Resource)]
pub struct EditorLayout {
    pub hierarchy_width: f32,
    pub inspector_width: f32,
    pub asset_browser_height: f32,
}

impl Default for EditorLayout {
    fn default() -> Self {
        Self {
            hierarchy_width: 250.0,
            inspector_width: 300.0,
            asset_browser_height: 200.0,
        }
    }
}
