use anyhow::{anyhow, Result};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn export_pdf(
    app: AppHandle,
    content: String,
    default_name: String,
) -> Result<String, String> {
    export_pdf_inner(app, content, default_name)
    .await
    .map_err(|e| e.to_string())
}

async fn export_pdf_inner(
    app: AppHandle,
    content: String,
    default_name: String,
) -> Result<String> {
    let path = app
    .dialog()
    .file()
    .set_file_name(&default_name)
    .add_filter("PDF", &["pdf"])
    .blocking_save_file()
    .ok_or_else(|| anyhow!("Anulowano zapis"))?;

    let path_buf: PathBuf = path
    .into_path()
    .map_err(|e| anyhow!("Nieprawidłowa ścieżka: {e}"))?;

    let font_path = app
    .path()
    .resource_dir()
    .map_err(|e| anyhow!("Brak resource_dir: {e}"))?
    .join("resources/fonts/Inter-Regular.ttf");

    let font_data = std::fs::read(&font_path)
    .map_err(|e| anyhow!("Nie można wczytać czcionki {}: {e}", font_path.display()))?;

    generate_pdf(&content, &font_data, &path_buf)?;

    Ok(path_buf.display().to_string())
}

fn generate_pdf(content: &str, font_data: &[u8], out_path: &PathBuf) -> Result<()> {
    let mut doc = PdfDocument::new("Transkrypcja");

    // 1. Wczytaj czcionkę jako ParsedFont (API 0.12)
    let mut font_warnings = Vec::new();
    let font = ParsedFont::from_bytes(font_data, 0, &mut font_warnings)
    .ok_or_else(|| anyhow!("Nie można sparsować czcionki"))?;

    // 2. Dodaj czcionkę do dokumentu – zwraca FontId
    let font_id = doc.add_font(&font);

    // 3. Zbuduj strony z automatyczną paginacją
    let mut pages: Vec<PdfPage> = Vec::new();
    let mut ops: Vec<Op> = Vec::new();

    let page_width_mm = 210.0;
    let page_height_mm = 297.0;
    let font_size = 11.0;
    let line_height = 5.5;
    let margin_left = 20.0;
    let margin_top = 270.0;
    let margin_bottom = 20.0;
    let max_width_mm = 170.0;

    let mut y = margin_top;

    for paragraph in content.split('\n') {
        for line in wrap_text(paragraph, font_size, max_width_mm) {
            // Jeśli zabraknie miejsca na obecnej stronie, zapisz ją i zacznij nową
            if y < margin_bottom {
                pages.push(PdfPage::new(
                    Mm(page_width_mm),
                                        Mm(page_height_mm),
                                        std::mem::take(&mut ops),
                ));
                y = margin_top;

                // Pomijamy puste linie tuż po przejściu na nową stronę
                if line.is_empty() {
                    continue;
                }
            }

            if !line.is_empty() {
                ops.push(Op::StartTextSection);
                ops.push(Op::SetTextCursor {
                    pos: Point {
                        x: Mm(margin_left).into(),
                         y: Mm(y).into(),
                    },
                });
                ops.push(Op::SetFont {
                    font: PdfFontHandle::External(font_id.clone()),
                         size: Pt(font_size),
                });
                ops.push(Op::ShowText {
                    items: vec![TextItem::Text(line)],
                });
                ops.push(Op::EndTextSection);
            }

            y -= line_height;
        }
        y -= line_height * 0.3;
    }

    // Dodaj ostatnią stronę (lub pustą stronę, jeśli dokument nie miał tekstu)
    if !ops.is_empty() || pages.is_empty() {
        pages.push(PdfPage::new(Mm(page_width_mm), Mm(page_height_mm), ops));
    }

    // 4. Przypisz wszystkie wygenerowane strony do dokumentu
    doc.with_pages(pages);

    // 5. Zapisz
    let file = File::create(out_path)?;
    let mut writer = BufWriter::new(file);
    let mut warnings = Vec::new();
    let pdf_bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);
    std::io::Write::write_all(&mut writer, &pdf_bytes)?;

    Ok(())
}

fn wrap_text(text: &str, size: f32, max_width_mm: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let char_width_mm = size * 0.5 / 2.835;

    for word in text.split_whitespace() {
        let word_len = word.chars().count() as f32 * char_width_mm;
        let current_len = current.chars().count() as f32 * char_width_mm;

        if current_len + word_len > max_width_mm && !current.is_empty() {
            lines.push(current.trim().to_string());
            current = String::new();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }

    if !current.is_empty() {
        lines.push(current.trim().to_string());
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
