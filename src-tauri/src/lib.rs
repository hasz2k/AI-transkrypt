mod audio;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![
        commands::transcribe::transcribe_file,
        commands::export::export_txt,
        commands::export_pdf::export_pdf,
        commands::model::check_model,
        commands::model::download_model,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
