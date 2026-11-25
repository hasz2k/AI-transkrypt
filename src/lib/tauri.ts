import { invoke } from "@tauri-apps/api/core";

export async function transcribeFile(
  path: string,
  language: string = "pl"
): Promise<string> {
  return invoke<string>("transcribe_file", { path, language });
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
