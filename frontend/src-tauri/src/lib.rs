mod git;
mod render;

#[allow(unused_imports)]
use git::{BranchInfo, ChangedFile, FileContent};

#[tauri::command]
async fn open_repository(path: String) -> Result<Vec<BranchInfo>, String> {
    git::list_refs(&path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_changed_files(
    repo_path: String,
    left_ref: String,
    right_ref: String,
) -> Result<Vec<ChangedFile>, String> {
    git::diff_refs(&repo_path, &left_ref, &right_ref).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_file_content(
    repo_path: String,
    git_ref: String,
    file_path: String,
) -> Result<FileContent, String> {
    git::read_file_at_ref(&repo_path, &git_ref, &file_path).map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
struct RenderedDiff {
    left_html: String,
    right_html: String,
    left_exists: bool,
    right_exists: bool,
}

#[tauri::command]
async fn render_diff(
    repo_path: String,
    left_ref: String,
    right_ref: String,
    file_path: String,
) -> Result<RenderedDiff, String> {
    let left = git::read_file_at_ref(&repo_path, &left_ref, &file_path)
        .map_err(|e| e.to_string())?;
    let right = git::read_file_at_ref(&repo_path, &right_ref, &file_path)
        .map_err(|e| e.to_string())?;

    // Resolve include:: directives from the git object store
    let left_resolved = if left.exists {
        resolve_includes(&repo_path, &left_ref, &file_path, &left.content, 0)
    } else {
        String::new()
    };
    let right_resolved = if right.exists {
        resolve_includes(&repo_path, &right_ref, &file_path, &right.content, 0)
    } else {
        String::new()
    };

    let (left_html, right_html) = if left.exists && right.exists {
        render::compute_diff_html(&left_resolved, &right_resolved)
    } else if left.exists {
        (render::render_asciidoc(&left_resolved), String::new())
    } else if right.exists {
        (String::new(), render::render_asciidoc(&right_resolved))
    } else {
        (String::new(), String::new())
    };

    Ok(RenderedDiff {
        left_html,
        right_html,
        left_exists: left.exists,
        right_exists: right.exists,
    })
}

const MAX_INCLUDE_DEPTH: u8 = 8;

/// Recursively resolves `include::path[]` directives by reading from the git object store.
/// The `base_path` is used to resolve relative includes.
fn resolve_includes(
    repo_path: &str,
    git_ref: &str,
    file_path: &str,
    content: &str,
    depth: u8,
) -> String {
    if depth >= MAX_INCLUDE_DEPTH {
        return content.to_string();
    }

    let base_dir = std::path::Path::new(file_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let mut result = String::new();
    for line in content.lines() {
        if line.starts_with("include::") && line.ends_with("[]") {
            let raw_path = line
                .trim_start_matches("include::")
                .trim_end_matches("[]")
                .trim();

            // Resolve relative path against the including file's directory
            let include_path = if raw_path.starts_with('/') {
                raw_path.trim_start_matches('/').to_string()
            } else if base_dir.is_empty() {
                raw_path.to_string()
            } else {
                format!("{}/{}", base_dir, raw_path)
            };

            // Read the included file from git
            match git::read_file_at_ref(repo_path, git_ref, &include_path) {
                Ok(fc) if fc.exists => {
                    // Recursively resolve includes in the included content
                    let expanded = resolve_includes(
                        repo_path,
                        git_ref,
                        &include_path,
                        &fc.content,
                        depth + 1,
                    );
                    result.push_str(&expanded);
                    result.push('\n');
                }
                _ => {
                    // Include not found -- emit a marker so the renderer shows it
                    result.push_str(line);
                    result.push('\n');
                }
            }
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_repo_path() -> String {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        format!("{}/../../../test-repo", manifest_dir)
    }

    #[test]
    fn test_resolve_includes_expands_partials() {
        let repo = test_repo_path();
        if !std::path::Path::new(&repo).join(".git").exists() {
            eprintln!("Skipping: test-repo not found. Run `make test-repo` first.");
            return;
        }

        let content = "= Title\n\ninclude::partials/_header.adoc[]\n\n== Section";
        let resolved = resolve_includes(&repo, "main", "architecture-guide.adoc", content, 0);

        // The include should be expanded (contains :toc: from _header.adoc)
        assert!(resolved.contains(":toc:"), "Include was not resolved. Got:\n{}", resolved);
        assert!(resolved.contains(":toclevels:"), "Include content missing toclevels");
        // Original content should still be there
        assert!(resolved.contains("= Title"));
        assert!(resolved.contains("== Section"));
    }

    #[test]
    fn test_resolve_includes_missing_file_keeps_directive() {
        let repo = test_repo_path();
        if !std::path::Path::new(&repo).join(".git").exists() {
            eprintln!("Skipping: test-repo not found. Run `make test-repo` first.");
            return;
        }

        let content = "include::nonexistent.adoc[]";
        let resolved = resolve_includes(&repo, "main", "architecture-guide.adoc", content, 0);
        assert!(resolved.contains("include::nonexistent.adoc[]"));
    }

    #[test]
    fn test_resolve_includes_respects_depth_limit() {
        // Even without a real repo, depth limit should return content unchanged
        let content = "include::foo.adoc[]";
        let resolved = resolve_includes("/nonexistent", "main", "test.adoc", content, MAX_INCLUDE_DEPTH);
        assert_eq!(resolved, content);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_repository,
            list_changed_files,
            get_file_content,
            render_diff,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
