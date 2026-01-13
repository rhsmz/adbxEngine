use bevy::prelude::*;
use super::ErrorDialog;
use super::show_functions::close_error_dialog;

/// エラーダイアログの入力処理
pub fn handle_error_dialog_input(
    mut error_dialog: ResMut<ErrorDialog>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<(&bevy::ui::Interaction, &Name), Changed<bevy::ui::Interaction>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !error_dialog.is_visible {
        return;
    }
    
    // ESCキーで閉じる
    if keyboard_input.just_pressed(KeyCode::Escape) {
        close_error_dialog(error_dialog.as_mut());
        return;
    }
    
    // マウスクリックで閉じるボタンまたはOKボタンを処理
    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, name) in interaction_query.iter() {
            if *interaction == bevy::ui::Interaction::Pressed {
                let name_str = name.as_str();
                
                // 閉じるボタンまたはOKボタン
                if name_str == "ErrorDialogCloseButton" || name_str == "ErrorDialogOkButton" {
                    close_error_dialog(error_dialog.as_mut());
                    return;
                }
                
                // オーバーレイをクリックした場合（ダイアログ外）は閉じない
                // （ユーザーが意図的に閉じる必要があるため）
            }
        }
    }
}
