# アーキテクチャ概要

Adbx Engine Editorのアーキテクチャと主要コンポーネントの関係を説明します。

## アーキテクチャの概要

Adbx Engine Editorは、以下の主要コンポーネントから構成されています：

- **adbx_editor**: エディタアプリケーション本体（ECSベース）
- **adbx_runtime**: ゲームランタイム（Lua統合、ホットリロード）
- **adbx_shared**: 共有ライブラリ（通信プロトコル、シーン/アセット定義）

エディタはBevy ECSを基盤としており、リアルタイム同期とホットリロード機能を備えています。

## 主要コンポーネント

### ECSシステム構造

- **UI Components**: インスペクタ、階層ビュー、アセットブラウザ、コードエディタなどのUI要素
- **Core Systems**: 選択、シーン管理、ギズモ操作、メニューステムなどのコア機能
- **Communication**: エディタ-ランタイム間のリアルタイム同期
- **Hot Reload**: アセットとスクリプトのホットリロード機能

### リソースと依存関係

主要なECSリソース：
- `EditorApp`: エディタのメイン状態
- `InspectorPanel`: コンポーネント編集パネル
- `SceneManager`: シーン管理
- `CustomAssetRegistry`: カスタムアセットローダー管理
- `BuildGameRequest`: ゲームビルド要求

## 図一覧

### [システム概要](system-overview.md)
- 全体構成図
- ECSシステムの詳細フロー
- リソースとシステムの依存関係

### [データフロー図](data-flow.md)
- エディタ-ランタイム通信フロー
- シーン読み込み/保存フロー
- ホットリロードフロー（標準/カスタムアセット）
- アプリケーション初期化フロー

### [コンポーネント図](component-diagram.md)
- UIコンポーネント階層
- システム依存関係
- UIパネルの相互作用フロー

### [パフォーマンス最適化](performance.md)
- パフォーマンス監視システム
- メモリ使用量の最適化

## Kroki図の生成手順

ドキュメント内のダイアグラムは[Kroki](https://kroki.io/)を使用して生成されています。

### Dockerを使用したローカル生成

1. **Krokiサービスの起動**:
   ```bash
   cd docs/kroki-docker
   docker-compose up -d
   ```

2. **ダイアグラムのプレビュー**:
   - PlantUML: `http://localhost:8000/plantuml/svg/{encoded-text}`
   - BlockDiag: `http://localhost:8000/blockdiag/svg/{encoded-text}`
   - Diagrams.net: `http://localhost:8000/diagramsnet/svg/{encoded-text}`

3. **サービスの停止**:
   ```bash
   docker-compose down
   ```

### テキストからのエンコード

Krokiでは、ダイアグラムテキストをBase64エンコードしてURLに含めます：

```bash
# PlantUMLの場合
echo "@startuml\nclass Example\n@enduml" | base64 -w 0
```

### 注意事項

- ダイアグラムは`kroki-plantuml`, `kroki-blockdiag`などの形式でマークダウンに記述
- ローカル環境ではDockerコンテナを使用
- CI/CD環境ではKrokiサービスを別途用意

## 関連ドキュメント

- [APIドキュメント](../api/)
- [開発者ガイド](../developer-guide/)
- [ユーザーガイド](../user-guide/)