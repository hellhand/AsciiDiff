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

    let (left_html, right_html) = if left.exists && right.exists {
        render::compute_diff_html(&left.content, &right.content)
    } else if left.exists {
        (render::render_asciidoc(&left.content), String::new())
    } else if right.exists {
        (String::new(), render::render_asciidoc(&right.content))
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
