use bevy::prelude::*;

/// ドッキングレイアウトを読み込む
pub fn load_docking_layout(mut docking: ResMut<crate::ui::docking::DockingSystem>) {
    if let Err(e) = crate::ui::docking::load_layout(&mut docking) {
        bevy::log::warn!("Failed to load docking layout: {}. Using default layout.", e);
    }
}
