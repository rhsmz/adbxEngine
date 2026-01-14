use super::super::resource::{DockingSystem, PanelPosition};
use bevy::prelude::*;

/// パネルの表示/非表示を切り替え
#[allow(dead_code)]
pub fn toggle_panel_visibility(mut docking: ResMut<DockingSystem>, panel_name: String) {
    if let Some(panel_state) = docking.panels.get_mut(&panel_name) {
        panel_state.is_visible = !panel_state.is_visible;
    }
}

/// パネルのドッキング状態を切り替え
#[allow(dead_code)]
pub fn toggle_panel_docking(
    mut docking: ResMut<DockingSystem>,
    panel_name: String,
    windows: Query<&Window>,
) {
    if let Some(panel_state) = docking.panels.get_mut(&panel_name) {
        panel_state.is_docked = !panel_state.is_docked;
        if !panel_state.is_docked {
            panel_state.position = PanelPosition::Floating;
            // フローティング位置が設定されていない場合、デフォルト位置を設定
            if panel_state.floating_position.is_none() {
                if let Some(window) = windows.iter().next() {
                    panel_state.floating_position = Some((
                        (window.width() - panel_state.size.0) / 2.0,
                        (window.height() - panel_state.size.1) / 2.0,
                    ));
                }
            }
        } else {
            // ドッキングに戻す場合、元の位置を復元（デフォルト位置を使用）
            match panel_name.as_str() {
                "HierarchyPanel" => panel_state.position = PanelPosition::Left,
                "InspectorPanel" => panel_state.position = PanelPosition::Right,
                "AssetBrowserPanel" | "ScriptEditorPanel" | "CodeEditorPanel" => {
                    panel_state.position = PanelPosition::Bottom;
                }
                _ => {}
            }
            panel_state.floating_position = None;
        }
    }
}
