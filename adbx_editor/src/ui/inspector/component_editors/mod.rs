pub mod camera_2d_editor_impl;
pub mod camera_3d_editor_impl;
pub mod mesh_3d_editor_impl;
pub mod mesh_material_3d_editor_impl;
pub mod sprite_editor_impl;
pub mod transform_editor_impl;

pub use camera_2d_editor_impl::Camera2dEditor;
pub use camera_3d_editor_impl::Camera3dEditor;
pub use mesh_3d_editor_impl::Mesh3dEditor;
pub use mesh_material_3d_editor_impl::MeshMaterial3dEditor;
pub use sprite_editor_impl::SpriteEditor;
pub use transform_editor_impl::TransformEditor;

/// デフォルトのエディタを登録
pub fn register_default_component_editors(
    mut registry: bevy::prelude::ResMut<
        crate::ui::inspector::component_editor_registry::ComponentEditorRegistry,
    >,
) {
    use crate::ui::inspector::component_editor_registry::ComponentEditorRegistry;
    use bevy::prelude::*;

    // Transformエディタを登録
    registry.register::<Transform>(Box::new(TransformEditor));

    // Cameraエディタを登録
    registry.register::<bevy::camera::Camera3d>(Box::new(Camera3dEditor));
    registry.register::<bevy::camera::Camera2d>(Box::new(Camera2dEditor));

    // Spriteエディタを登録
    registry.register::<Sprite>(Box::new(SpriteEditor));

    // Mesh3dエディタを登録
    registry.register::<bevy::prelude::Mesh3d>(Box::new(Mesh3dEditor));

    // MeshMaterial3dエディタを登録
    registry.register::<bevy::prelude::MeshMaterial3d<bevy::pbr::StandardMaterial>>(Box::new(
        MeshMaterial3dEditor,
    ));

    bevy::log::info!("Component editors registered: Transform, Camera3d, Camera2d, Sprite, Mesh3d, MeshMaterial3d");
}
