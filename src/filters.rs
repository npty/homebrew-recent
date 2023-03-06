pub mod filters {
    use walkdir::DirEntry;

    pub fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    pub fn is_node_modules(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s == "node_modules")
            .unwrap_or(false)
    }
}
