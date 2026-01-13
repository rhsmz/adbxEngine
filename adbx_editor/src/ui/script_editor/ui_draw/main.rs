use super::super::ScriptEditor;
use super::content_draw::draw_script_content;
use super::header::draw_script_editor_header;
use bevy::prelude::*;
use bevy::ui::Interaction;

/// スクリプトエディタのUI描画
pub fn draw_script_editor(
    mut commands: Commands,
    mut script_editor: ResMut<ScriptEditor>,
    script_editor_panel_query: Query<Entity, (With<Name>, With<Node>)>,
    name_query: Query<&Name>,
) {
    // スクリプトエディタパネルを検索
    let panel_opt = script_editor_panel_query.iter().find(|&e| {
        if let Ok(name) = name_query.get(e) {
            name.as_str() == "ScriptEditorPanel"
        } else {
            false
        }
    });

    if let Some(panel) = panel_opt {
        // コンテンツが変更された場合、またはコンテンツエンティティが存在しない場合のみ更新
        let needs_update = script_editor.content_entity.is_none();

        if needs_update {
            // 既存のコンテンツを削除
            if let Some(content_entity) = script_editor.content_entity {
                if let Ok(mut entity_commands) = commands.get_entity(content_entity) {
                    entity_commands.despawn();
                }
            }

            let content_entity = commands
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(5.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    Name::new("ScriptEditorContent"),
                ))
                .with_children(|content: &mut ChildSpawnerCommands| {
                    // ヘッダー（ファイル名とボタン）
                    draw_script_editor_header(content, &script_editor);

                    // エディタエリア（スクリプトコンテンツ表示）
                    content
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(5.0)),
                                overflow: Overflow::clip_y(),
                                margin: UiRect::top(Val::Px(5.0)),
                                ..default()
                            },
                            BackgroundColor(if script_editor.is_focused {
                                Color::srgb(0.12, 0.12, 0.12)
                            } else {
                                Color::srgb(0.1, 0.1, 0.1)
                            }),
                            Interaction::default(),
                            Name::new("ScriptEditorArea"),
                        ))
                        .with_children(|editor_area: &mut ChildSpawnerCommands| {
                            // スクリプトコンテンツを表示（簡易実装：テキスト表示のみ）
                            let text_entity = editor_area
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        flex_grow: 1.0,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::FlexStart,
                                        ..default()
                                    },
                                    Name::new("ScriptEditorText"),
                                ))
                                .id();
                            script_editor.text_area_entity = Some(text_entity);
                        });
                })
                .id();

            // スクリプトコンテンツを表示
            let content_text = if script_editor.content.is_empty() {
                "// Luaスクリプトをここに記述してください\n// シンタックスハイライトは今後実装予定"
                    .to_string()
            } else {
                script_editor.content.clone()
            };

            if let Some(text_entity) = script_editor.text_area_entity {
                draw_script_content(&mut commands, text_entity, &script_editor, &content_text);
            }

            commands.entity(panel).add_child(content_entity);
            script_editor.content_entity = Some(content_entity);
        }
    }
}
