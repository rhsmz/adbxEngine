---
name: Gizmo Query Conflict Fix
overview: Resolve Bevy B0001 runtime panic in `handle_gizmo_interaction` by separating conflicting Transform accesses via ParamSet/Without and verifying behavior.
todos:
  - id: inspect-gizmo-system
    content: handle_gizmo_interactionのTransformアクセスを調査
    status: completed
  - id: separate-transform-access
    content: ParamSet/WithoutでTransformアクセスを分離 or システム分割
    status: completed
  - id: verify-no-b0001
    content: cargo checkと実行テストでB0001解消を確認
    status: completed
---

# Gizmo 衝突エラー解消プラン（B0001）

## 背景

- 起動時に Bevy エラー B0001: `handle_gizmo_interaction` の `Query<Transform>` が他パラメータと競合。
- Bevy 0.17.3 では同一システム内で同じコンポーネントを異なるアクセス（mutable / immutable）で重複取得するとパニック。

## 対応方針

1) シグネチャ調査

- 対象: `adbx_editor/src/systems/gizmo/interaction/main.rs`（`handle_gizmo_interaction`）
- 何が Transform をどう参照しているか（`Query<&Transform>` と `Query<&mut Transform>` など）を確認。

2) アクセス分離

- 競合する Transform Query を `ParamSet` でまとめるか、`Without<T>` でクエリ集合を分離。
- 原則: 同一システムでは Transform へのミュータブルアクセスを1系統に統一。

3) 必要ならシステム分割

- 入力処理系（読み取り）とギズモ適用系（書き込み）を別システムに分割し、`SystemSet` の順序付けで依存を解消。

4) 動作確認

- `cargo check` で型・所有権エラー確認。
- 実行テスト（`cargo run -p adbx_editor` 相当）で B0001 パニックが消えることを確認。

## 実装ステップ

- [ ] `handle_gizmo_interaction` のパラメータと内部クエリを確認。
- [ ] Transform への読み/書きが複数ある場合は ParamSet に統合、または Without で分離。
- [ ] 必要なら読取専用部分を別システムへ分離し順序付け。
- [ ] `cargo check` → 実行テストで再発防止を確認。