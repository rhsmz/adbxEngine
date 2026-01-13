use bevy::prelude::*;
use super::button_placement::{build_file_operation_buttons, build_edit_operation_buttons, build_run_operation_buttons, build_view_operation_buttons};

/// ツールバーの構築
pub fn build_toolbar(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(35.0),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(3.0)),
            align_items: AlignItems::Center,
            border: UiRect::bottom(Val::Px(1.0)),
            ..default()
        },
        Name::new("Toolbar"),
        BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
        bevy::ui::BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
    )).with_children(|toolbar| {
        // ファイル操作グループ
        build_file_operation_buttons(toolbar);
        
        // 編集操作グループ
        build_edit_operation_buttons(toolbar);
        
        // 実行操作グループ
        build_run_operation_buttons(toolbar);
        
        // ビュー操作グループ
        build_view_operation_buttons(toolbar);
    });
}
