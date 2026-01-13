use bevy::prelude::*;
use crate::ui::inspector::component_editor_registry::ComponentEditor;

/// Camera2dエディタの実装
pub struct Camera2dEditor;

impl ComponentEditor for Camera2dEditor {
    fn draw_ui(
        &self,
        commands: &mut Commands,
        _entity: Entity,
        parent_ui: Entity,
    ) {
        commands.entity(parent_ui).with_children(|parent: &mut ChildSpawnerCommands| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(5.0)),
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                Name::new("Camera2dEditor"),
                BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
            )).with_children(|camera_editor: &mut ChildSpawnerCommands| {
                camera_editor.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(25.0),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    Name::new("Camera2dHeader"),
                )).with_children(|header: &mut ChildSpawnerCommands| {
                    header.spawn((
                        Text::new("Camera 2D"),
                        bevy::text::TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::WHITE),
                    ));
                });
                
                camera_editor.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(20.0),
                        padding: UiRect::left(Val::Px(10.0)),
                        ..default()
                    },
                )).with_children(|info: &mut ChildSpawnerCommands| {
                    info.spawn((
                        Text::new("2D Camera Component"),
                        bevy::text::TextFont {
                            font_size: 11.0,
                            ..default()
                        },
                        bevy::text::TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });
            });
        });
    }
    
    fn component_name(&self) -> &'static str {
        "Camera2d"
    }
}
