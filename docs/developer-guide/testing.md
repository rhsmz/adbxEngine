# テストの書き方と実行方法

Adbx Engine Editorのテストについて説明します。

## テストの種類

### 単体テスト

各クレートの`tests/`ディレクトリに配置されます。

### 統合テスト

プロジェクトルートの`tests/integration/`ディレクトリに配置されます。

### E2Eテスト

プロジェクトルートの`tests/e2e/`ディレクトリに配置されます。

## テストの実行

```bash
# すべてのテストを実行
cargo test

# 特定のクレートのテストを実行
cargo test -p adbx_shared

# 特定のテストを実行
cargo test test_name
```

## テストの書き方

### 基本的なテスト

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(1 + 1, 2);
    }
}
```

### Bevyシステムのテスト

```rust
use bevy::prelude::*;

#[test]
fn test_system() {
    let mut app = App::new();
    app.add_systems(Update, my_system);
    app.update();
    // アサーション
}
```

## 関連ドキュメント

- [エディタの拡張方法](extending-editor.md)
