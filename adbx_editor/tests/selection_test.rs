#[cfg(test)]
mod tests {
    use adbx_editor::systems::selection::Selection;

    #[test]
    fn test_selection() {
        let selection = Selection::default();
        assert_eq!(selection.selected_entities.len(), 0);
    }
}
