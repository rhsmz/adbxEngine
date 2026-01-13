pub mod message_receiving;
pub mod message_sending;
pub mod notifications;
pub mod process_management;
pub mod requests;
pub mod resource;

// 公開API
pub use message_receiving::{handle_runtime_messages, receive_from_runtime};
pub use message_sending::send_to_runtime;
pub use notifications::{notify_component_update, notify_entity_selection};
pub use process_management::{start_runtime_process, stop_runtime_process};
pub use resource::EditorRuntimeCommunication;

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use notifications::notify_hot_reload;
#[allow(unused_imports)]
pub use requests::{request_asset_load, request_scene_load, request_scene_save};
