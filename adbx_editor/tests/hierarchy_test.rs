#[cfg(test)]
mod tests {
    use adbx_editor::ui::hierarchy::HierarchyView;
    use bevy::prelude::*;

    #[test]
    fn test_hierarchy_view() {
        let view = HierarchyView::default();
        assert_eq!(view.expanded_entities.len(), 0);
    }
}
