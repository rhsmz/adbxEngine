use super::resource::LogPanel;
use bevy::prelude::*;

/// ログパネルの描画
pub fn draw_log_panel(
    mut commands: Commands,
    mut log_panel: ResMut<LogPanel>,
    log_panel_query: Query<Entity, (With<Name>, With<Node>)>,
    name_query: Query<&Name>,
) {
    let panel_opt = log_panel_query.iter().find(|&e| {
        if let Ok(name) = name_query.get(e) {
            name.as_str() == "LogPanel"
        } else {
            false
        }
    });

    if let Some(panel) = panel_opt {
        // ログが追加された場合、またはコンテンツエンティティが存在しない場合のみ更新
        let needs_update = log_panel.content_entity.is_none() || !log_panel.logs.is_empty();

        if needs_update {
            // 既存のコンテンツを削除
            if let Some(content_entity) = log_panel.content_entity {
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
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    Name::new("LogPanelContent"),
                ))
                .with_children(|content: &mut ChildSpawnerCommands| {
                    // ログエントリを表示（最新のものから）
                    let logs: Vec<_> = log_panel.logs.iter().rev().take(100).collect();

                    for log_entry in logs.iter().rev() {
                        content
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Auto,
                                    flex_direction: FlexDirection::Row,
                                    padding: UiRect::all(Val::Px(2.0)),
                                    margin: UiRect::bottom(Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                                Name::new(format!("LogEntry_{}", log_entry.timestamp)),
                            ))
                            .with_children(|entry: &mut ChildSpawnerCommands| {
                                // ログレベル表示
                                entry
                                    .spawn((
                                        Node {
                                            width: Val::Px(60.0),
                                            height: Val::Auto,
                                            justify_content: JustifyContent::FlexStart,
                                            align_items: AlignItems::Center,
                                            padding: UiRect::right(Val::Px(5.0)),
                                            ..default()
                                        },
                                        Name::new("LogLevel"),
                                    ))
                                    .with_children(|level_node: &mut ChildSpawnerCommands| {
                                        level_node.spawn((
                                            Text::new(log_entry.level.prefix()),
                                            bevy::text::TextFont {
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            bevy::text::TextColor(log_entry.level.color()),
                                        ));
                                    });

                                // ログメッセージ
                                entry
                                    .spawn((
                                        Node {
                                            flex_grow: 1.0,
                                            height: Val::Auto,
                                            justify_content: JustifyContent::FlexStart,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        Name::new("LogMessage"),
                                    ))
                                    .with_children(|message_node: &mut ChildSpawnerCommands| {
                                        message_node.spawn((
                                            Text::new(&log_entry.message),
                                            bevy::text::TextFont {
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            bevy::text::TextColor(Color::WHITE),
                                        ));
                                    });
                            });
                    }
                })
                .id();

            commands.entity(panel).add_child(content_entity);
            log_panel.content_entity = Some(content_entity);
        }
    }
}

/// ログパネルの表示/非表示切り替え
#[allow(dead_code)]
pub fn toggle_log_panel(mut log_panel: ResMut<LogPanel>) {
    log_panel.is_visible = !log_panel.is_visible;
}
