use super::preview_generation::{detect_preview_type, update_preview};
use super::RealtimePreview;
use bevy::prelude::*;

/// リアルタイムプレビューウィンドウの描画
pub fn draw_realtime_preview(
    mut commands: Commands,
    mut preview: ResMut<RealtimePreview>,
    code_editor: Res<crate::ui::code_editor::CodeEditor>,
    windows: Query<&Window>,
    preview_window_query: Query<Entity, (With<Name>, With<Node>)>,
    name_query: Query<&Name>,
) {
    if !preview.is_enabled {
        return;
    }

    // プレビューウィンドウを検索
    let window_opt = preview_window_query.iter().find(|&e| {
        if let Ok(name) = name_query.get(e) {
            name.as_str() == "RealtimePreviewWindow"
        } else {
            false
        }
    });

    // 現在のファイルの内容を取得
    let current_file = if let Some(active_file) = code_editor.open_files.get(code_editor.active_tab)
    {
        Some((active_file.path.clone(), active_file.content.clone()))
    } else if let Some(ref current_file) = code_editor.current_file {
        Some((current_file.clone(), code_editor.content.clone()))
    } else {
        None
    };

    if let Some((file_path, content)) = current_file {
        // ファイルタイプに基づいてプレビュータイプを決定
        let preview_type = detect_preview_type(&file_path);

        // プレビューを更新
        update_preview(
            &mut preview,
            file_path.clone(),
            content.clone(),
            preview_type,
        );

        // プレビューウィンドウが存在しない場合は作成
        if window_opt.is_none() && preview.preview_window_entity.is_none() {
            if let Some(window) = windows.iter().next() {
                let _window_size = Vec2::new(window.width(), window.height());

                let preview_window = commands
                    .spawn((
                        Node {
                            width: Val::Px(400.0),
                            height: Val::Px(300.0),
                            position_type: bevy::ui::PositionType::Absolute,
                            right: Val::Px(10.0),
                            bottom: Val::Px(10.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                        bevy::ui::BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
                        Name::new("RealtimePreviewWindow"),
                    ))
                    .with_children(|preview_window| {
                        // ヘッダー
                        preview_window
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(25.0),
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                                Name::new("PreviewWindowHeader"),
                            ))
                            .with_children(|header| {
                                header.spawn((
                                    Text::new("Preview"),
                                    bevy::text::TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    bevy::text::TextColor(Color::WHITE),
                                ));

                                // 閉じるボタン
                                header
                                    .spawn((
                                        Node {
                                            width: Val::Px(20.0),
                                            height: Val::Px(20.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
                                        bevy::ui::Interaction::default(),
                                        Name::new("PreviewWindowCloseButton"),
                                    ))
                                    .with_children(|button| {
                                        button.spawn((
                                            Text::new("×"),
                                            bevy::text::TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            bevy::text::TextColor(Color::WHITE),
                                        ));
                                    });
                            });

                        // プレビューコンテンツ
                        preview_window.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                padding: UiRect::all(Val::Px(5.0)),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                            Name::new("PreviewWindowContent"),
                        ));
                    })
                    .id();

                preview.preview_window_entity = Some(preview_window);
            }
        }

        // プレビューコンテンツを更新（既存のコンテンツノードを検索して更新）
        if let Some(preview_window_entity) = preview.preview_window_entity {
            if let Some(preview_content) = preview.get_preview(&file_path) {
                // 既存のコンテンツノードを検索
                let content_node_opt = preview_window_query.iter().find(|&e| {
                    if let Ok(name) = name_query.get(e) {
                        name.as_str() == "PreviewContent"
                    } else {
                        false
                    }
                });

                if content_node_opt.is_none() {
                    // コンテンツノードが存在しない場合は作成
                    if let Ok(mut entity_commands) = commands.get_entity(preview_window_entity) {
                        entity_commands.with_children(|preview_window| {
                            preview_window
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        flex_grow: 1.0,
                                        padding: UiRect::all(Val::Px(5.0)),
                                        overflow: Overflow::clip_y(),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                                    Name::new("PreviewContent"),
                                ))
                                .with_children(|content| {
                                    if let Some(ref rendered) = preview_content.rendered_content {
                                        content.spawn((
                                            Text::new(rendered),
                                            bevy::text::TextFont {
                                                font_size: 10.0,
                                                ..default()
                                            },
                                            bevy::text::TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        ));
                                    } else {
                                        if let Some(ref error) = preview_content.error {
                                            content.spawn((
                                                Text::new(&format!("Preview Error: {}", error)),
                                                bevy::text::TextFont {
                                                    font_size: 10.0,
                                                    ..default()
                                                },
                                                bevy::text::TextColor(Color::srgb(1.0, 0.3, 0.3)),
                                            ));
                                        } else {
                                            content.spawn((
                                                Text::new(
                                                    "Preview not available for this file type",
                                                ),
                                                bevy::text::TextFont {
                                                    font_size: 10.0,
                                                    ..default()
                                                },
                                                bevy::text::TextColor(Color::srgb(0.6, 0.6, 0.6)),
                                            ));
                                        }
                                    }
                                });
                        });
                    }
                }
                // 既存のコンテンツノードがある場合は、更新は次回の再描画で行う
            }
        }
    }
}
