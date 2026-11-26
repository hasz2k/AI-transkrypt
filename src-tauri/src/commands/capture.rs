use std::process::Child;
use std::sync::Mutex;
use tauri::State;

pub struct CaptureState {
    pub recorder: Mutex<Option<Child>>,
    pub output_path: Mutex<Option<String>>,
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub fn start_system_capture(state: State<CaptureState>) -> Result<(), String> {
    use std::process::{Command, Stdio};

    let mut recorder_guard = state.recorder.lock().unwrap();

    if recorder_guard.is_some() {
        return Err("Nagrywanie już trwa".to_string());
    }

    let output = Command::new("pactl")
    .arg("get-default-sink")
    .output()
    .map_err(|e| format!("Błąd uruchamiania pactl: {}", e))?;

    let sink_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if sink_name.is_empty() {
        return Err("Nie można znaleźć domyślnego wyjścia audio".to_string());
    }

    let temp_path = std::env::temp_dir().join("ai_transkrypt_capture.wav");
    let path_str = temp_path.to_string_lossy().to_string();

    let child = Command::new("pw-record")
    .arg("--target")
    .arg(&sink_name)
    .arg("-P")
    .arg("{ stream.capture.sink = true }")
    .arg("--format")
    .arg("s16")
    .arg("--rate")
    .arg("16000")
    .arg("--channels")
    .arg("1")
    .arg(&path_str)
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .map_err(|e| format!("Nie można uruchomić pw-record: {}", e))?;

    *recorder_guard = Some(child);
    *state.output_path.lock().unwrap() = Some(path_str);

    Ok(())
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub fn stop_system_capture(state: State<CaptureState>) -> Result<String, String> {
    let mut recorder_guard = state.recorder.lock().unwrap();

    if let Some(mut child) = recorder_guard.take() {
        let pid = child.id() as i32;
        unsafe {
            libc::kill(pid, libc::SIGINT);
        }

        let _ = child.wait();

        let path = state.output_path.lock().unwrap().take();
        path.ok_or_else(|| "Brak ścieżki do pliku".to_string())
    } else {
        Err("Nagrywanie nie jest aktywne".to_string())
    }
}

#[cfg(not(target_os = "linux"))]
#[tauri::command]
pub fn start_system_capture(_state: State<CaptureState>) -> Result<(), String> {
    Err("Nagrywanie dźwięku systemowego na macOS jest w trakcie wdrażania — w wersji testowej wybierz plik audio/wideo.".to_string())
}

#[cfg(not(target_os = "linux"))]
#[tauri::command]
pub fn stop_system_capture(_state: State<CaptureState>) -> Result<String, String> {
    Err("Nagrywanie nie jest aktywne".to_string())
}
