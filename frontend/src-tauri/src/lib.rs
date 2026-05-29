mod git;

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
