use super::ProjectConfigInput;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn source_files_iter(src: &Path, config: &ProjectConfigInput) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(src)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| {
            // skip from foundry.toml config
            config.skip.iter().all(|skipper| !skipper.is_match(e.path())) &&

            // don't descend into directories that are meant to be excluded
            config.exclude_starting.iter().all(|exclude| !e.path().starts_with(exclude))
        })
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext == "sol").unwrap_or_default())
        .filter(|e| config.is_included(e.path())) // Final filter
        .map(|e| e.path().into())
}
