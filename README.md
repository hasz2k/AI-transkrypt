# AI Transkrypt

[🇵🇱 Polski](#-polski) | [🇬🇧 English](#-english)

---

## 🇵🇱 Polski

**AI Transkrypt** to darmowa, w 100% lokalna aplikacja desktopowa do transkrypcji audio i wideo na tekst. Działa w pełni offline, bez przesyłania danych do chmury, co zapewnia pełną prywatność. Aplikacja obsługuje formaty PDF i TXT, a interfejs jest w pełni dwujęzyczny (polski/angielski).

### ✨ Główne funkcje

- **100% offline i prywatnie** – żadne dane nie opuszczają Twojego komputera.
- **Wiele formatów audio/wideo** – obsługa `.wav`, `.mp3`, `.flac`, `.ogg`, `.m4a`, `.mp4`, `.mkv`, `.webm`.
- **Automatyczny resampling** – konwersja dowolnego pliku do formatu wymaganego przez AI (16 kHz).
- **Wybór języka transkrypcji** – polski, angielski lub auto-detekcja.
- **Eksport TXT i PDF** – gotowe notatki z polskimi znakami i automatyczną paginacją.
- **Menedżer modelu AI** – aplikacja pobierze model (Whisper Small) przy pierwszym uruchomieniu.
- **Dwujęzyczny interfejs** – przełącznik języka UI (PL/EN).

### 🚧 Status projektu

**Faza 1: MVP (Beta v0.1.0)** – działa na Linuxie (CachyOS/Arch). Wersje na macOS i Windows są w planach.

### 🖥️ Wymagania systemowe

- **System:** Linux (testowano na CachyOS/Arch)
- **Karta graficzna:** Zalecana karta z obsługą Vulkan (znacznie przyspiesza transkrypcję)
- **Miejsce na dysku:** ~1 GB (w tym model AI ~466 MB)

### 🚀 Instalacja i uruchomienie (dla użytkowników)

1. Pobierz plik `.AppImage` (lub `.deb`/`.rpm`) z zakładki [Releases](https://github.com/hasz2k/AI-transkrypt/releases).
2. Nadaj uprawnienia do wykonania (dla `.AppImage`):
   ```bash
   chmod +x ai-transkrypt_0.1.0_amd64.AppImage
   ./ai-transkrypt_0.1.0_amd64.AppImage

    Przy pierwszym uruchomieniu aplikacja poprosi o pobranie modelu AI (Whisper Small, ~466 MB).

    Przeciągnij plik audio/wideo, wybierz język i kliknij Transkrybuj.

🛠️ Kompilacja ze źródeł (dla deweloperów)

Wymagane narzędzia: Rust, Node.js, pnpm, Tauri CLI.

Zależności systemowe (Arch/CachyOS):
bash

sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl \
  appmenu-gtk-module libappindicator-gtk3 librsvg xdotool ffmpeg \
  vulkan-headers vulkan-icd-loader shaderc spirv-headers cmake clang

Kroki:
bash

git clone https://github.com/hasz2k/AI-transkrypt.git
cd AI-transkrypt
pnpm install
pnpm tauri dev      # Tryb deweloperski
pnpm tauri build    # Budowanie wersji produkcyjnej

📂 Struktura projektu
text

src/                    # Frontend (React + TypeScript)
├── components/         # Komponenty UI
├── lib/tauri.ts        # Wrapper na komendy Tauri
└── App.tsx

src-tauri/              # Backend (Rust)
├── src/
│   ├── audio/decode.rs # Dekodowanie audio i resampling
│   ├── commands/       # Komendy (transkrypcja, eksport, model)
│   └── lib.rs
├── resources/fonts/    # Czcionka Inter (dla PDF)
└── Cargo.toml

⚠️ Znane ograniczenia (Beta)

    Wymaga Linuksa (macOS/Windows w kolejnych fazach).

    Transkrypcja na samym CPU jest znacznie wolniejsza niż na GPU.

    Brak możliwości anulowania transkrypcji w trakcie.

📄 Licencja

Projekt jest obecnie w fazie beta. Licencja zostanie ogłoszona przed pierwszym publicznym wydaniem (rozważane: AGPL-3.0 lub BSL).
🇬🇧 English

AI Transkrypt is a free, 100% local desktop application for transcribing audio and video to text. It works fully offline, without sending any data to the cloud, ensuring complete privacy. The app supports PDF and TXT export, and the interface is fully bilingual (Polish/English).
✨ Key Features

    100% offline & private – no data leaves your computer.

    Multiple audio/video formats – supports .wav, .mp3, .flac, .ogg, .m4a, .mp4, .mkv, .webm.

    Automatic resampling – converts any file to the format required by AI (16 kHz).

    Transcription language selection – Polish, English, or auto-detect.

    TXT and PDF export – ready-to-print notes with proper characters and automatic pagination.

    AI Model Manager – the app downloads the model (Whisper Small) on first launch.

    Bilingual interface – UI language toggle (PL/EN).

🚧 Project Status

Phase 1: MVP (Beta v0.1.0) – works on Linux (CachyOS/Arch). macOS and Windows versions are planned.
🖥️ System Requirements

    OS: Linux (tested on CachyOS/Arch)

    GPU: Vulkan-capable graphics card recommended (significantly speeds up transcription).

    Disk space: ~1 GB (including AI model ~466 MB).

🚀 Installation & Running (for users)

    Download the .AppImage (or .deb/.rpm) from the Releases page.

    Make it executable (for .AppImage):
    bash

    chmod +x ai-transkrypt_0.1.0_amd64.AppImage
    ./ai-transkrypt_0.1.0_amd64.AppImage

    On first launch, the app will prompt you to download the AI model (Whisper Small, ~466 MB).

    Drag & drop an audio/video file, select the language, and click Transcribe.

🛠️ Building from Source (for developers)

Required tools: Rust, Node.js, pnpm, Tauri CLI.

System dependencies (Arch/CachyOS):
bash

sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl \
  appmenu-gtk-module libappindicator-gtk3 librsvg xdotool ffmpeg \
  vulkan-headers vulkan-icd-loader shaderc spirv-headers cmake clang

Steps:
bash

git clone https://github.com/hasz2k/AI-transkrypt.git
cd AI-transkrypt
pnpm install
pnpm tauri dev      # Development mode
pnpm tauri build    # Production build

📂 Project Structure
text

src/                    # Frontend (React + TypeScript)
├── components/         # UI Components
├── lib/tauri.ts        # Tauri commands wrapper
└── App.tsx

src-tauri/              # Backend (Rust)
├── src/
│   ├── audio/decode.rs # Audio decoding and resampling
│   ├── commands/       # Commands (transcription, export, model)
│   └── lib.rs
├── resources/fonts/    # Inter font (for PDF)
└── Cargo.toml

⚠️ Known Limitations (Beta)

    Linux only (macOS/Windows in future phases).

    CPU-only transcription is much slower than GPU.

    No way to cancel transcription mid-process.

📄 License

The project is currently in beta. The license will be announced before the first public release (considering: AGPL-3.0 or BSL).
