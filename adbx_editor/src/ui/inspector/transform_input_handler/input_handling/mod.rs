pub mod event_handling;
pub mod input_validation;
mod main;
pub mod value_update;

pub use event_handling::{
    handle_button_click, handle_drag_end, handle_drag_event, handle_field_click,
    handle_mouse_wheel_event,
};
pub use input_validation::{normalize_transform_value, validate_transform_value};
pub use main::handle_inspector_transform_input;
pub use value_update::{update_value_with_click, update_value_with_drag, update_value_with_wheel};
