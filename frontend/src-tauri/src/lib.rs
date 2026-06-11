pub mod git;
pub mod render;

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
    left_source: String,
    right_source: String,
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

    // Resolve include:: directives and get line mappings (expanded line -> raw line number)
    let (left_resolved, left_map) = if left.exists {
        resolve_includes_mapped(&repo_path, &left_ref, &file_path, &left.content, 0)
    } else {
        (String::new(), Vec::new())
    };
    let (right_resolved, right_map) = if right.exists {
        resolve_includes_mapped(&repo_path, &right_ref, &file_path, &right.content, 0)
    } else {
        (String::new(), Vec::new())
    };

    let (left_html, right_html) = if left.exists && right.exists {
        // Collect all unique source files referenced in the expanded content
        let mut all_files: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for loc in &left_map {
            all_files.insert(&loc.file);
        }
        for loc in &right_map {
            all_files.insert(&loc.file);
        }

        // Get diff lines for each source file
        let mut diff_by_file: std::collections::HashMap<String, git::FileDiffLines> =
            std::collections::HashMap::new();
        for file in all_files {
            if let Ok(diff) = git::diff_file_lines(&repo_path, &left_ref, &right_ref, file) {
                if !diff.left_changed.is_empty() || !diff.right_changed.is_empty() {
                    diff_by_file.insert(file.to_string(), diff);
                }
            }
        }

        // Map source locations to expanded line numbers
        let left_changed: Vec<u32> = left_map.iter().enumerate()
            .filter_map(|(idx, loc)| {
                diff_by_file.get(&loc.file)
                    .and_then(|d| {
                        if d.left_changed.contains(&loc.line) {
                            Some((idx as u32) + 1)
                        } else {
                            None
                        }
                    })
            })
            .collect();

        let right_changed: Vec<u32> = right_map.iter().enumerate()
            .filter_map(|(idx, loc)| {
                diff_by_file.get(&loc.file)
                    .and_then(|d| {
                        if d.right_changed.contains(&loc.line) {
                            Some((idx as u32) + 1)
                        } else {
                            None
                        }
                    })
            })
            .collect();

        let expanded_diff = git::FileDiffLines {
            left_changed,
            right_changed,
        };

        render::compute_diff_html(&left_resolved, &right_resolved, &expanded_diff)
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
        left_source: left_resolved,
        right_source: right_resolved,
        left_exists: left.exists,
        right_exists: right.exists,
    })
}

/// Tracks where each expanded line originated from.
#[derive(Debug, Clone)]
struct SourceLocation {
    /// The file path (relative to repo root) this line came from
    file: String,
    /// 1-indexed line number within that file
    line: u32,
}

const MAX_INCLUDE_DEPTH: u8 = 8;

/// Recursively resolves `include::path[]` directives by reading from the git object store.
/// The `base_path` is used to resolve relative includes.
#[cfg(test)]
fn resolve_includes(
    repo_path: &str,
    git_ref: &str,
    file_path: &str,
    content: &str,
    depth: u8,
) -> String {
    resolve_includes_mapped(repo_path, git_ref, file_path, content, depth).0
}

/// Like `resolve_includes` but also returns a mapping from each expanded line index (0-based)
/// to the source file and line number it originated from.
fn resolve_includes_mapped(
    repo_path: &str,
    git_ref: &str,
    file_path: &str,
    content: &str,
    depth: u8,
) -> (String, Vec<SourceLocation>) {
    if depth >= MAX_INCLUDE_DEPTH {
        let mapping: Vec<SourceLocation> = (1..=(content.lines().count() as u32))
            .map(|line| SourceLocation { file: file_path.to_string(), line })
            .collect();
        return (content.to_string(), mapping);
    }

    let base_dir = std::path::Path::new(file_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let mut result = String::new();
    let mut line_map: Vec<SourceLocation> = Vec::new();

    for (raw_idx, line) in content.lines().enumerate() {
        let raw_lineno = (raw_idx as u32) + 1; // 1-indexed

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
                    let (expanded, child_map) = resolve_includes_mapped(
                        repo_path,
                        git_ref,
                        &include_path,
                        &fc.content,
                        depth + 1,
                    );
                    for (idx, expanded_line) in expanded.lines().enumerate() {
                        result.push_str(expanded_line);
                        result.push('\n');
                        // Preserve the child's source location mapping
                        if let Some(loc) = child_map.get(idx) {
                            line_map.push(loc.clone());
                        } else {
                            // Fallback: attribute to the include directive itself
                            line_map.push(SourceLocation {
                                file: file_path.to_string(),
                                line: raw_lineno,
                            });
                        }
                    }
                }
                _ => {
                    // Include not found -- emit a marker so the renderer shows it
                    result.push_str(line);
                    result.push('\n');
                    line_map.push(SourceLocation {
                        file: file_path.to_string(),
                        line: raw_lineno,
                    });
                }
            }
        } else {
            result.push_str(line);
            result.push('\n');
            line_map.push(SourceLocation {
                file: file_path.to_string(),
                line: raw_lineno,
            });
        }
    }
    (result, line_map)
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
    // Workaround for webkit2gtk DMA-BUF renderer bug on Linux:
    // the DMA-BUF renderer fails to trigger repaints after DOM changes,
    // so the UI only updates visually on window resize.
    // Disabling it falls back to shared-memory rendering which works correctly.
    // See: https://bugs.webkit.org/show_bug.cgi?id=261874
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

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
