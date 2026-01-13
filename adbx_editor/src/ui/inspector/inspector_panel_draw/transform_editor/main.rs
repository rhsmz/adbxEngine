use super::field_groups::{draw_position_fields, draw_rotation_fields, draw_scale_fields};
use bevy::prelude::*;

/// Transformコンポーネント専用エディタの描画
pub fn draw_transform_editor(
    parent: &mut ChildSpawnerCommands,
    entity: bevy::ecs::entity::Entity,
    transform: &Transform,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(5.0)),
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
            Name::new("TransformSection"),
            BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
        ))
        .with_children(|transform_section: &mut ChildSpawnerCommands| {
            // Transformヘッダー
            transform_section
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(25.0),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    Name::new("TransformHeader"),
                ))
                .with_children(|header| {
                    header.spawn((
                        Text::new("Transform"),
                        bevy::text::TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                    ));
                });

            // Position編集フィールド
            draw_position_fields(transform_section, entity, transform);

            // Rotation編集フィールド
            draw_rotation_fields(transform_section, entity, transform);

            // Scale編集フィールド
            draw_scale_fields(transform_section, entity, transform);
        });
}
