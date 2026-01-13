use bevy::prelude::*;
use super::super::resource::PanelPosition;
use super::super::drag_handler::DropZoneIndicator;

/// ドロップゾーンの視覚的表示（改善版）
pub fn show_drop_zone(
    commands: &mut Commands,
    position: PanelPosition,
    window_size: Vec2,
) {
    let (width, height, left, top) = match position {
        PanelPosition::Left => (window_size.x * 0.2, window_size.y, 0.0, 0.0),
        PanelPosition::Right => (window_size.x * 0.2, window_size.y, window_size.x * 0.8, 0.0),
        PanelPosition::Top => (window_size.x, window_size.y * 0.2, 0.0, 0.0),
        PanelPosition::Bottom => (window_size.x, window_size.y * 0.2, 0.0, window_size.y * 0.8),
        PanelPosition::Center => (window_size.x * 0.6, window_size.y * 0.6, window_size.x * 0.2, window_size.y * 0.2),
        PanelPosition::Floating => return,
    };
    
    // より目立つドロップゾーン表示
    commands.spawn((
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            position_type: bevy::ui::PositionType::Absolute,
            left: Val::Px(left),
            top: Val::Px(top),
            border: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.6, 0.9, 0.4)),
        bevy::ui::BorderColor::all(Color::srgb(0.4, 0.8, 1.0)),
        DropZoneIndicator { position },
        Name::new("DropZoneIndicator"),
    )).with_children(|zone: &mut ChildSpawnerCommands| {
        // ドロップゾーンのラベルを表示
        let label_text = match position {
            PanelPosition::Left => "← Left",
            PanelPosition::Right => "Right →",
            PanelPosition::Top => "↑ Top",
            PanelPosition::Bottom => "↓ Bottom",
            PanelPosition::Center => "Center",
            PanelPosition::Floating => "",
        };
        
        zone.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new("DropZoneLabel"),
        )).with_children(|label: &mut ChildSpawnerCommands| {
            label.spawn((
                Text::new(label_text),
                bevy::text::TextFont {
                    font_size: 24.0,
                    ..default()
                },
                bevy::text::TextColor(Color::srgb(0.4, 0.8, 1.0)),
            ));
        });
    });
}
