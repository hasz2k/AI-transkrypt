use std::process::Command;
use tauri::{AppHandle, Manager};

fn run_cmd(bin: &str, args: &[&str]) -> String {
    Command::new(bin)
    .args(args)
    .output()
    .ok()
    .filter(|o| o.status.success())
    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    .filter(|s| !s.is_empty())
    .unwrap_or_else(|| "brak / niedostępne".to_string())
}

#[tauri::command]
pub fn get_diagnostic_logs(app: AppHandle) -> Result<String, String> {
    let pkg = app.package_info();
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "unknown".into());
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "unknown".into());

    let kernel = if cfg!(target_os = "linux") {
        run_cmd("uname", &["-srm"])
    } else if cfg!(target_os = "macos") {
        run_cmd("sw_vers", &["-productVersion"])
    } else {
        "Windows".to_string()
    };

    let gpu_info = if cfg!(target_os = "linux") {
        let lspci = run_cmd("lspci", &[]);
        lspci
        .lines()
        .filter(|l| {
            let low = l.to_lowercase();
            low.contains("vga") || low.contains("3d") || low.contains("display")
        })
        .collect::<Vec<_>>()
        .join(" | ")
    } else {
        "n/d".to_string()
    };

    let audio_server = if cfg!(target_os = "linux") {
        run_cmd("pactl", &["info"])
        .lines()
        .find(|l| l.starts_with("Server Name:"))
        .unwrap_or("pactl niedostępny")
        .to_string()
    } else {
        "n/d".to_string()
    };

    let model_status = match app.path().app_data_dir() {
        Ok(dir) => {
            let exists = std::fs::read_dir(&dir)
            .ok()
            .map(|entries| {
                entries
                .flatten()
                .any(|e| e.path().extension().and_then(|s| s.to_str()) == Some("bin"))
            })
            .unwrap_or(false);
            if exists {
                "zainstalowany (.bin obecny)".to_string()
            } else {
                "brak pliku .bin w katalogu danych".to_string()
            }
        }
        Err(e) => format!("błąd ścieżki: {e}"),
    };

    let report = format!(
        "=== AI Transkrypt — Raport diagnostyczny ===\n\
Wersja aplikacji: {} v{}\n\
System / Arch: {} ({})\n\
Kernel / OS ver: {}\n\
Środowisko / Sesja: {} ({})\n\
GPU: {}\n\
Serwer audio: {}\n\
Model Whisper: {}\n",
pkg.name,
pkg.version,
os,
arch,
kernel,
desktop,
session_type,
if gpu_info.is_empty() { "nie wykryto" } else { &gpu_info },
    audio_server,
    model_status
    );

    Ok(report)
}
