# カスタムコンポーネントエディタの追加

カスタムコンポーネントエディタを追加する方法を説明します。

## 実装手順

1. `ComponentEditor`トレイトを実装
2. `ComponentEditorRegistry`にエディタを登録

## 実装例

```rust
use adbx_editor::ui::inspector::{ComponentEditor, ComponentEditorRegistry};

struct MyComponentEditor;

impl ComponentEditor for MyComponentEditor {
    fn draw(
        &self,
        commands: &mut Commands,
        entity: Entity,
        component: &dyn std::any::Any,
    ) {
        // エディタUIを構築
    }
}

// 登録
registry.register_editor("MyComponent", MyComponentEditor);
```

## 関連ドキュメント

- [エディタの拡張方法](extending-editor.md)
- [エディタUI API](../../api/editor/ui.md)
