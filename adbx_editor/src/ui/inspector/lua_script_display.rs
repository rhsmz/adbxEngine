use adbx_shared::components::{LuaScript, LuaScriptState};
use bevy::prelude::*;

/// Luaスクリプトコンポーネントの表示
pub fn draw_lua_script_component(
    commands: &mut Commands,
    entity: Entity,
    parent_ui: Entity,
    lua_script: &LuaScript,
    lua_script_state: Option<&LuaScriptState>,
) {
    commands.entity(parent_ui).with_children(|parent| {
        parent
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(8.0)),
                    margin: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                Name::new("LuaScriptEditor"),
                BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
            ))
            .with_children(|lua_script_editor| {
                // スクリプトパス表示
                lua_script_editor
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(4.0)),
                            ..default()
                        },
                        Name::new("LuaScriptPath"),
                    ))
                    .with_children(|path_row| {
                        path_row.spawn((
                            Text::new(format!("Script: {}", lua_script.script_path.display())),
                            Name::new("LuaScriptPathText"),
                        ));
                    });

                // スクリプト状態表示
                if let Some(state) = lua_script_state {
                    let state_text = match state {
                        LuaScriptState::Loaded => "Loaded",
                        LuaScriptState::Running => "Running",
                        LuaScriptState::Error => "Error",
                    };
                    let state_color = match state {
                        LuaScriptState::Loaded => {
                            Color::srgb(0.0, 1.0, 0.0)
                        }
                        LuaScriptState::Running => {
                            Color::srgb(0.0, 0.5, 1.0)
                        }
                        LuaScriptState::Error => {
                            Color::srgb(1.0, 0.0, 0.0)
                        }
                    };
                    lua_script_editor
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(4.0)),
                                ..default()
                            },
                            Name::new("LuaScriptState"),
                        ))
                        .with_children(|state_row| {
                            state_row.spawn((
                                Text::new(format!("State: {}", state_text)),
                                TextColor(state_color),
                                Name::new("LuaScriptStateText"),
                            ));
                        });
                }

                // スクリプト削除ボタン
                lua_script_editor
                    .spawn((
                        Button,
                        Node {
                            padding: UiRect::all(Val::Px(8.0)),
                            margin: UiRect::top(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.5, 0.0, 0.0)),
                        Name::new(format!("DetachScriptButton_{}", entity.index())),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("Detach Script"),
                            Name::new("DetachScriptButtonText"),
                        ));
                    });
            });
    });
}
