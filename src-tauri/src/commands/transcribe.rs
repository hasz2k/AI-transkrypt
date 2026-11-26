use crate::audio::decode::decode_to_16k_mono;
use crate::AppState;
use anyhow::{anyhow, Result};
use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

#[derive(Clone, serde::Serialize)]
pub struct ProgressPayload {
    pub percent: u8,
}

// Funkcja trampoliny – wywoływana przez Whisper w trakcie obliczeń
extern "C" fn abort_callback_trampoline(user_data: *mut c_void) -> bool {
    if user_data.is_null() {
        return false;
    }
    let flag = unsafe { &*(user_data as *const AtomicBool) };
    flag.load(Ordering::Relaxed)
}

#[tauri::command]
pub fn cancel_transcription(state: tauri::State<AppState>) {
    state.cancel_flag.store(true, Ordering::Relaxed);
}

#[tauri::command]
pub async fn transcribe_file(
    app: AppHandle,
    path: String,
    language: String,
) -> Result<String, String> {
    transcribe_inner(app, path, language)
    .await
    .map_err(|e| e.to_string())
}

async fn transcribe_inner(app: AppHandle, path: String, language: String) -> Result<String> {
    let file_path = PathBuf::from(&path);
    if !file_path.exists() {
        return Err(anyhow!("Plik nie istnieje: {}", path));
    }

    // Reset flagi anulowania przed startem
    let cancel_flag = app.state::<AppState>().cancel_flag.clone();
    cancel_flag.store(false, Ordering::Relaxed);

    // Ścieżka do modelu w app_data_dir
    let app_data = app
    .path()
    .app_data_dir()
    .map_err(|e| anyhow!("Brak app_data_dir: {e}"))?;
    let model_path = app_data.join("models").join("ggml-small.bin");
    if !model_path.exists() {
        return Err(anyhow!(
            "Model nie znaleziony: {}. Skopiuj ggml-small.bin do {}",
            model_path.display(),
                           model_path.parent().unwrap().display()
        ));
    }

    // Dekodowanie audio (blokujące – przenieś do spawn_blocking)
    let app_clone = app.clone();
    let audio_samples = tokio::task::spawn_blocking(move || decode_to_16k_mono(&file_path))
    .await
    .map_err(|e| anyhow!("Błąd zadania dekodowania: {e}"))??;

    if audio_samples.is_empty() {
        return Err(anyhow!("Brak danych audio"));
    }

    // Jeśli użytkownik anulował już na etapie dekodowania
    if cancel_flag.load(Ordering::Relaxed) {
        return Err(anyhow!("Transkrypcja anulowana"));
    }

    // Transkrypcja
    let app_clone2 = app_clone.clone();
    let result = tokio::task::spawn_blocking(move || -> Result<String> {
        let ctx = WhisperContext::new_with_params(
            model_path.to_str().unwrap(),
                                                  WhisperContextParameters::default(),
        )
        .map_err(|e| anyhow!("Nie można wczytać modelu: {e}"))?;

        let mut state = ctx
        .create_state()
        .map_err(|e| anyhow!("Nie można utworzyć stanu: {e}"))?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        if language.is_empty() {
            params.set_language(None); // auto-detekcja
        } else {
            params.set_language(Some(&language));
        }
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_n_threads(num_cpus());
        params.set_translate(false);

        // Callback progresu
        let app_for_cb = app_clone2.clone();
        params.set_progress_callback_safe(move |progress: i32| {
            let percent = (progress.clamp(0, 100)) as u8;
            let _ = app_for_cb.emit("transcription-progress", ProgressPayload { percent });
        });

        // Przygotuj wskaźnik do flagi – musi żyć przez cały czas trwania state.full()
        let flag_ptr = Arc::into_raw(cancel_flag.clone()) as *mut c_void;

        unsafe {
            params.set_abort_callback(Some(abort_callback_trampoline));
            params.set_abort_callback_user_data(flag_ptr);
        }

        let full_res = state.full(params, &audio_samples);

        // Po zakończeniu (sukces lub błąd) – odzyskaj wskaźnik, żeby uniknąć wycieku pamięci
        unsafe {
            let _ = Arc::from_raw(flag_ptr as *const AtomicBool);
        }

        if cancel_flag.load(Ordering::Relaxed) {
            return Err(anyhow!("Transkrypcja anulowana"));
        }

        full_res.map_err(|e| anyhow!("Błąd transkrypcji: {e}"))?;

        let mut text = String::new();
        for seg in state.as_iter() {
            if let Ok(s) = seg.to_str() {
                text.push_str(s);
            }
        }

        let _ = app_clone2.emit(
            "transcription-progress",
            ProgressPayload { percent: 100 },
        );

        Ok(text.trim().to_string())
    })
    .await
    .map_err(|e| anyhow!("Błąd zadania transkrypcji: {e}"))??;

    Ok(result)
}

fn num_cpus() -> i32 {
    std::thread::available_parallelism()
    .map(|n| n.get() as i32)
    .unwrap_or(4)
}
