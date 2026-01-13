pub mod export;
pub mod recording_functions;
pub mod resource;
pub mod serialization;

// 公開API
pub use recording_functions::{
    record_panel_moved, record_panel_resized, record_scene_saved, record_transform_changed,
};
pub use resource::{OperationContext, OperationRecorder, OperationType, RecordedOperation};

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use export::{
    export_operations_to_json, export_operations_to_markdown, export_operations_to_text,
};
#[allow(unused_imports)]
pub use recording_functions::{record_code_generated, record_entity_created};
#[allow(unused_imports)]
pub use serialization::{get_ai_context, get_ai_readable_history, get_history_json};
