use super::super::ErrorDialog;
use super::buttons::draw_error_dialog_buttons;
use super::message_area::draw_error_dialog_message_area;
use super::overlay::draw_error_dialog_overlay;
use super::theme::get_error_theme;
use super::title_bar::draw_error_dialog_title_bar;
use bevy::prelude::*;

/// エラーダイアログのUI描画
pub fn draw_error_dialog(
    mut commands: Commands,
    mut error_dialog: ResMut<ErrorDialog>,
    windows: Query<&Window>,
) {
    // ダイアログが表示されている場合のみ描画
    if !error_dialog.is_visible {
        // 既存のダイアログを削除
        if let Some(dialog_entity) = error_dialog.dialog_entity {
            if let Ok(mut entity_commands) = commands.get_entity(dialog_entity) {
                entity_commands.despawn();
            }
            error_dialog.dialog_entity = None;
        }
        return;
    }

    // 既にダイアログが表示されている場合は再描画しない
    if error_dialog.dialog_entity.is_some() {
        return;
    }

    // ウィンドウサイズを取得（使用しないが、将来の拡張のために保持）
    let _window_size = if let Ok(window) = windows.single() {
        Vec2::new(window.width(), window.height())
    } else {
        Vec2::new(1920.0, 1080.0) // デフォルト値
    };

    // エラータイプに応じた色を決定
    let (bg_color, title_color, icon) = get_error_theme(error_dialog.error_type);

    // ダイアログの背景（オーバーレイ）
    let overlay_entity = draw_error_dialog_overlay(&mut commands);

    // ダイアログ本体
    let dialog_entity = commands
        .spawn((
            Node {
                width: Val::Px(500.0),
                min_height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(bg_color),
            Name::new("ErrorDialog"),
        ))
        .with_children(|dialog: &mut ChildSpawnerCommands| {
            // タイトルバー
            draw_error_dialog_title_bar(dialog, &error_dialog, title_color, icon);

            // メッセージ
            draw_error_dialog_message_area(dialog, &error_dialog);

            // OKボタン
            draw_error_dialog_buttons(dialog);
        })
        .id();

    commands.entity(overlay_entity).add_child(dialog_entity);
    error_dialog.dialog_entity = Some(overlay_entity);
}
