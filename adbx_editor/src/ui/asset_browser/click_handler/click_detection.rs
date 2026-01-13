use bevy::prelude::*;
use bevy::ui::Interaction;

/// クリックイベントの検出
pub fn detect_click_event(
    mouse_input: &Res<ButtonInput<MouseButton>>,
    interaction_query: &Query<(&Interaction, &Name), Changed<Interaction>>,
) -> Vec<(String, Interaction)> {
    let mut clicked_items = Vec::new();

    if mouse_input.just_pressed(MouseButton::Left) {
        for (interaction, name) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                clicked_items.push((name.as_str().to_string(), *interaction));
            }
        }
    }

    clicked_items
}
