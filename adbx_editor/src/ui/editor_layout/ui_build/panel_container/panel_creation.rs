use super::super::super::EditorLayout;
use super::panel_header::build_panel_header;
use bevy::prelude::*;

/// ヒエラルキーパネルの構築
pub fn build_hierarchy_panel(parent: &mut ChildSpawnerCommands, layout: &EditorLayout) {
    parent
        .spawn((
            Node {
                width: Val::Px(layout.hierarchy_width),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::right(Val::Px(1.0)),
                ..default()
            },
            Name::new("HierarchyPanel"),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            bevy::ui::BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
        ))
        .with_children(|hierarchy| {
            build_panel_header(hierarchy, "Hierarchy", "HierarchyPanel");

            // パネルコンテンツ
            hierarchy.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                Name::new("HierarchyPanelContent"),
            ));
        });
}

/// シーンビューエリアの構築
pub fn build_scene_view_area(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Name::new("SceneViewArea"),
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|scene| {
            scene.spawn((
                Text::new("Scene View"),
                bevy::text::TextFont {
                    font_size: 16.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
}

/// インスペクターパネルの構築
pub fn build_inspector_panel(parent: &mut ChildSpawnerCommands, layout: &EditorLayout) {
    parent
        .spawn((
            Node {
                width: Val::Px(layout.inspector_width),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::left(Val::Px(1.0)),
                ..default()
            },
            Name::new("InspectorPanel"),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            bevy::ui::BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
        ))
        .with_children(|inspector| {
            build_panel_header(inspector, "Inspector", "InspectorPanel");

            // パネルコンテンツ
            inspector.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                Name::new("InspectorPanelContent"),
            ));
        });
}

/// アセットブラウザーパネルの構築
pub fn build_asset_browser_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(33.33),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::right(Val::Px(1.0)),
                ..default()
            },
            Name::new("AssetBrowserPanel"),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            bevy::ui::BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
        ))
        .with_children(|browser| {
            build_panel_header(browser, "Asset Browser", "AssetBrowserPanel");

            // パネルコンテンツ
            browser.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                Name::new("AssetBrowserPanelContent"),
            ));
        });
}

/// コードエディタパネルの構築
pub fn build_code_editor_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(33.33),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::right(Val::Px(1.0)),
                ..default()
            },
            Name::new("CodeEditorPanel"),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            bevy::ui::BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
        ))
        .with_children(|code_editor| {
            build_panel_header(code_editor, "Code Editor", "CodeEditorPanel");

            // パネルコンテンツ
            code_editor.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                Name::new("CodeEditorPanelContent"),
            ));
        });
}

/// スクリプトエディタパネルの構築
pub fn build_script_editor_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(33.33),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::right(Val::Px(1.0)),
                ..default()
            },
            Name::new("ScriptEditorPanel"),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            bevy::ui::BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
        ))
        .with_children(|script_editor| {
            build_panel_header(script_editor, "Script Editor", "ScriptEditorPanel");

            // パネルコンテンツ
            script_editor.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                Name::new("ScriptEditorPanelContent"),
            ));
        });
}

/// ログパネルの構築
pub fn build_log_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(33.34),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            Name::new("LogPanel"),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        ))
        .with_children(|log_panel| {
            log_panel.spawn((
                Text::new("Log"),
                bevy::text::TextFont {
                    font_size: 16.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
}
