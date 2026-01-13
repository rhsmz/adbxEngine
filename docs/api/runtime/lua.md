# Luaスクリプトシステム API リファレンス

`adbx_runtime`クレートのLuaスクリプトシステムに関するAPIリファレンスです。

## 概要

Luaスクリプトシステムは、Bevy ECSにLuaスクリプトを統合する機能を提供します。エンティティに`LuaScript`コンポーネントをアタッチすることで、Luaスクリプトを実行できます。

## LuaVm

Lua VMリソースを表す構造体です。スレッドセーフなLua VMインスタンスを保持します。

### フィールド

- **`lua: Arc<Mutex<Lua>>`**: スレッドセーフなLua VMインスタンス

### 初期化

```rust
use adbx_runtime::lua::vm::LuaVm;
use std::sync::{Arc, Mutex};

let commands_bridge = Arc::new(Mutex::new(LuaCommandsBridge::default()));
let transform_bridge = Arc::new(Mutex::new(TransformBridge::default()));
let resource_bridge = Arc::new(Mutex::new(ResourceBridge::default()));
let event_bridge = Arc::new(Mutex::new(EventBridge::default()));
let component_bridge = Arc::new(Mutex::new(ComponentBridge::default()));

let lua_vm = LuaVm::new(
    commands_bridge,
    transform_bridge,
    resource_bridge,
    event_bridge,
    component_bridge,
);
```

## LuaScript

エンティティにアタッチされるLuaスクリプトコンポーネントです。

### フィールド

- **`script_path: PathBuf`**: スクリプトファイルのパス
- **`script_content: String`**: スクリプトの内容

### メソッド

#### `new(script_path: PathBuf, script_content: String) -> Self`

新しい`LuaScript`インスタンスを作成します。

```rust
use adbx_runtime::lua::component::LuaScript;
use std::path::PathBuf;

let script = LuaScript::new(
    PathBuf::from("assets/scripts/player.lua"),
    "print('Hello, World!')".to_string(),
);
```

#### `from_path(script_path: PathBuf) -> Self`

ファイルパスから`LuaScript`インスタンスを作成します。

```rust
let script = LuaScript::from_path(PathBuf::from("assets/scripts/player.lua"));
```

## LuaScriptState

Luaスクリプトの実行状態を表すenumです。

### バリアント

- **`Loaded`**: スクリプトが読み込まれ、実行準備ができている
- **`Running`**: スクリプトが実行中
- **`Error`**: スクリプトでエラーが発生

## ScriptCache

スクリプトのキャッシュを管理するリソースです。コンパイル済みスクリプトをキャッシュし、実行頻度の制限と不要な実行のスキップを実装します。

### フィールド

- **`compiled_scripts: HashMap<Entity, Arc<Mutex<Option<ScriptExecutionState>>>>`**: コンパイル済みスクリプトのキャッシュ
- **`script_hashes: HashMap<Entity, u64>`**: スクリプトのハッシュ値
- **`execution_configs: HashMap<Entity, ScriptExecutionConfig>`**: エンティティごとの実行設定
- **`execution_timestamps: HashMap<Entity, f64>`**: エンティティごとの最後の実行時刻
- **`skip_execution: HashMap<Entity, bool>`**: 実行をスキップするかどうか（update関数がない場合など）

### メソッド

#### `get_or_compile(entity: Entity, script_content: &str, lua: &Lua) -> mlua::Result<Option<ScriptExecutionState>>`

スクリプトをキャッシュから取得するか、コンパイルします。

```rust
let result = script_cache.get_or_compile(entity, &script_content, &lua)?;
```

#### `clear_entity(entity: Entity)`

エンティティに関連するキャッシュをクリアします。

```rust
script_cache.clear_entity(entity);
```

#### `set_execution_rate(entity: Entity, max_executions_per_second: f64)`

エンティティの実行頻度を設定します。

```rust
script_cache.set_execution_rate(entity, 60.0); // 60FPS
```

## スクリプト実行システム

### `load_lua_scripts`

新しく追加された`LuaScript`コンポーネントを読み込んで実行するシステムです。

```rust
use adbx_runtime::lua::vm::load_lua_scripts;

app.add_systems(Update, load_lua_scripts);
```

### `execute_lua_scripts`

`LuaScript`コンポーネントを持つエンティティのスクリプトを実行するシステムです。最適化機能が含まれています：

- スクリプトのコンパイル結果をキャッシュ
- 実行頻度の制限（デフォルト60FPS）
- 不要なスクリプト実行のスキップ（update関数がない場合）

```rust
use adbx_runtime::lua::vm::execute_lua_scripts;

app.add_systems(Update, execute_lua_scripts);
```

## Bevy APIバインディング

LuaスクリプトからBevy ECSの機能にアクセスするためのAPIバインディングが提供されます。

### Entity操作

```lua
-- エンティティを生成
local entity_id = Entity.spawn()

-- エンティティを削除
Entity.despawn(entity_id)
```

### Transform操作

```lua
-- Transformの位置を設定
Transform.set_translation(entity_id, 10.0, 20.0, 30.0)

-- Transformの位置を移動
Transform.translate(entity_id, 1.0, 0.0, 0.0)

-- Transformのスケールを設定
Transform.set_scale(entity_id, 2.0, 2.0, 2.0)
```

### Vec3操作

```lua
-- Vec3を作成
local vec = Vec3.new(1.0, 2.0, 3.0)

-- ゼロベクトルを作成
local zero = Vec3.zero()
```

### Resource操作

```lua
-- Resourceを取得
Resource.get("Time")

-- Resourceの値を取得
local time = Resource.get_value("Time")

-- Resourceを設定
Resource.set("MyResource", { value = 42 })
```

### Event操作

```lua
-- イベントを送信
Event.send("MyEvent", { data = "value" })

-- イベントを読み取り
local events = Event.read("MyEvent")
for i, event in ipairs(events) do
    print(event.data)
end
```

### Component操作

```lua
-- Componentを取得
Component.get(entity_id, "Transform")

-- Componentの値を取得
local transform = Component.get_value(entity_id, "Transform")

-- Componentを設定
Component.set(entity_id, "Transform", { translation = { 0, 0, 0 } })
```

### Log操作

```lua
-- ログを出力
Log.info("情報メッセージ")
Log.warn("警告メッセージ")
Log.error("エラーメッセージ")
```

## スクリプトの書き方

### 基本的なスクリプト

```lua
-- グローバル変数の初期化
local speed = 5.0

-- update関数が定義されている場合、毎フレーム実行される
function update()
    -- エンティティIDを取得
    local id = entity_id
    
    -- Transformを移動
    Transform.translate(id, speed * delta_time, 0.0, 0.0)
end
```

### グローバル変数

スクリプト実行時に以下のグローバル変数が利用可能です：

- **`entity_id`**: スクリプトがアタッチされているエンティティのID
- **`delta_time`**: 前フレームからの経過時間（秒）

### update関数

`update`関数が定義されている場合、毎フレーム実行されます。実行頻度は`ScriptCache::set_execution_rate`で制限できます。

## 実行最適化

### スクリプトキャッシュ

スクリプトは初回実行時にコンパイルされ、ハッシュ値に基づいてキャッシュされます。スクリプトの内容が変更されていない場合、再コンパイルは行われません。

### 実行頻度制限

デフォルトでは、スクリプトは60FPSで実行されます。`ScriptCache::set_execution_rate`を使用して、エンティティごとに実行頻度を設定できます。

```rust
script_cache.set_execution_rate(entity, 30.0); // 30FPS
```

### 不要な実行のスキップ

`update`関数が定義されていないスクリプトは、自動的に実行をスキップします。これにより、不要なオーバーヘッドを削減できます。

## エラーハンドリング

スクリプトの実行中にエラーが発生した場合、`LuaScriptState::Error`が設定されます。エラーメッセージはログに出力されます。

## 関連型

- [`LuaScript`](component.rs): Luaスクリプトコンポーネント
- [`LuaScriptState`](component.rs): スクリプトの実行状態
- [`ScriptCache`](script_cache.rs): スクリプトキャッシュ
- [`register_bevy_bindings`](bindings.rs): Bevy APIバインディングの登録
