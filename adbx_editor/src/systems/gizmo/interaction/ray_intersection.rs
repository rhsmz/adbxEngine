use bevy::prelude::*;

/// レイと線分の最短距離を計算
pub fn ray_to_line_distance(ray_origin: Vec3, ray_dir: Vec3, line_start: Vec3, line_end: Vec3) -> f32 {
    let line_dir = line_end - line_start;
    let w = ray_origin - line_start;
    
    let a = ray_dir.dot(ray_dir);
    let b = ray_dir.dot(line_dir);
    let c = line_dir.dot(line_dir);
    let d = ray_dir.dot(w);
    let e = line_dir.dot(w);
    
    let denom = a * c - b * b;
    if denom.abs() < 1e-6 {
        // レイと線分が平行
        return w.length();
    }
    
    let t_ray = (b * e - c * d) / denom;
    let t_line = (a * e - b * d) / denom;
    
    // 線分の範囲内に制限
    let t_line_clamped = t_line.clamp(0.0, 1.0);
    let closest_point_on_line = line_start + line_dir * t_line_clamped;
    let closest_point_on_ray = ray_origin + ray_dir * t_ray.max(0.0);
    
    closest_point_on_ray.distance(closest_point_on_line)
}

/// レイと球の交差判定
pub fn ray_sphere_intersection(ray_origin: Vec3, ray_dir: Vec3, sphere_center: Vec3, sphere_radius: f32) -> Option<f32> {
    let oc = ray_origin - sphere_center;
    let a = ray_dir.dot(ray_dir);
    let b = 2.0 * oc.dot(ray_dir);
    let c = oc.dot(oc) - sphere_radius * sphere_radius;
    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant < 0.0 {
        return None;
    }
    
    let t = (-b - discriminant.sqrt()) / (2.0 * a);
    if t >= 0.0 {
        Some(t)
    } else {
        None
    }
}

/// レイと円（平面内）の交差判定
pub fn ray_circle_intersection(
    ray_origin: Vec3,
    ray_dir: Vec3,
    circle_center: Vec3,
    circle_normal: Vec3,
    circle_radius: f32,
) -> Option<f32> {
    // 平面との交差を計算
    let plane_d = -circle_normal.dot(circle_center);
    let denom = circle_normal.dot(ray_dir);
    
    if denom.abs() < 1e-6 {
        return None; // レイが平面と平行
    }
    
    let t = -(circle_normal.dot(ray_origin) + plane_d) / denom;
    if t < 0.0 {
        return None;
    }
    
    let intersection = ray_origin + ray_dir * t;
    let dist_from_center = (intersection - circle_center).length();
    
    if dist_from_center <= circle_radius {
        Some(t)
    } else {
        None
    }
}
