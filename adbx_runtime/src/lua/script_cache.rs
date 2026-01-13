use bevy::prelude::*;
use mlua::Lua;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// スクリプトの実行状態
/// 注意: Functionはライフタイムを持つため、直接保存できない
/// 代わりに、スクリプトがコンパイル済みかどうかのフラグのみを保持
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScriptExecutionState {
    /// スクリプトがコンパイル済みで、update関数が存在
    HasUpdate,
    /// スクリプトがコンパイル済みだが、update関数は存在しない
    NoUpdate,
}

/// スクリプトの実行頻度制限設定
#[derive(Debug, Clone, Copy)]
pub struct ScriptExecutionConfig {
    pub max_executions_per_second: f64, // 1秒あたりの最大実行回数
    pub last_execution_time: f64,       // 最後の実行時刻
}

impl Default for ScriptExecutionConfig {
    fn default() -> Self {
        Self {
            max_executions_per_second: 60.0, // デフォルトは60FPS
            last_execution_time: 0.0,
        }
    }
}

/// スクリプトのキャッシュ（コンパイル済み関数を保持）
#[derive(Resource, Default)]
pub struct ScriptCache {
    pub compiled_scripts: HashMap<Entity, Arc<Mutex<Option<ScriptExecutionState>>>>,
    pub script_hashes: HashMap<Entity, u64>,
    pub execution_configs: HashMap<Entity, ScriptExecutionConfig>, // エンティティごとの実行設定
    pub execution_timestamps: HashMap<Entity, f64>, // エンティティごとの最後の実行時刻
    pub skip_execution: HashMap<Entity, bool>, // 実行をスキップするかどうか（update関数がない場合など）
}

impl ScriptCache {
    pub fn get_or_compile(
        &mut self,
        entity: Entity,
        script_content: &str,
        lua: &Lua,
    ) -> mlua::Result<Option<ScriptExecutionState>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // スクリプトのハッシュを計算
        let mut hasher = DefaultHasher::new();
        script_content.hash(&mut hasher);
        let hash = hasher.finish();

        // ハッシュが変わっていない場合はキャッシュから取得
        if let Some(&old_hash) = self.script_hashes.get(&entity) {
            if old_hash == hash {
                if let Some(cached) = self.compiled_scripts.get(&entity) {
                    if let Ok(guard) = cached.lock() {
                        if let Some(state) = guard.as_ref() {
                            return Ok(Some(*state));
                        }
                    }
                }
            }
        }

        // スクリプトを実行してグローバル変数を初期化
        lua.load(script_content).exec()?;

        // スクリプトが実行された後、update関数があれば取得
        let globals = lua.globals();
        let execution_state = if globals.get::<_, mlua::Function>("update").is_ok() {
            ScriptExecutionState::HasUpdate
        } else {
            ScriptExecutionState::NoUpdate
        };

        // キャッシュに保存
        self.script_hashes.insert(entity, hash);
        self.compiled_scripts
            .insert(entity, Arc::new(Mutex::new(Some(execution_state))));

        Ok(Some(execution_state))
    }

    pub fn clear_entity(&mut self, entity: Entity) {
        self.compiled_scripts.remove(&entity);
        self.script_hashes.remove(&entity);
        self.execution_configs.remove(&entity);
        self.execution_timestamps.remove(&entity);
        self.skip_execution.remove(&entity);
    }

    /// エンティティの実行頻度を設定
    pub fn set_execution_rate(&mut self, entity: Entity, max_executions_per_second: f64) {
        let config = ScriptExecutionConfig {
            max_executions_per_second,
            last_execution_time: 0.0,
        };
        self.execution_configs.insert(entity, config);
    }
}
