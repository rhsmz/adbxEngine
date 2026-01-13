use crate::ui::inspector::component_editor_registry::ComponentEditor;
use bevy::prelude::*;

/// Transformエディタの実装
pub struct TransformEditor;

impl ComponentEditor for TransformEditor {
    fn draw_ui(&self, commands: &mut Commands, _entity: Entity, parent_ui: Entity) {
        // TransformエディタのUIを描画
        // この実装は、draw_inspector_panel内のTransform描画ロジックと統合されています
        // ここでは、カスタムエディタの例として実装
        commands
            .entity(parent_ui)
            .with_children(|parent: &mut ChildSpawnerCommands| {
                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        Name::new("TransformEditor"),
                    ))
                    .with_children(|transform_editor: &mut ChildSpawnerCommands| {
                        transform_editor.spawn((
                            Text::new("Transform (Custom Editor)"),
                            bevy::text::TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            bevy::text::TextColor(Color::WHITE),
                        ));
                    });
            });
    }

    fn component_name(&self) -> &'static str {
        "Transform"
    }
}
