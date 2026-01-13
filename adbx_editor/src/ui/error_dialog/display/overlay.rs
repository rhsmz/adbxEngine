use bevy::prelude::*;

/// エラーダイアログのオーバーレイを描画
pub fn draw_error_dialog_overlay(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: bevy::ui::PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)), // 半透明の黒
            Name::new("ErrorDialogOverlay"),
        ))
        .id()
}
