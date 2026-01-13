use super::super::resource::{DockingSystem, PanelState};
use bevy::prelude::*;

/// レイアウトを保存
pub fn save_layout(docking: &DockingSystem) -> Result<(), String> {
    use serde_json;

    // HashMapをVec<(String, PanelState)>に変換
    let layout_data: Vec<(String, PanelState)> = docking
        .panels
        .iter()
        .map(|(name, state)| (name.clone(), state.clone()))
        .collect();

    let layout_json = serde_json::to_string_pretty(&layout_data)
        .map_err(|e| format!("Failed to serialize layout: {}", e))?;

    // 設定ディレクトリに保存
    let config_dir = crate::settings::save_load::SettingsManager::get_config_dir()
        .map_err(|e| format!("Failed to get config directory: {}", e))?;
    let layout_path = config_dir.join("layout.json");

    std::fs::write(&layout_path, layout_json)
        .map_err(|e| format!("Failed to write layout file: {}", e))?;

    bevy::log::info!("Layout saved to: {:?}", layout_path);

    Ok(())
}

/// レイアウトを読み込み
pub fn load_layout(docking: &mut DockingSystem) -> Result<(), String> {
    let config_dir = crate::settings::save_load::SettingsManager::get_config_dir()
        .map_err(|e| format!("Failed to get config directory: {}", e))?;
    let layout_path = config_dir.join("layout.json");

    if !layout_path.exists() {
        bevy::log::info!("Layout file not found, using default layout");
        return Ok(());
    }

    let layout_content = std::fs::read_to_string(&layout_path)
        .map_err(|e| format!("Failed to read layout file: {}", e))?;

    // レイアウトデータをパース
    // 保存形式: Vec<(&String, &PanelState)> を Vec<(String, PanelState)> に変換
    let layout_data: Vec<(String, PanelState)> = serde_json::from_str(&layout_content)
        .map_err(|e| format!("Failed to parse layout file: {}", e))?;

    // パネル状態を適用
    for (panel_name, panel_state) in layout_data {
        docking.panels.insert(panel_name.clone(), panel_state);
        bevy::log::info!("Loaded layout for panel: {}", panel_name);
    }

    bevy::log::info!("Layout loaded successfully from: {:?}", layout_path);

    Ok(())
}

/// レイアウトを適用するシステム（エディタレイアウトとドッキングシステムを連携）
pub fn apply_layout_to_editor(
    docking: Res<DockingSystem>,
    mut editor_layout: ResMut<crate::ui::editor_layout::EditorLayout>,
) {
    // ドッキングシステムのパネル状態をEditorLayoutに反映
    if let Some(hierarchy_panel) = docking.panels.get("HierarchyPanel") {
        if hierarchy_panel.is_visible {
            editor_layout.hierarchy_width = hierarchy_panel.size.0;
        }
    }

    if let Some(inspector_panel) = docking.panels.get("InspectorPanel") {
        if inspector_panel.is_visible {
            editor_layout.inspector_width = inspector_panel.size.0;
        }
    }

    if let Some(asset_browser_panel) = docking.panels.get("AssetBrowserPanel") {
        if asset_browser_panel.is_visible {
            editor_layout.asset_browser_height = asset_browser_panel.size.1;
        }
    }
}
