use super::RealtimePreview;
use bevy::prelude::*;

/// プレビューウィンドウのクリック処理
pub fn handle_preview_window_click(
    mut preview: ResMut<RealtimePreview>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<(&bevy::ui::Interaction, &Name), Changed<bevy::ui::Interaction>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, name) in interaction_query.iter() {
            if *interaction == bevy::ui::Interaction::Pressed {
                let name_str = name.as_str();

                // 閉じるボタン
                if name_str == "PreviewWindowCloseButton" {
                    preview.is_enabled = false;
                    preview.preview_window_entity = None;
                }
            }
        }
    }
}
