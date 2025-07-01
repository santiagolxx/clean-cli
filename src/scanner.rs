use crate::models::{ProjectEntry, ProjectType, Projects};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn should_skip(entry: &DirEntry) -> bool {
    let skip_keywords = [
        "node_modules",
        ".bun",
        ".rustup",
        ".cache",
        ".local/share/zed",
        ".windsurf",
        "ms-playwright-go",
        ".config",
        ".cargo",
    ];

    let path_str = entry.path().to_string_lossy();

    // Ignora directorios que contengan cualquiera de los patrones
    entry.file_type().is_dir()
        && skip_keywords
            .iter()
            .any(|pattern| path_str.contains(pattern))
}

fn should_visit(entry: &DirEntry) -> bool {
    !should_skip(entry)
}
pub fn scan_files(path: &PathBuf, scan_cargo: bool, scan_node: bool) -> Projects {
    let mut entries = Vec::new();

    if !path.is_dir() {
        eprintln!("Error: path is not a directory: {}", path.display());
        return Projects { entries };
    }

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_entry(should_visit)
        .filter_map(Result::ok)
    {
        let file_name = entry.file_name().to_string_lossy();

        if scan_cargo && file_name == "Cargo.toml" {
            if let Some(project_dir) = entry.path().parent() {
                entries.push(ProjectEntry {
                    path: project_dir.to_path_buf(),
                    project_type: ProjectType::Cargo,
                });
            }
        }

        if scan_node && file_name == "package.json" {
            if let Some(project_dir) = entry.path().parent() {
                entries.push(ProjectEntry {
                    path: project_dir.to_path_buf(),
                    project_type: ProjectType::Node,
                });
            }
        }
    }

    Projects { entries }
}
