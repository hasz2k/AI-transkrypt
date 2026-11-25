use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn export_txt(
    app: AppHandle,
    content: String,
    default_name: String,
) -> Result<String, String> {
    export_txt_inner(app, content, default_name)
        .await
        .map_err(|e| e.to_string())
}

async fn export_txt_inner(
    app: AppHandle,
    content: String,
    default_name: String,
) -> Result<String> {
    let path = app
        .dialog()
        .file()
        .set_file_name(&default_name)
        .add_filter("Tekst", &["txt"])
        .blocking_save_file()
        .ok_or_else(|| anyhow!("Anulowano zapis"))?;

    let path_buf: PathBuf = path
        .into_path()
        .map_err(|e| anyhow!("Nieprawidłowa ścieżka: {e}"))?;

    fs::write(&path_buf, content)
        .map_err(|e| anyhow!("Nie można zapisać pliku: {e}"))?;

    Ok(path_buf.display().to_string())
}