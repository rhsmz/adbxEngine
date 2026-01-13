pub mod processing;
pub mod requests;
pub mod resource;

// 公開API
pub use processing::{
    handle_ai_code_generation_responses, process_pending_ai_code_generation_requests,
};
pub use requests::{
    request_ai_code_completion, request_ai_code_generation, request_ai_code_refactor,
};
pub use resource::{
    AiIntegration, AiProvider, AiRequest, AiRequestStatus, AiRequestType, AiResponse,
};
