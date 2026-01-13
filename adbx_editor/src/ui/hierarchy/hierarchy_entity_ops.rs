use bevy::prelude::*;
use crate::systems::selection::Selection;

/// Entityの追加・削除処理
pub fn handle_hierarchy_entity_create_and_delete(
    mut commands: Commands,
    mut selection: ResMut<Selection>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    _name_query: Query<&Name>,
) {
    // Ctrl+N: 新しいEntityを追加
    if (keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight))
        && keyboard_input.just_pressed(KeyCode::KeyN) {
        let entity_name = format!("NewEntity_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        let new_entity = commands.spawn((
            Name::new(entity_name),
            Transform::default(),
        )).id();
        
        // 新しく作成したEntityを選択
        selection.selected_entities.clear();
        selection.selected_entities.push(new_entity);
    }
    
    // Delete: 選択されたEntityを削除
    if keyboard_input.just_pressed(KeyCode::Delete) {
        for &entity in selection.selected_entities.iter() {
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
        selection.selected_entities.clear();
    }
}
