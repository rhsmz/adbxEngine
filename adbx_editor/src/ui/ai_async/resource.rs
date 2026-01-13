#[cfg(feature = "ai")]
use bevy::prelude::*;
#[cfg(feature = "ai")]
use std::sync::mpsc;

/// AI非同期処理のリソース
#[cfg(feature = "ai")]
#[derive(Resource)]
pub struct AiAsyncProcessor {
    // mpsc::ReceiverはSyncではないため、Mutexで保護する必要がある
    pub response_receiver: Option<std::sync::Mutex<mpsc::Receiver<(usize, crate::ui::code_editor::AiResponse)>>>,
    pub response_sender: Option<mpsc::Sender<(usize, crate::ui::code_editor::AiResponse)>>,
}

#[cfg(feature = "ai")]
impl Default for AiAsyncProcessor {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            response_receiver: Some(std::sync::Mutex::new(receiver)),
            response_sender: Some(sender),
        }
    }
}
