mod audio;
mod commands;

use commands::capture::CaptureState;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub cancel_flag: Arc<AtomicBool>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .manage(AppState {
        cancel_flag: Arc::new(AtomicBool::new(false)),
    })
    .manage(CaptureState {
        recorder: Mutex::new(None),
            output_path: Mutex::new(None),
    })
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![
        commands::transcribe::transcribe_file,
        commands::transcribe::cancel_transcription,
        commands::export::export_txt,
        commands::export_pdf::export_pdf,
        commands::model::check_model,
        commands::model::download_model,
        commands::capture::start_system_capture,
        commands::capture::stop_system_capture,
        commands::diagnostics::get_diagnostic_logs,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
