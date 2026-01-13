# エラーハンドリング API リファレンス

`adbx_shared`クレートのエラーハンドリングに関するAPIリファレンスです。

## 概要

`error.rs`は、統一されたエラータイプ、エラーコンテキスト、エラーの優先度付けを定義します。

## ErrorSeverity

エラーの優先度を表すenumです。

### バリアント

- **`Low`**: 低優先度（警告レベル）
- **`Medium`**: 中優先度（通常のエラー）
- **`High`**: 高優先度（重大なエラー）
- **`Critical`**: 致命的（アプリケーションを停止させる可能性がある）

### 使用例

```rust
use adbx_shared::error::ErrorSeverity;

let severity = ErrorSeverity::High;
match severity {
    ErrorSeverity::Low => println!("警告"),
    ErrorSeverity::Medium => println!("通常のエラー"),
    ErrorSeverity::High => println!("重大なエラー"),
    ErrorSeverity::Critical => println!("致命的なエラー"),
}
```

## ErrorContext

エラーのコンテキスト情報を表す構造体です。

### フィールド

- **`file_path: Option<PathBuf>`**: エラーが発生したファイルのパス
- **`line_number: Option<usize>`**: エラーが発生した行番号
- **`column_number: Option<usize>`**: エラーが発生した列番号
- **`function_name: Option<String>`**: エラーが発生した関数名
- **`stack_trace: Option<String>`**: スタックトレース
- **`additional_info: Option<String>`**: 追加情報

### 使用例

```rust
use adbx_shared::error::ErrorContext;
use std::path::PathBuf;

let context = ErrorContext {
    file_path: Some(PathBuf::from("src/main.rs")),
    line_number: Some(42),
    column_number: Some(10),
    function_name: Some("main".to_string()),
    stack_trace: Some("...".to_string()),
    additional_info: Some("詳細な情報".to_string()),
};
```

## AdbxError

共有エラー型を表すenumです。

### バリアント

#### `Io { source: std::io::Error, context: ErrorContext, severity: ErrorSeverity }`
IOエラーを表します。

#### `Serialization { source: serde_json::Error, context: ErrorContext, severity: ErrorSeverity }`
シリアライゼーションエラーを表します。

#### `Asset { message: String, context: ErrorContext, severity: ErrorSeverity }`
アセットエラーを表します。

#### `Scene { message: String, context: ErrorContext, severity: ErrorSeverity }`
シーンエラーを表します。

#### `Lua { message: String, context: ErrorContext, severity: ErrorSeverity }`
Luaスクリプトエラーを表します。

#### `Communication { message: String, context: ErrorContext, severity: ErrorSeverity }`
通信エラーを表します。

#### `Validation { message: String, context: ErrorContext, severity: ErrorSeverity }`
検証エラーを表します。

### 使用例

```rust
use adbx_shared::error::{AdbxError, ErrorContext, ErrorSeverity};

let error = AdbxError::Asset {
    message: "アセットが見つかりません".to_string(),
    context: ErrorContext::default(),
    severity: ErrorSeverity::Medium,
};
```

## メソッド

### `severity(&self) -> ErrorSeverity`

エラーの優先度を取得します。

```rust
let severity = error.severity();
```

### `context(&self) -> &ErrorContext`

エラーコンテキストを取得します。

```rust
let context = error.context();
```

### `user_friendly_message(&self) -> String`

ユーザーフレンドリーなエラーメッセージを生成します。日本語で表示されます。

```rust
let message = error.user_friendly_message();
println!("{}", message);
```

出力例:
```
アセットエラー: アセットが見つかりません
ファイル: assets/textures/player.png
行: 42, 列: 10
関数: load_asset
詳細: ファイルが存在しません
解決方法: アセットファイルが存在し、正しい形式か確認してください。
```

## エラーの変換

既存のエラー型から`AdbxError`への変換が実装されています。

### `std::io::Error`からの変換

```rust
use std::io;
use adbx_shared::error::AdbxError;

let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
let adbx_error: AdbxError = io_error.into();
```

### `serde_json::Error`からの変換

```rust
use serde_json;
use adbx_shared::error::AdbxError;

let json_error = serde_json::from_str::<String>("invalid json").unwrap_err();
let adbx_error: AdbxError = json_error.into();
```

## Result型

`adbx_shared::error::Result<T>`は`std::result::Result<T, AdbxError>`のエイリアスです。

```rust
use adbx_shared::error::Result;

fn load_asset(path: &str) -> Result<AssetMetadata> {
    // ...
    Ok(metadata)
}
```

## エラーハンドリングのベストプラクティス

1. **適切な優先度を設定**: エラーの重要度に応じて`ErrorSeverity`を適切に設定します。
2. **コンテキスト情報を追加**: デバッグを容易にするため、可能な限り詳細なコンテキスト情報を追加します。
3. **ユーザーフレンドリーなメッセージ**: `user_friendly_message()`を使用して、ユーザーに分かりやすいメッセージを表示します。
4. **エラーの伝播**: `?`演算子を使用してエラーを適切に伝播します。

### 例

```rust
use adbx_shared::error::{AdbxError, ErrorContext, ErrorSeverity, Result};
use std::path::PathBuf;

fn load_scene(path: &str) -> Result<SceneData> {
    let file_path = PathBuf::from(path);
    
    // ファイルの存在確認
    if !file_path.exists() {
        return Err(AdbxError::Scene {
            message: format!("シーンファイルが見つかりません: {}", path),
            context: ErrorContext {
                file_path: Some(file_path),
                ..Default::default()
            },
            severity: ErrorSeverity::High,
        });
    }
    
    // ファイルの読み込み
    let content = std::fs::read_to_string(&file_path)?;
    
    // デシリアライゼーション
    let scene: SceneData = serde_json::from_str(&content)
        .map_err(|e| AdbxError::Serialization {
            source: e,
            context: ErrorContext {
                file_path: Some(file_path),
                ..Default::default()
            },
            severity: ErrorSeverity::Medium,
        })?;
    
    Ok(scene)
}
```

## 関連型

- [`EditorError`](../../editor/error.md): エディタ固有のエラー型
- [`RuntimeError`](../../runtime/error.md): ランタイム固有のエラー型
