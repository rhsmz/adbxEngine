use bevy::prelude::*;
use super::super::resource::ResizeEdge;

/// リサイズエッジを検出
pub fn detect_resize_edge(
    cursor_pos: Vec2,
    node: &Node,
    transform: &GlobalTransform,
    window: &Window,
) -> (bool, Option<ResizeEdge>) {
    // リサイズ可能なエッジの閾値（ピクセル）
    const RESIZE_THRESHOLD: f32 = 8.0;
    
    // パネルの位置とサイズを取得
    // GlobalTransformから画面座標を計算
    let node_size = match (node.width, node.height) {
        (Val::Px(w), Val::Px(h)) => Vec2::new(w, h),
        (Val::Percent(w), Val::Percent(h)) => {
            Vec2::new(
                window.width() * w / 100.0,
                window.height() * h / 100.0,
            )
        }
        (Val::Px(w), Val::Percent(h)) => {
            Vec2::new(w, window.height() * h / 100.0)
        }
        (Val::Percent(w), Val::Px(h)) => {
            Vec2::new(window.width() * w / 100.0, h)
        }
        _ => Vec2::ZERO,
    };
    
    // パネルの左上座標を取得
    let node_pos = transform.translation().truncate();
    // BevyのUI座標系は左上が原点なので、変換が必要な場合がある
    let node_left = node_pos.x;
    let node_top = node_pos.y;
    let node_right = node_left + node_size.x;
    let node_bottom = node_top + node_size.y;
    
    // マウス位置がパネルのどのエッジに近いかを判定
    let dist_to_left = (cursor_pos.x - node_left).abs();
    let dist_to_right = (cursor_pos.x - node_right).abs();
    let dist_to_top = (cursor_pos.y - node_top).abs();
    let dist_to_bottom = (cursor_pos.y - node_bottom).abs();
    
    // 最も近いエッジを検出
    let min_dist = dist_to_left.min(dist_to_right).min(dist_to_top).min(dist_to_bottom);
    
    if min_dist < RESIZE_THRESHOLD {
        // リサイズ可能なエッジを返す
        if dist_to_left == min_dist {
            return (true, Some(ResizeEdge::Left));
        } else if dist_to_right == min_dist {
            return (true, Some(ResizeEdge::Right));
        } else if dist_to_top == min_dist {
            return (true, Some(ResizeEdge::Top));
        } else if dist_to_bottom == min_dist {
            return (true, Some(ResizeEdge::Bottom));
        }
    }
    
    // リサイズエッジではない
    (false, None)
}
