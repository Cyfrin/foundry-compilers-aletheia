use super::ProjectConfigInput;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn source_files_iter(
    root: &Path,
    config: &ProjectConfigInput,
) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext == "sol").unwrap_or_default())
        .filter(|e| config.is_included(e.path()))
        .map(|e| e.path().into())
}
