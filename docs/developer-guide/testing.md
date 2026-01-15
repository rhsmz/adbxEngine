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

### 基本的なテスト実行

```bash
# Taskfileを使用（推奨）
task test

# または直接Cargoを使用
# すべてのテストを実行
cargo test

# 特定のクレートのテストを実行
cargo test -p adbx_shared

# 特定のテストを実行
cargo test test_name
```

### 品質チェックとCI

```bash
# コードフォーマットチェック
task fmt-check

# Clippyによるリントチェック
task clippy-all

# CI相当のチェック（フォーマット + Clippy + テスト）
task ci

# 開発環境全体のチェック
task dev
```

### E2Eスモークテスト

主要機能の正常動作を確認するための簡易E2Eテスト手順：

1. **ビルド機能のテスト**:
   - エディタを起動
   - メニューから「Build」→「Build Game」を選択
   - ビルド進捗が表示され、完了することを確認
   - `target/release/game_package/`にパッケージが生成されることを確認

2. **Transform編集機能のテスト**:
   - エンティティを作成
   - インスペクターパネルでTransformコンポーネントを選択
   - 位置/回転/スケールの値を編集
   - 値が正しく反映されることを確認

3. **リネーム機能のテスト**:
   - ファイルやディレクトリを作成
   - リネームダイアログを開く
   - 新しい名前を入力して実行
   - 名前が正しく変更されることを確認

4. **ログパネルのテスト**:
   - エディタ操作中にログパネルを確認
   - ログが正しく表示されることを確認

5. **通信機能のテスト**:
   - エディタを起動
   - TCP通信未接続の警告が表示されないことを確認（自動再接続が機能している）
   - エンティティ選択時に通信エラーが発生しないことを確認

6. **SceneViewのテスト**:
   - エディタを起動
   - SceneViewが表示され、Camera order曖昧性の警告が出ないことを確認
   - 3Dカメラが正しく初期化されていることを確認

### 既知の問題の検証

known-issues.mdに記載された問題が解消されていることを確認：

- **BuildGameMenuRequest初期化**: ビルドメニュー操作時にパニックが発生しない
- **アセットディレクトリ**: `adbx_editor/assets/`および`adbx_runtime/assets/`が存在し、警告が表示されない
- **TCP通信**: 分離モード時の接続状態管理が正しく機能し、過度な警告が出ない
- **Camera order曖昧性**: SceneViewのカメラorderが適切に設定され、重複警告が出ない
- **警告数**: `cargo clippy --workspace -- -D warnings`がゼロ警告で完了する

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
