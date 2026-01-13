pub mod handle_detection;
mod main;
pub mod ray_intersection;
pub mod transform_update;

// 公開API
pub use handle_detection::detect_gizmo_handle_click;
pub use main::handle_gizmo_interaction;

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use ray_intersection::{
    ray_circle_intersection, ray_sphere_intersection, ray_to_line_distance,
};
#[allow(unused_imports)]
pub use transform_update::update_transform_from_gizmo;
