pub mod resource;
pub mod process_management;
pub mod message_sending;
pub mod message_receiving;
pub mod notifications;
pub mod requests;

// 公開API
pub use resource::EditorRuntimeCommunication;
pub use process_management::{start_runtime_process, stop_runtime_process};
pub use message_sending::send_to_runtime;
pub use message_receiving::{receive_from_runtime, handle_runtime_messages};
pub use notifications::{notify_entity_selection, notify_component_update, notify_hot_reload};
pub use requests::{request_scene_load, request_scene_save, request_asset_load};
