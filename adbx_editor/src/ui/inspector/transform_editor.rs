use crate::ui::inspector::inspector_panel_resource::{TransformFieldType, TransformInputField};
use bevy::prelude::*;

/// Transform編集フィールドの描画（Vec3フィールドの生成）
pub fn spawn_transform_vector3_edit_field(
    commands: &mut Commands,
    label: &str,
    value: Vec3,
    entity: Entity,
    field_x: TransformFieldType,
    field_y: TransformFieldType,
    field_z: TransformFieldType,
) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(5.0)),
                margin: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            Name::new(format!("{}Field", label)),
        ))
        .with_children(|field_container: &mut ChildSpawnerCommands| {
            // ラベル
            field_container
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(20.0),
                    padding: UiRect::left(Val::Px(5.0)),
                    ..default()
                },))
                .with_children(|label_node: &mut ChildSpawnerCommands| {
                    label_node.spawn((
                        Text::new(label),
                        bevy::text::TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });

            // X, Y, Zフィールド
            let fields = [
                ("X", value.x, field_x),
                ("Y", value.y, field_y),
                ("Z", value.z, field_z),
            ];

            for (axis, val, field_type) in fields.iter() {
                field_container
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(25.0),
                            flex_direction: FlexDirection::Row,
                            padding: UiRect::all(Val::Px(2.0)),
                            margin: UiRect::top(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ))
                    .with_children(|field_row: &mut ChildSpawnerCommands| {
                        // 軸ラベル
                        field_row
                            .spawn((Node {
                                width: Val::Px(20.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },))
                            .with_children(|axis_label: &mut ChildSpawnerCommands| {
                                axis_label.spawn((
                                    Text::new(*axis),
                                    bevy::text::TextFont {
                                        font_size: 10.0,
                                        ..default()
                                    },
                                    bevy::text::TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                ));
                            });

                        // 値表示（編集可能にするには、TextInputコンポーネントが必要）
                        field_row
                            .spawn((
                                Node {
                                    flex_grow: 1.0,
                                    height: Val::Percent(100.0),
                                    padding: UiRect::all(Val::Px(3.0)),
                                    ..default()
                                },
                                TransformInputField {
                                    entity,
                                    field_type: *field_type,
                                },
                                Name::new(format!("{}{}Field", label, axis)),
                            ))
                            .with_children(|value_node: &mut ChildSpawnerCommands| {
                                value_node.spawn((
                                    Text::new(&format!("{:.2}", val)),
                                    bevy::text::TextFont {
                                        font_size: 11.0,
                                        ..default()
                                    },
                                    bevy::text::TextColor(Color::WHITE),
                                ));
                            });
                    });
            }
        })
        .id()
}
