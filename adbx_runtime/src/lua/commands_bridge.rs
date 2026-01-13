use bevy::prelude::*;
use std::collections::VecDeque;

/// LuaスクリプトからCommandsを実行するためのブリッジ
/// Commandsはシステム内でのみ使用可能なため、キュー経由で実行します
#[derive(Resource, Default)]
pub struct LuaCommandsBridge {
    pub command_queue: VecDeque<LuaCommand>,
}

#[derive(Debug, Clone)]
pub enum LuaCommand {
    SpawnEntity {
        components: Vec<String>,
    },
    DespawnEntity {
        entity_id: u32,
    },
    InsertComponent {
        entity_id: u32,
        component_type: String,
        data: String,
    },
    RemoveComponent {
        entity_id: u32,
        component_type: String,
    },
}

/// LuaCommandsBridgeからコマンドを実行するシステム
pub fn execute_lua_commands(
    mut bridge: ResMut<LuaCommandsBridge>,
    mut commands: Commands,
    query: Query<Entity>,
) {
    while let Some(cmd) = bridge.command_queue.pop_front() {
        match cmd {
            LuaCommand::SpawnEntity { .. } => {
                // Entityをspawn（簡易実装）
                let entity = commands.spawn_empty().id();
                bevy::log::debug!("Lua spawned entity: {:?}", entity);
            }
            LuaCommand::DespawnEntity { entity_id } => {
                // Entityをdespawn
                // 注意: entity_idはu32だが、Entityは異なる構造を持つ可能性がある
                // 簡易実装として、全Entityを検索
                for entity in query.iter() {
                    if entity.index() == entity_id {
                        commands.entity(entity).despawn();
                        break;
                    }
                }
            }
            LuaCommand::InsertComponent { entity_id, .. } => {
                // Componentを追加（簡易実装）
                for entity in query.iter() {
                    if entity.index() == entity_id {
                        bevy::log::debug!("Lua insert component to entity: {:?}", entity);
                        break;
                    }
                }
            }
            LuaCommand::RemoveComponent { entity_id, .. } => {
                // Componentを削除（簡易実装）
                for entity in query.iter() {
                    if entity.index() == entity_id {
                        bevy::log::debug!("Lua remove component from entity: {:?}", entity);
                        break;
                    }
                }
            }
        }
    }
}
