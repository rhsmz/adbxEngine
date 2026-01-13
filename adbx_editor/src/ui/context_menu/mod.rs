pub mod asset_operations;
pub mod click_handler;
pub mod clipboard_operations;
pub mod entity_operations;
pub mod resources;
pub mod right_click;
pub mod script_execution;

// マクロをモジュールレベルで公開
#[macro_export]
macro_rules! spawn_menu_item_inline {
    ($parent:expr, $label:expr, $name:expr) => {
        $parent
            .spawn((
                bevy::prelude::Node {
                    width: bevy::ui::Val::Percent(100.0),
                    height: bevy::ui::Val::Px(25.0),
                    padding: bevy::ui::UiRect::horizontal(bevy::ui::Val::Px(10.0)),
                    justify_content: bevy::ui::JustifyContent::FlexStart,
                    align_items: bevy::ui::AlignItems::Center,
                    ..Default::default()
                },
                bevy::prelude::BackgroundColor(bevy::prelude::Color::srgb(0.25, 0.25, 0.25)),
                bevy::ui::Interaction::default(),
                bevy::prelude::Name::new($name),
            ))
            .with_children(|item| {
                item.spawn((
                    bevy::prelude::Text::new($label),
                    bevy::text::TextFont {
                        font_size: 12.0,
                        ..Default::default()
                    },
                    bevy::text::TextColor(bevy::prelude::Color::WHITE),
                ));
            });
    };
}

#[macro_export]
macro_rules! spawn_menu_separator_inline {
    ($parent:expr) => {
        $parent.spawn((
            bevy::prelude::Node {
                width: bevy::ui::Val::Percent(100.0),
                height: bevy::ui::Val::Px(1.0),
                margin: bevy::ui::UiRect::vertical(bevy::ui::Val::Px(2.0)),
                ..Default::default()
            },
            bevy::prelude::BackgroundColor(bevy::prelude::Color::srgb(0.4, 0.4, 0.4)),
            bevy::prelude::Name::new("ContextMenuSeparator"),
        ));
    };
}

// 公開API
pub use asset_operations::handle_asset_operations;
pub use click_handler::handle_context_menu_item_click;
pub use clipboard_operations::{
    handle_entity_clipboard_operations, handle_search_replace_dialog,
    handle_text_editor_clipboard_operations,
};
pub use clipboard_operations::{SearchReplaceDialogType, TextEditorClipboardOperation};
pub use entity_operations::handle_entity_operations;
pub use resources::{
    AssetOperation, ContextMenu, ContextType, EntityClipboardOperation, EntityOperation,
};
pub use right_click::detect_right_click_for_context_menu;
pub use script_execution::handle_script_execution;

// display functions
pub fn show_context_menu(
    commands: bevy::prelude::Commands,
    context_menu: bevy::prelude::ResMut<ContextMenu>,
    windows: bevy::prelude::Query<&bevy::window::Window>,
    context_type: ContextType,
) {
    resources::show_context_menu(commands, context_menu, windows, context_type);
}

pub fn hide_context_menu(
    commands: bevy::prelude::Commands,
    context_menu: bevy::prelude::ResMut<ContextMenu>,
) {
    resources::hide_context_menu(commands, context_menu);
}
