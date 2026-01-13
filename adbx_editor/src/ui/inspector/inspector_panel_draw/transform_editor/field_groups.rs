use super::super::super::inspector_panel_resource::TransformFieldType;
use super::field_display::draw_transform_field;
use bevy::prelude::*;

/// Position編集フィールドの描画
pub fn draw_position_fields(
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
                margin: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            Name::new("PositionField"),
        ))
        .with_children(|field_container| {
            field_container
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(20.0),
                    padding: UiRect::left(Val::Px(5.0)),
                    ..default()
                },))
                .with_children(|label_node| {
                    label_node.spawn((
                        Text::new("Position"),
                        bevy::text::TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });

            for (axis, val, field_type) in [
                (
                    "X",
                    transform.translation.x,
                    TransformFieldType::TranslationX,
                ),
                (
                    "Y",
                    transform.translation.y,
                    TransformFieldType::TranslationY,
                ),
                (
                    "Z",
                    transform.translation.z,
                    TransformFieldType::TranslationZ,
                ),
            ]
            .iter()
            {
                draw_transform_field(field_container, entity, *axis, *val, *field_type, false);
            }
        });
}

/// Rotation編集フィールドの描画
pub fn draw_rotation_fields(
    parent: &mut ChildSpawnerCommands,
    entity: bevy::ecs::entity::Entity,
    transform: &Transform,
) {
    let (roll, pitch, yaw) = transform.rotation.to_euler(bevy::math::EulerRot::XYZ);

    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(5.0)),
                margin: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            Name::new("RotationField"),
        ))
        .with_children(|field_container| {
            field_container
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(20.0),
                    padding: UiRect::left(Val::Px(5.0)),
                    ..default()
                },))
                .with_children(|label_node| {
                    label_node.spawn((
                        Text::new("Rotation"),
                        bevy::text::TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });

            for (axis, val, field_type) in [
                ("X", roll.to_degrees(), TransformFieldType::RotationX),
                ("Y", pitch.to_degrees(), TransformFieldType::RotationY),
                ("Z", yaw.to_degrees(), TransformFieldType::RotationZ),
            ]
            .iter()
            {
                draw_transform_field(field_container, entity, *axis, *val, *field_type, true);
            }
        });
}

/// Scale編集フィールドの描画
pub fn draw_scale_fields(
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
                margin: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            Name::new("ScaleField"),
        ))
        .with_children(|field_container| {
            field_container
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(20.0),
                    padding: UiRect::left(Val::Px(5.0)),
                    ..default()
                },))
                .with_children(|label_node| {
                    label_node.spawn((
                        Text::new("Scale"),
                        bevy::text::TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });

            for (axis, val, field_type) in [
                ("X", transform.scale.x, TransformFieldType::ScaleX),
                ("Y", transform.scale.y, TransformFieldType::ScaleY),
                ("Z", transform.scale.z, TransformFieldType::ScaleZ),
            ]
            .iter()
            {
                draw_transform_field(field_container, entity, *axis, *val, *field_type, false);
            }
        });
}
