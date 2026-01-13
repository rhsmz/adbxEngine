# Luaスクリプティングガイド

Adbx Engine EditorでのLuaスクリプティングについて説明します。

## 基本的なスクリプト

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

## グローバル変数

- **`entity_id`**: スクリプトがアタッチされているエンティティのID
- **`delta_time`**: 前フレームからの経過時間（秒）

## Bevy APIバインディング

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
```

### Resource操作

```lua
-- Resourceを取得
local time = Resource.get_value("Time")
```

### Event操作

```lua
-- イベントを送信
Event.send("MyEvent", { data = "value" })

-- イベントを読み取り
local events = Event.read("MyEvent")
```

## 関連ドキュメント

- [エディタ機能の説明](editor-features.md)
- [ランタイムLua API](../../api/runtime/lua.md)
