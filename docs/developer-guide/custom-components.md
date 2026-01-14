# カスタムコンポーネントエディタの追加

カスタムコンポーネントエディタを追加する方法を説明します。

## 実装手順

1. `ComponentEditor`トレイトを実装
2. `ComponentEditorRegistry`にエディタを登録
3. インスペクターパネルで自動的に使用されます

## 実装例

```rust
use adbx_editor::ui::inspector::component_editor_registry::ComponentEditor;
use adbx_editor::ui::inspector::component_editor_registry::ComponentEditorRegistry;
use bevy::prelude::*;

struct MyComponentEditor;

impl ComponentEditor for MyComponentEditor {
    fn draw_ui(&self, commands: &mut Commands, entity: Entity, parent_ui: Entity) {
        // エディタUIを構築
        commands.entity(parent_ui).with_children(|parent| {
            parent.spawn(Text::new("My Component Editor"));
        });
    }
    
    fn component_name(&self) -> &'static str {
        "MyComponent"
    }
}

// 登録（システム内で実行）
fn register_my_component_editor(
    mut registry: ResMut<ComponentEditorRegistry>,
) {
    use bevy::prelude::*;
    registry.register::<MyComponent>(Box::new(MyComponentEditor));
}
```

## デフォルトのコンポーネントエディタ

以下のコンポーネントエディタがデフォルトで登録されています：

- **TransformEditor**: `Transform`コンポーネント用
- **Camera3dEditor**: `Camera3d`コンポーネント用
- **Camera2dEditor**: `Camera2d`コンポーネント用
- **SpriteEditor**: `Sprite`コンポーネント用
- **Mesh3dEditor**: `Mesh3d`コンポーネント用
- **MeshMaterial3dEditor**: `MeshMaterial3d<StandardMaterial>`コンポーネント用

### デフォルトエディタの登録

デフォルトのコンポーネントエディタは`register_default_component_editors`システムで登録されます：

```rust
/// デフォルトのエディタを登録
pub fn register_default_component_editors(
    mut registry: bevy::prelude::ResMut<
        crate::ui::inspector::component_editor_registry::ComponentEditorRegistry,
    >,
) {
    use crate::ui::inspector::component_editor_registry::ComponentEditorRegistry;
    use bevy::prelude::*;

    // Transformエディタを登録
    registry.register::<Transform>(Box::new(TransformEditor));

    // Cameraエディタを登録
    registry.register::<bevy::camera::Camera3d>(Box::new(Camera3dEditor));
    registry.register::<bevy::camera::Camera2d>(Box::new(Camera2dEditor));

    // Spriteエディタを登録
    registry.register::<Sprite>(Box::new(SpriteEditor));

    // Mesh3dエディタを登録
    registry.register::<bevy::prelude::Mesh3d>(Box::new(Mesh3dEditor));

    // MeshMaterial3dエディタを登録
    registry.register::<bevy::prelude::MeshMaterial3d<bevy::pbr::StandardMaterial>>(Box::new(
        MeshMaterial3dEditor,
    ));

    bevy::log::info!("Component editors registered: Transform, Camera3d, Camera2d, Sprite, Mesh3d, MeshMaterial3d");
}
```

## 注意事項

- `ComponentEditorRegistry`の`iter`メソッドと`component_name`メソッドは現在未使用です
- エディタは`TypeId`をキーとして登録されるため、同じ型に対して複数のエディタを登録することはできません

## 関連ドキュメント

- [エディタの拡張方法](extending-editor.md)
- [エディタUI API](../../api/editor/ui.md)
