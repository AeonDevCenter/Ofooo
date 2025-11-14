pub fn build_conditional_class(list: &[(&str, bool)]) -> String {
    list.iter()
        .filter(|(_, cond)| *cond)
        .map(|(name, _)| *name)
        .collect::<Vec<_>>()
        .join(" ")
}
