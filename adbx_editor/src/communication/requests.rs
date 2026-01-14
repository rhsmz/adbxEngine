use super::message_sending::send_to_runtime;
use super::EditorRuntimeCommunication;
use adbx_shared::scene::SceneData;

/// エディタからランタイムにシーン読み込みを要求
#[allow(dead_code)]
pub fn request_scene_load(
    communication: &EditorRuntimeCommunication,
    scene_path: String,
) -> Result<(), String> {
    send_to_runtime(
        communication,
        adbx_shared::EditorMessage::LoadScene { scene_path },
    )
}

/// エディタからランタイムにシーン保存を要求
#[allow(dead_code)]
pub fn request_scene_save(
    communication: &EditorRuntimeCommunication,
    scene: SceneData,
) -> Result<(), String> {
    send_to_runtime(
        communication,
        adbx_shared::EditorMessage::SaveScene { scene },
    )
}

/// エディタからランタイムにアセット読み込みを要求
#[allow(dead_code)]
pub fn request_asset_load(
    communication: &EditorRuntimeCommunication,
    asset_path: String,
) -> Result<(), String> {
    send_to_runtime(
        communication,
        adbx_shared::EditorMessage::LoadAsset { asset_path },
    )
}
