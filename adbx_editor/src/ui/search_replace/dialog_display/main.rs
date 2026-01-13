use bevy::prelude::*;
use super::super::resource::SearchReplace;
use super::title::draw_dialog_title;
use super::search_input::draw_search_input;
use super::replace_input::draw_replace_input;
use super::options::draw_search_options;
use super::buttons::draw_dialog_buttons;

/// 検索・置換ダイアログを描画
pub fn draw_search_replace(
    mut commands: Commands,
    mut search_replace: ResMut<SearchReplace>,
    _code_editor: Res<crate::ui::code_editor::CodeEditor>,
    _interaction_query: Query<(&bevy::ui::Interaction, &Name), Changed<bevy::ui::Interaction>>,
    _keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // 既存のダイアログを削除
    if let Some(dialog_entity) = search_replace.dialog_entity {
        if let Ok(mut entity_commands) = commands.get_entity(dialog_entity) {
            entity_commands.despawn();
        }
    }
    
    if !search_replace.is_search_visible {
        search_replace.dialog_entity = None;
        return;
    }
    
    // ダイアログを表示
    let dialog_entity = commands.spawn((
        Node {
            width: Val::Px(400.0),
            min_height: Val::Px(if search_replace.is_replace_visible { 150.0 } else { 100.0 }),
            position_type: bevy::ui::PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Px(50.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        bevy::ui::BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
        Name::new("SearchReplaceDialog"),
    )).with_children(|dialog: &mut ChildSpawnerCommands| {
        draw_dialog_title(dialog, &search_replace);
        draw_search_input(dialog, &search_replace);
        draw_replace_input(dialog, &search_replace);
        draw_search_options(dialog, &search_replace);
        draw_dialog_buttons(dialog, &search_replace);
    }).id();
    
    search_replace.dialog_entity = Some(dialog_entity);
}
