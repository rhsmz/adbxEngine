use bevy::prelude::*;
use crate::ui::code_editor::{CompletionState, CompletionKind};

/// 補完候補のポップアップを描画
pub fn draw_completion_popup(
    commands: &mut Commands,
    completion_state: &CompletionState,
    parent: Entity,
) {
    // 補完候補のポップアップを表示
    commands.entity(parent).with_children(|popup| {
        popup.spawn((
            Node {
                width: Val::Px(300.0),
                min_height: Val::Px(100.0),
                max_height: Val::Px(300.0),
                position_type: bevy::ui::PositionType::Absolute,
                left: Val::Px(50.0),
                top: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(2.0)),
                border: UiRect::all(Val::Px(1.0)),
                overflow: Overflow::clip_y(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
            bevy::ui::BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
            Name::new("CodeCompletionPopup"),
        )).with_children(|list| {
            for (i, candidate) in completion_state.candidates.iter().enumerate() {
                let is_selected = i == completion_state.selected_index;
                
                list.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(25.0),
                        padding: UiRect::horizontal(Val::Px(5.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(if is_selected {
                        Color::srgb(0.3, 0.3, 0.4)
                    } else {
                        Color::srgb(0.2, 0.2, 0.25)
                    }),
                    Name::new(format!("CompletionItem_{}", i)),
                )).with_children(|item| {
                    // アイコン（種類に応じて）
                    let icon = match candidate.kind {
                        CompletionKind::Keyword => "K",
                        CompletionKind::Function => "F",
                        CompletionKind::Variable => "V",
                        CompletionKind::Type => "T",
                        CompletionKind::Property => "P",
                        CompletionKind::Method => "M",
                        CompletionKind::Module => "M",
                    };
                    
                    item.spawn((
                        Node {
                            width: Val::Px(20.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::right(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                        Name::new(format!("CompletionIcon_{}", i)),
                    )).with_children(|icon_node| {
                        icon_node.spawn((
                            Text::new(icon),
                            bevy::text::TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                            bevy::text::TextColor(Color::srgb(0.7, 0.7, 0.7)),
                        ));
                    });
                    
                    // ラベル
                    item.spawn((
                        Text::new(&candidate.label),
                        bevy::text::TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        bevy::text::TextColor(if is_selected {
                            Color::WHITE
                        } else {
                            Color::srgb(0.9, 0.9, 0.9)
                        }),
                        Name::new(format!("CompletionLabel_{}", i)),
                    ));
                    
                    // 詳細情報（関数のシグネチャ、型情報、ドキュメントコメント）
                    if let Some(detail) = &candidate.detail {
                        // 詳細情報を行ごとに分割して表示
                        let detail_lines: Vec<&str> = detail.lines().collect();
                        if !detail_lines.is_empty() {
                            item.spawn((
                                Node {
                                    flex_grow: 1.0,
                                    height: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::left(Val::Px(10.0)),
                                    ..default()
                                },
                                Name::new(format!("CompletionDetailContainer_{}", i)),
                            )).with_children(|detail_container| {
                                for (line_idx, line) in detail_lines.iter().enumerate() {
                                    // 最初の行は関数シグネチャ（型情報）
                                    let color = if line_idx == 0 {
                                        Color::srgb(0.7, 0.8, 0.9) // シグネチャは少し明るい色
                                    } else {
                                        Color::srgb(0.6, 0.6, 0.6) // ドキュメントコメントは通常の色
                                    };
                                    
                                    detail_container.spawn((
                                        Text::new(*line),
                                        bevy::text::TextFont {
                                            font_size: 9.0,
                                            ..default()
                                        },
                                        bevy::text::TextColor(color),
                                        Name::new(format!("CompletionDetailLine_{}_{}", i, line_idx)),
                                    ));
                                }
                            });
                        }
                    }
                });
            }
        });
    });
}
