use bevy::prelude::*;
use crate::ui::rename_dialog::hide_rename_dialog;

/// リネームダイアログの入力処理
pub fn handle_rename_dialog_input(
    mut rename_dialog: ResMut<super::super::RenameDialogRequest>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // キーボードショートカット
    if keyboard_input.just_pressed(KeyCode::Escape) {
        hide_rename_dialog(&mut rename_dialog);
    }
}
