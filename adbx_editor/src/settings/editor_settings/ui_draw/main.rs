use bevy::prelude::*;
use super::super::resource::{EditorSettings, SettingsPanel};
use super::header::draw_settings_panel_header;
use super::editor_section::draw_editor_settings_section;

/// 設定パネルのUI描画
pub fn draw_settings_panel(
    mut commands: Commands,
    mut settings_panel: ResMut<SettingsPanel>,
    editor_settings: ResMut<EditorSettings>,
    project: Option<Res<crate::project::Project>>,
    windows: Query<&Window>,
    _settings_panel_query: Query<Entity, (With<Name>, With<Node>)>,
    _name_query: Query<&Name>,
) {
    if !settings_panel.is_open {
        // パネルが閉じられている場合、既存のパネルを削除
        if let Some(content_entity) = settings_panel.content_entity {
            if let Ok(mut entity_commands) = commands.get_entity(content_entity) {
                entity_commands.despawn();
            }
            settings_panel.content_entity = None;
        }
        return;
    }
    
    // 既にパネルが存在する場合はスキップ
    if settings_panel.content_entity.is_some() {
        return;
    }
    
    // ウィンドウサイズを取得
    let _window_size = if let Some(window) = windows.iter().next() {
        Vec2::new(window.width(), window.height())
    } else {
        Vec2::new(1920.0, 1080.0)
    };
    
    // モーダルウィンドウとして設定パネルを作成
    let panel_entity = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: bevy::ui::PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), // 半透明の背景
        Name::new("SettingsPanelOverlay"),
    )).with_children(|overlay| {
        // 設定パネル本体
        overlay.spawn((
            Node {
                width: Val::Px(600.0),
                height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            bevy::ui::BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
            Name::new("SettingsPanel"),
        )).with_children(|panel| {
            draw_settings_panel_header(panel);
            draw_settings_panel_content(panel, &editor_settings, project.as_ref());
        });
    }).id();
    
    settings_panel.content_entity = Some(panel_entity);
}

/// 設定パネルのコンテンツ描画
fn draw_settings_panel_content(
    parent: &mut ChildSpawnerCommands,
    editor_settings: &EditorSettings,
    project: Option<&Res<crate::project::Project>>,
) {
    use crate::settings::project_settings::ui_draw::{draw_project_settings_section, draw_settings_panel_footer};
    
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            overflow: Overflow::clip_y(),
            ..default()
        },
        Name::new("SettingsPanelContent"),
    )).with_children(|content| {
        draw_editor_settings_section(content, editor_settings);
        
        // プロジェクト設定セクション
        if let Some(project) = project {
            draw_project_settings_section(content, project);
        }
    });
    
    // フッター
    draw_settings_panel_footer(parent);
}
