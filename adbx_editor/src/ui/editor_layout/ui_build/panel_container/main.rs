use bevy::prelude::*;
use super::super::super::EditorLayout;
use super::panel_creation::{build_hierarchy_panel, build_scene_view_area, build_inspector_panel, build_asset_browser_panel, build_code_editor_panel, build_script_editor_panel, build_log_panel};

/// パネルコンテナの構築
pub fn build_panel_containers(parent: &mut ChildSpawnerCommands, layout: &EditorLayout) {
    // メインコンテンツエリア
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        Name::new("MainContent"),
    )).with_children(|content| {
        // ヒエラルキーパネル（左側）
        build_hierarchy_panel(content, layout);
        
        // 中央エリア（シーンビュー）
        build_scene_view_area(content);
        
        // インスペクターパネル（右側）
        build_inspector_panel(content, layout);
    });
    
    // 下部エリア（アセットブラウザー、スクリプトエディタ、ログパネル）
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(layout.asset_browser_height),
            flex_direction: FlexDirection::Row,
            border: UiRect::top(Val::Px(1.0)),
            ..default()
        },
        Name::new("BottomPanel"),
    )).with_children(|bottom| {
        // アセットブラウザー（左側）
        build_asset_browser_panel(bottom);
        
        // コードエディタ（中央）
        build_code_editor_panel(bottom);
        
        // スクリプトエディタ（右側）
        build_script_editor_panel(bottom);
        
        // ログパネル（右端）
        build_log_panel(bottom);
    });
}
