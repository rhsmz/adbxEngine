# システム概要

Adbx Engine Editorのシステムアーキテクチャの概要です。

## アーキテクチャ図

```kroki-plantuml
@startuml
!define RECTANGLE class

package "adbx_editor" {
    RECTANGLE EditorApp {
        + UI Components
        + Systems
        + Communication
    }
}

package "adbx_runtime" {
    RECTANGLE RuntimeApp {
        + Lua VM
        + Hot Reload
        + Communication
    }
}

package "adbx_shared" {
    RECTANGLE Protocol {
        + EditorMessage
        + RuntimeMessage
    }
    RECTANGLE Scene {
        + SceneData
        + EntityData
    }
    RECTANGLE Asset {
        + AssetType
        + AssetMetadata
    }
    RECTANGLE Error {
        + AdbxError
        + ErrorSeverity
    }
}

EditorApp --> Protocol
EditorApp --> Scene
EditorApp --> Asset
EditorApp --> Error
EditorApp --> RuntimeApp : TCP / mpsc

RuntimeApp --> Protocol
RuntimeApp --> Scene
RuntimeApp --> Asset
RuntimeApp --> Error
RuntimeApp --> EditorApp : TCP / mpsc

@enduml
```

## コンポーネント概要

### adbx_editor

エディタアプリケーション。以下の主要コンポーネントを含みます：

- **UI Components**: ヒエラルキービュー、インスペクターパネル、アセットブラウザー、コードエディタなど
- **Systems**: 選択システム、シーン管理、操作記録、リアルタイム同期など
- **Communication**: ランタイムとの通信（TCP/mpsc）

### adbx_runtime

ゲームランタイム。以下の主要コンポーネントを含みます：

- **Lua VM**: Luaスクリプトの実行環境
- **Hot Reload**: アセットとスクリプトのホットリロード機能
- **Communication**: エディタとの通信（TCP/mpsc）

### adbx_shared

共有ライブラリ。以下の主要コンポーネントを含みます：

- **Protocol**: エディタ-ランタイム通信プロトコル
- **Scene**: シーンデータ定義
- **Asset**: アセット定義
- **Error**: エラーハンドリング

## 通信方式

### 同一プロセスモード

エディタとランタイムが同じプロセスで実行される場合、`mpsc`チャネルを使用してメッセージを送受信します。

### 分離実行モード

エディタとランタイムが別々のプロセスで実行される場合、TCP通信を使用してメッセージを送受信します。

## 関連ドキュメント

- [データフロー図](data-flow.md)
- [コンポーネント図](component-diagram.md)
- [パフォーマンス最適化](performance.md)
