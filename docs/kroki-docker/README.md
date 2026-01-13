# Kroki Docker環境

このディレクトリには、ドキュメント作成時にダイアグラムを生成するためのKroki Docker環境が含まれています。

## セットアップ

### 前提条件

- Docker
- Docker Compose

### 起動方法

```bash
docker-compose up -d
```

Krokiサービスは `http://localhost:8000` で利用可能になります。

### 停止方法

```bash
docker-compose down
```

## 使用方法

Krokiは、テキスト記述からダイアグラムを生成するサービスです。

### サポートされている形式

- PlantUML
- BlockDiag
- Diagrams.net (draw.io)

### 例

PlantUMLの例:
```
http://localhost:8000/plantuml/svg/SyfFKj2rKt3CoKnELR1Io4ZDoSa70000
```

詳細は [Kroki公式ドキュメント](https://docs.kroki.io/kroki/) を参照してください。

## トラブルシューティング

### ポートが既に使用されている場合

`docker-compose.yml` の `ports` セクションを変更して、別のポートを使用できます。

例:
```yaml
ports:
  - "8001:8000"  # ホストの8001ポートを使用
```

### ヘルスチェックが失敗する場合

コンテナのログを確認してください:
```bash
docker-compose logs kroki
```
