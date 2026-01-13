use bevy::prelude::*;
use super::super::hierarchy_view_resource::{HierarchyView, HierarchyDragState, HierarchyItem};
use crate::systems::selection::Selection;

/// ヒエラルキーアイテムを生成
pub fn spawn_hierarchy_view_item(
    commands: &mut Commands,
    entity: Entity,
    name: &Name,
    hierarchy_view: &HierarchyView,
    selection: &Selection,
    drag_state: &HierarchyDragState,
    children_query: &Query<&Children>,
    _name_query: &Query<&Name>,
    parent_ui: Entity,
    indent: u32,
    display_index: usize, // 表示位置のインデックス（仮想スクロール用）
) -> Entity {
    let is_expanded = hierarchy_view.expanded_entities.contains(&entity);
    let is_selected = selection.selected_entities.contains(&entity);
    let is_dragging = drag_state.dragging_entity == Some(entity);
    let is_drop_target = drag_state.hovered_drop_target == Some(entity);
    
    // 背景色を決定（ドラッグ中、ドロップ先、選択状態に応じて）
    let bg_color = if is_dragging {
        Color::srgb(0.4, 0.6, 0.8) // ドラッグ中は明るい青
    } else if is_drop_target {
        Color::srgb(0.6, 0.8, 0.4) // ドロップ先は明るい緑
    } else if is_selected {
        Color::srgb(0.2, 0.4, 0.6) // 選択中は青
    } else {
        Color::NONE
    };
    
    let item_entity = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(hierarchy_view.item_height),
            flex_direction: FlexDirection::Row,
            padding: UiRect::left(Val::Px((indent * 15) as f32)),
            position_type: bevy::ui::PositionType::Absolute,
            top: Val::Px(display_index as f32 * hierarchy_view.item_height),
            ..default()
        },
        HierarchyItem { entity },
        Name::new(format!("HierarchyItem_{}", entity.index())),
        BackgroundColor(bg_color),
    )).with_children(|item| {
        // 展開/折りたたみボタン
        if children_query.get(entity).is_ok() {
            item.spawn((
                Node {
                    width: Val::Px(15.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Name::new(format!("ExpandButton_{}", entity.index())),
            )).with_children(|btn| {
                btn.spawn((
                    Text::new(if is_expanded { "▼" } else { "▶" }),
                    bevy::text::TextFont {
                        font_size: 10.0,
                        ..default()
                    },
                    bevy::text::TextColor(Color::WHITE),
                ));
            });
        } else {
            // 子がない場合はスペーサー
            item.spawn((
                Node {
                    width: Val::Px(15.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
            ));
        }
        
        // エンティティ名
        item.spawn((
            Node {
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new(format!("EntityName_{}", entity.index())),
        )).with_children(|name_node| {
            name_node.spawn((
                Text::new(name.as_str()),
                bevy::text::TextFont {
                    font_size: 12.0,
                    ..default()
                },
                bevy::text::TextColor(if is_selected {
                    Color::srgb(1.0, 1.0, 1.0)
                } else {
                    Color::srgb(0.9, 0.9, 0.9)
                }),
            ));
        });
    }).id();
    
    commands.entity(parent_ui).add_child(item_entity);
    
    // 子エンティティは仮想スクロールの実装では、collect_visible_entitiesで処理されるため、
    // ここでは個別にspawnしない（表示範囲内のエンティティのみがspawnされる）
    // これにより、大規模シーンでもパフォーマンスを維持できる
    
    item_entity
}
