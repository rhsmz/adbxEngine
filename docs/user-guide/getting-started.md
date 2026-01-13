# はじめに

Adbx Engine Editorの基本的な使い方を説明します。

## インストール

### 前提条件

- Rust 1.70以上
- Cargo

### ビルド

```bash
cargo build
```

### 実行

```bash
cargo run --bin adbx_editor
```

## 基本操作

### プロジェクトの作成

1. メニューバーから「File」→「New Project」を選択
2. プロジェクト名と保存場所を指定
3. 「Create」をクリック

### プロジェクトの開く

1. メニューバーから「File」→「Open Project」を選択
2. プロジェクトフォルダを選択

### シーンの作成

1. ヒエラルキービューで右クリック
2. 「Create Entity」を選択
3. エンティティ名を入力

### エンティティの編集

1. ヒエラルキービューでエンティティを選択
2. インスペクターパネルでコンポーネントを編集

### シーンの保存

1. メニューバーから「File」→「Save Scene」を選択
2. シーン名を入力
3. 「Save」をクリック

### ゲームのビルド

1. メニューバーから「Build」→「Build Game」を選択
2. ビルド進捗が表示されます
3. ビルド完了後、`target/release/game_package/`にパッケージ化されたゲームが生成されます

ビルドされたゲームは、`target/release/game_package/`ディレクトリに配置され、実行可能ファイルと必要なアセットが含まれます。

## 関連ドキュメント

- [エディタ機能の説明](editor-features.md)
- [Luaスクリプティングガイド](scripting-guide.md)
- [トラブルシューティング](troubleshooting.md)
