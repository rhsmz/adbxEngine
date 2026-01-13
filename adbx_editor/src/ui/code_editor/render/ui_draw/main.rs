use bevy::prelude::*;
use super::super::super::resource::CodeEditor;
use super::change_detection::{check_if_update_needed, update_change_detection};
use super::tab_bar::draw_tab_bar;
use super::editor_area::{create_editor_area_entity, draw_text_area, draw_text_area_content};

/// コードエディタのUI描画
pub fn draw_code_editor_ui(
    mut commands: Commands,
    mut code_editor: ResMut<CodeEditor>,
    code_editor_panel_query: Query<Entity, (With<Name>, With<Node>)>,
    name_query: Query<&Name>,
) {
    // コードエディタパネルを検索
    let panel_opt = code_editor_panel_query.iter().find(|&e| {
        if let Ok(name) = name_query.get(e) {
            name.as_str() == "CodeEditorPanel"
        } else {
            false
        }
    });
    
    if let Some(panel) = panel_opt {
        // 変更検知
        let needs_update = check_if_update_needed(&mut code_editor);
        
        if needs_update {
            update_change_detection(&mut code_editor);
            
            // 既存のコンテンツを削除
            if let Some(content_entity) = code_editor.content_entity {
                if let Ok(mut entity_commands) = commands.get_entity(content_entity) {
                    entity_commands.despawn();
                }
            }
        
            // コンテンツエンティティを作成
            let content_entity_id = create_content_entity(&mut commands);
            
            // エディタエリアエンティティを作成
            let editor_area_entity = create_editor_area_entity(&mut commands);
            
            // コンテンツの子要素を追加
            commands.entity(content_entity_id).with_children(|content: &mut ChildSpawnerCommands| {
                draw_tab_bar(content, &code_editor);
            });
            
            // エディタエリアを子として追加
            commands.entity(content_entity_id).add_child(editor_area_entity);
            
            // エディタエリアの子要素を追加
            commands.entity(editor_area_entity).with_children(|editor_area: &mut ChildSpawnerCommands| {
                draw_ai_progress(editor_area, &code_editor);
                draw_text_area(editor_area, &code_editor);
            });
            
            // テキストエリアの内容を描画
            draw_text_area_content(&mut commands, &mut code_editor, editor_area_entity);
            
            // 補完候補を表示
            if code_editor.completion_state.is_visible && !code_editor.completion_state.candidates.is_empty() {
                crate::ui::code_editor_completion::draw_completion_popup(&mut commands, &code_editor.completion_state, editor_area_entity);
            }
            
            commands.entity(panel).add_child(content_entity_id);
            code_editor.content_entity = Some(content_entity_id);
        }
    }
}

/// コンテンツエンティティを作成
fn create_content_entity(commands: &mut Commands) -> Entity {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(5.0)),
            overflow: Overflow::clip(),
            ..default()
        },
        Name::new("CodeEditorContent"),
    )).id()
}

/// AI処理の進行状況を表示
fn draw_ai_progress(parent: &mut ChildSpawnerCommands, code_editor: &CodeEditor) {
    #[cfg(feature = "ai")]
    if let Some(ref progress) = code_editor.ai_progress {
        let bg_color = if progress.contains("failed") || progress.contains("Failed") {
            Color::srgb(0.6, 0.2, 0.2)
        } else if progress.contains("completed") || progress.contains("Completed") {
            Color::srgb(0.2, 0.6, 0.2)
        } else {
            Color::srgb(0.2, 0.4, 0.6)
        };
        
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(5.0)),
                margin: UiRect::bottom(Val::Px(5.0)),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(bg_color),
            Name::new("AiProgressIndicator"),
        )).with_children(|progress_node: &mut ChildSpawnerCommands| {
            progress_node.spawn((
                Text::new("⏳"),
                bevy::text::TextFont {
                    font_size: 14.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
                Name::new("AiProgressSpinner"),
            ));
            
            progress_node.spawn((
                Text::new(progress),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(Color::WHITE),
                Name::new("AiProgressText"),
            ));
        });
    }
}
