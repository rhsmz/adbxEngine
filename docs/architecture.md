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

#### 1. Krokiサービスの起動

```bash
cd docs/kroki-docker
docker-compose up -d
```

サービスが起動すると `http://localhost:8000` でアクセス可能になります。

#### 2. サービス状態の確認

```bash
curl -s http://localhost:8000/health | jq
```

#### 3. ダイアグラムの直接プレビュー

ブラウザで以下のURLにアクセスしてダイアグラムを直接確認できます：

- **PlantUML**: `http://localhost:8000/plantuml/svg/{encoded-text}`
- **BlockDiag**: `http://localhost:8000/blockdiag/svg/{encoded-text}`
- **Diagrams.net**: `http://localhost:8000/diagramsnet/svg/{encoded-text}`

### テキストからのエンコード方法

Krokiでは、ダイアグラムテキストをBase64エンコードしてURLに含めます。

#### Linux/macOSの場合

```bash
# PlantUMLの場合
echo "@startuml
class Example {
    + method()
}
@enduml" | base64 -w 0

# または
plantuml_text="@startuml
class Example {
    + method()
}
@enduml"
echo "$plantuml_text" | base64 -w 0
```

#### Windows (PowerShell)の場合

```powershell
# PlantUMLの場合
$plantuml = "@startuml
class Example {
    + method()
}
@enduml"
$encoded = [Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($plantuml))
Write-Host $encoded
```

#### 実際のURL例

エンコードされたテキストをURLに含めてアクセス：

```
http://localhost:8000/plantuml/svg/UGBzYXJ1bWwKIGNsYXNzIEV4YW1wbGUgewogICAgICBtZXRob2QoKQogIH0KIEBlbmR1bWw
```

### Markdownファイルでの確認方法

#### GitHub/GitLabでの自動表示

ドキュメント内の `kroki-plantuml` 形式のコードブロックは、GitHubやGitLabなどのプラットフォームで自動的にダイアグラムに変換されます：

```markdown
```kroki-plantuml
@startuml
class Example {
    + method()
}
@enduml
```
```

#### VS Codeでのローカル確認

1. **Markdown Preview Enhanced**拡張機能をインストール
2. 設定でKrokiサポートを有効化：
   ```json
   {
     "markdown-preview-enhanced.enableKroki": true,
     "markdown-preview-enhanced.krokiServer": "http://localhost:8000"
   }
   ```

#### Cursor IDEでの確認

CursorのMarkdownプレビュー機能でKrokiダイアグラムが自動的に表示されます。

#### ブラウザでの確認

ローカル環境でダイアグラムをブラウザで確認するには：

1. **HTMLファイルを開く**:
   ```bash
   # ブラウザで開く
   start docs/architecture/diagrams.html
   ```

2. **Krokiサービスが起動していることを確認**

3. **「すべてのダイアグラムを読み込み」ボタンをクリック**

`diagrams.html`ファイルには以下の機能があります：
- 全アーキテクチャダイアグラムの統合表示
- Krokiサービスの状態確認
- ダイアグラムのリアルタイム読み込み
- エラーハンドリングとステータス表示

### トラブルシューティング

#### ダイアグラムが表示されない場合

1. **Krokiサービスが起動しているか確認**:
   ```bash
   curl http://localhost:8000/health
   ```

2. **ダイアグラム構文の確認**:
   - PlantUML: `@startuml` と `@enduml` で囲まれているか
   - 構文エラーがないか

3. **エンコードの確認**:
   ```bash
   # エンコード結果が正しいかテスト
   echo "SGVsbG8gV29ybGQ=" | base64 -d  # "Hello World" が出力されるはず
   ```

#### よくあるエラー

- **Invalid XML character**: PlantUML構文に特殊文字が含まれている
- **Syntax error**: ダイアグラム構文が不正
- **Connection refused**: Krokiサービスが起動していない

### 注意事項

- ダイアグラムは`kroki-plantuml`, `kroki-blockdiag`などの形式でマークダウンに記述
- ローカル環境ではDockerコンテナを使用
- CI/CD環境ではKrokiサービスを別途用意
- 大規模なダイアグラムはブラウザで直接確認することを推奨

## 関連ドキュメント

- [APIドキュメント](../api/)
- [開発者ガイド](../developer-guide/)
- [ユーザーガイド](../user-guide/)