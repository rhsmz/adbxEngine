use super::super::RenameDialogRequest;
use super::dialog_display::draw_rename_dialog;
use super::input_handling::handle_rename_dialog_input;
use bevy::prelude::*;

/// リネームダイアログのUI描画（メイン関数）
pub fn draw_rename_dialog_main(
    mut commands: Commands,
    mut rename_dialog: ResMut<RenameDialogRequest>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    _asset_browser_query: Query<Entity, (With<Name>, With<Node>)>,
) {
    draw_rename_dialog(commands, &rename_dialog);
    handle_rename_dialog_input(rename_dialog, keyboard_input);
}
