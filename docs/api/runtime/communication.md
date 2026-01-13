# ランタイム通信 API リファレンス

`adbx_runtime`クレートのランタイム-エディタ通信に関するAPIリファレンスです。

## 概要

ランタイム-エディタ通信は、エディタとランタイム間でメッセージを送受信する機能を提供します。同一プロセスモードと分離実行モードの両方をサポートします。

## RuntimeEditorCommunication

ランタイム-エディタ通信リソースです。

### フィールド

- **`editor_rx: Option<Arc<Mutex<mpsc::Receiver<EditorMessage>>>>`**: エディタからのメッセージ受信チャネル（同一プロセスモード）
- **`runtime_tx: Option<Arc<Mutex<mpsc::Sender<RuntimeMessage>>>>`**: エディタへのメッセージ送信チャネル（同一プロセスモード）
- **`tcp_stream: Option<Arc<Mutex<TcpStream>>>`**: TCP通信ストリーム（分離実行モード）
- **`is_separated: bool`**: 分離実行モードかどうか

### 使用例

```rust
use adbx_runtime::communication::RuntimeEditorCommunication;

let communication = RuntimeEditorCommunication::default();
```

## メッセージ送受信

### `receive_from_editor`

エディタからメッセージを受信します。

```rust
use adbx_runtime::communication::receive_from_editor;

let messages = receive_from_editor(&mut communication);
for message in messages {
    match message {
        EditorMessage::LoadScene { scene_path } => {
            // シーンを読み込む
        }
        _ => {}
    }
}
```

### `send_to_editor`

エディタにメッセージを送信します。

```rust
use adbx_runtime::communication::send_to_editor;
use adbx_shared::RuntimeMessage;

send_to_editor(&communication, RuntimeMessage::SceneLoaded {
    scene: scene_data,
})?;
```

## メッセージハンドリング

### `handle_editor_messages`

エディタからのメッセージを処理するシステムです。

```rust
use adbx_runtime::communication::handle_editor_messages;

app.add_systems(Update, handle_editor_messages);
```

## 通信モード

### 同一プロセスモード

エディタとランタイムが同じプロセスで実行されるモードです。`mpsc`チャネルを使用してメッセージを送受信します。

### 分離実行モード

エディタとランタイムが別々のプロセスで実行されるモードです。TCP通信を使用してメッセージを送受信します。

## 関連型

- [`EditorMessage`](../../shared/protocol.md#editormessage): エディタからランタイムへのメッセージ
- [`RuntimeMessage`](../../shared/protocol.md#runtimemessage): ランタイムからエディタへのメッセージ
