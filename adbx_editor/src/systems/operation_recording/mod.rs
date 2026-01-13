pub mod resource;
pub mod recording_functions;
pub mod serialization;
pub mod export;

// 公開API
pub use resource::{OperationRecorder, RecordedOperation, OperationType, OperationContext};
pub use recording_functions::{
    record_entity_created,
    record_transform_changed,
    record_scene_saved,
    record_code_generated,
    record_panel_moved,
    record_panel_resized,
};
pub use serialization::{get_ai_readable_history, get_ai_context, get_history_json};
pub use export::{export_operations_to_json, export_operations_to_text, export_operations_to_markdown};
