use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use std::io::Write;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};

const MODEL_URL: &str =
    "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin";
const MODEL_FILENAME: &str = "ggml-small.bin";
const MODEL_SIZE: u64 = 466_000_000;

#[derive(Clone, serde::Serialize)]
pub struct ModelProgressPayload {
    pub downloaded: u64,
    pub total: u64,
    pub percent: u8,
}

#[tauri::command]
pub async fn check_model(app: AppHandle) -> Result<bool, String> {
    check_model_inner(app).await.map_err(|e| e.to_string())
}

async fn check_model_inner(app: AppHandle) -> Result<bool> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow!("Brak app_data_dir: {e}"))?;
    let model_path = app_data.join("models").join(MODEL_FILENAME);
    Ok(model_path.exists() && model_path.metadata().map(|m| m.len() > 0).unwrap_or(false))
}

#[tauri::command]
pub async fn download_model(app: AppHandle) -> Result<String, String> {
    download_model_inner(app).await.map_err(|e| e.to_string())
}

async fn download_model_inner(app: AppHandle) -> Result<String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow!("Brak app_data_dir: {e}"))?;
    let models_dir = app_data.join("models");
    std::fs::create_dir_all(&models_dir)
        .map_err(|e| anyhow!("Nie można utworzyć katalogu models: {e}"))?;

    let model_path: PathBuf = models_dir.join(MODEL_FILENAME);
    let tmp_path = models_dir.join(format!("{MODEL_FILENAME}.part"));

    let client = reqwest::Client::new();
    let resp = client
        .get(MODEL_URL)
        .send()
        .await
        .map_err(|e| anyhow!("Błąd pobierania: {e}"))?;

    if !resp.status().is_success() {
        return Err(anyhow!("Serwer zwrócił błąd: {}", resp.status()));
    }

    let total = resp.content_length().unwrap_or(MODEL_SIZE);
    let mut downloaded: u64 = 0;
    let mut stream = resp.bytes_stream();
    let mut file = std::fs::File::create(&tmp_path)
        .map_err(|e| anyhow!("Nie można utworzyć pliku tymczasowego: {e}"))?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| anyhow!("Błąd strumienia: {e}"))?;
        file.write_all(&chunk)
            .map_err(|e| anyhow!("Błąd zapisu: {e}"))?;
        downloaded += chunk.len() as u64;
        let percent = if total > 0 {
            ((downloaded as f64 / total as f64) * 100.0).min(100.0) as u8
        } else {
            0
        };
        let _ = app.emit(
            "model-download-progress",
            ModelProgressPayload {
                downloaded,
                total,
                percent,
            },
        );
    }

    file.flush().map_err(|e| anyhow!("Błąd flush: {e}"))?;
    drop(file);

    std::fs::rename(&tmp_path, &model_path)
        .map_err(|e| anyhow!("Nie można przenieść modelu: {e}"))?;

    let _ = app.emit(
        "model-download-progress",
        ModelProgressPayload {
            downloaded: total,
            total,
            percent: 100,
        },
    );

    Ok(model_path.display().to_string())
}