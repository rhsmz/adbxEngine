use bevy::prelude::*;
use super::super::super::inspector_panel_resource::{TransformInputField, TransformFieldType};

/// Transformフィールドの描画（共通）
pub fn draw_transform_field(
    parent: &mut ChildSpawnerCommands,
    entity: bevy::ecs::entity::Entity,
    axis: &str,
    val: f32,
    field_type: TransformFieldType,
    is_rotation: bool,
) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(25.0),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(2.0)),
            margin: UiRect::top(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
    )).with_children(|field_row| {
        field_row.spawn((
            Node {
                width: Val::Px(20.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        )).with_children(|axis_label| {
            axis_label.spawn((
                Text::new(axis),
                bevy::text::TextFont { font_size: 10.0, ..default() },
                bevy::text::TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
        
        let field_name = if is_rotation {
            format!("Rotation{}Field", axis)
        } else if field_type == TransformFieldType::TranslationX || 
                  field_type == TransformFieldType::TranslationY || 
                  field_type == TransformFieldType::TranslationZ {
            format!("Position{}Field", axis)
        } else {
            format!("Scale{}Field", axis)
        };
        
        field_row.spawn((
            Node {
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            TransformInputField { entity, field_type },
            Name::new(field_name),
        )).with_children(|value_node| {
            let value_text = if is_rotation {
                format!("{:.2}°", val)
            } else {
                format!("{:.2}", val)
            };
            
            value_node.spawn((
                Text::new(&value_text),
                bevy::text::TextFont { font_size: 11.0, ..default() },
                bevy::text::TextColor(Color::WHITE),
            ));
        });
    });
}
