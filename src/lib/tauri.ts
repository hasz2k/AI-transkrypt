import { invoke } from "@tauri-apps/api/core";

export async function transcribeFile(
  path: string,
  language: string = "pl"
): Promise<string> {
  return invoke<string>("transcribe_file", { path, language });
}

export async function cancelTranscription(): Promise<void> {
  return invoke("cancel_transcription");
}

export async function exportTxt(
  content: string,
  defaultName: string
): Promise<string> {
  return invoke<string>("export_txt", { content, defaultName });
}

export async function exportPdf(
  content: string,
  defaultName: string
): Promise<string> {
  return invoke<string>("export_pdf", { content, defaultName });
}

export async function checkModel(): Promise<boolean> {
  return invoke<boolean>("check_model");
}

export async function downloadModel(): Promise<string> {
  return invoke<string>("download_model");
}

export async function startSystemCapture(): Promise<void> {
  return invoke("start_system_capture");
}

export async function stopSystemCapture(): Promise<string> {
  return invoke<string>("stop_system_capture");
}

export async function getDiagnosticLogs(): Promise<string> {
  return invoke<string>("get_diagnostic_logs");
}