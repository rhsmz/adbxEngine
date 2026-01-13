pub mod input_validation;
pub mod value_update;
pub mod event_handling;
mod main;

pub use main::handle_inspector_transform_input;
pub use input_validation::{validate_transform_value, normalize_transform_value};
pub use value_update::{update_value_with_wheel, update_value_with_drag, update_value_with_click};
pub use event_handling::{
    handle_mouse_wheel_event,
    handle_drag_event,
    handle_field_click,
    handle_drag_end,
    handle_button_click,
};
