<div align="center">

# AI Transkrypt

**Local, private desktop transcription for audio, video and system sound — no cloud, no uploads.**

[![Version](https://img.shields.io/badge/version-0.1.1--beta-orange.svg)](https://github.com/hasz2k/AI-transkrypt/releases)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20(Apple%20Silicon)-lightgrey.svg)](#-installation)
[![PayPal](https://img.shields.io/badge/PayPal-Support%20the%20project-00457C?logo=paypal&logoColor=white)](https://paypal.me/hasz2k)

[English](#-english) · [Polski](#-polski)

<!-- Add a screenshot or GIF here, e.g.: ![AI Transkrypt](docs/screenshot.png) -->

</div>

---

## 🇬🇧 English

### Table of contents

- [Why AI Transkrypt?](#why-ai-transkrypt)
- [Features](#-features)
- [Installation](#-installation)
- [Usage](#-usage)
- [Building from source](#%EF%B8%8F-building-from-source)
- [Tech stack](#-tech-stack)
- [Roadmap](#%EF%B8%8F-roadmap)
- [Reporting bugs](#-reporting-bugs)
- [Support the project](#-support-the-project)
- [License](#-license)

### Why AI Transkrypt?

Most transcription tools send your recordings to someone else's server. AI Transkrypt runs **whisper.cpp** directly on your machine, so interviews, meetings and lectures stay on your computer. It uses **Vulkan** on Linux and **Metal** on macOS for native GPU acceleration — no CUDA, Python or PyTorch required.

### ✨ Features

| | |
|---|---|
| 🔒 **100% local** | Audio files and transcripts never leave your machine. |
| ⚡ **Native GPU acceleration** | Fast inference via **Vulkan** on Linux (AMD/Intel; NVIDIA should work via Vulkan but is **not tested yet**) and **Metal** on macOS (Apple Silicon). |
| 🎧 **System audio capture** | Record desktop audio (online meetings, browser, lectures) with one click via PipeWire `pw-record` (Linux) or native `ScreenCaptureKit` (macOS 13.0+, no virtual audio drivers needed). |
| 🎞️ **Wide format support** | `.wav` `.mp3` `.flac` `.ogg` `.m4a` `.mp4` `.mkv` `.webm`, decoded and resampled to 16 kHz mono in pure Rust. |
| 📄 **TXT & PDF export** | Plain text or a formatted PDF with full Unicode / Polish diacritics support. |
| 🌍 **Bilingual UI** | Switch the interface between Polish and English. Recognition language: Polish, English or Auto. |
| 🛑 **Task cancellation** | Stop a running transcription at any time. |
| 🧪 **Opt-in diagnostics** | Copy an environment report (no recording content) to the clipboard and open a GitHub issue in one click. |

### 📦 Installation

> Currently available for **Linux** and **macOS (Apple Silicon)**. Windows builds are planned for v0.2.0.

#### 🐧 Linux

**Requirements:**
- **PipeWire** with `pw-record` (package `pipewire-utils` or equivalent), required for system audio capture
- **Vulkan drivers**, e.g. `mesa-vulkan-drivers` for AMD/Intel

> ⚠️ **NVIDIA GPUs have not been tested yet.** If you run AI Transkrypt on an NVIDIA card, please share your results (working or not) in [GitHub Issues](https://github.com/hasz2k/AI-transkrypt/issues), ideally with the built-in diagnostics report.

Download the latest `.AppImage` or `.deb` from **[Releases](https://github.com/hasz2k/AI-transkrypt/releases)**.

**AppImage:**

```bash
chmod +x ai-transkrypt_0.1.1_amd64.AppImage
./ai-transkrypt_0.1.1_amd64.AppImage
```

**Debian / Ubuntu:**

```bash
sudo apt install ./ai-transkrypt_0.1.1_amd64.deb
```

#### 🍎 macOS (Apple Silicon)

**Requirements:**
- **macOS 13.0 (Ventura)** or later on **Apple Silicon (`arm64` — M1 / M2 / M3 / M4)**

Download the latest `ai-transkrypt_*_aarch64.dmg` from **[Releases](https://github.com/hasz2k/AI-transkrypt/releases)**, drag **ai-transkrypt.app** to `/Applications`, and run:

```bash
xattr -cr /Applications/ai-transkrypt.app
```

> ℹ️ Because the release binary is ad-hoc signed, removing the Gatekeeper quarantine attribute (`xattr -cr`) is required before the first launch. On first run, grant **Screen & System Audio Recording** permission in *System Settings → Privacy & Security*.

On first launch the app downloads a local Whisper model into your user data directory (`models/` subfolder). This happens once and needs an internet connection. After that, transcription runs fully offline.

### 🎙️ Usage

1. Launch the app and wait for the model download to finish (first run only).
2. Choose the recognition language: **Polish**, **English** or **Auto**.
3. Either **open a file** (audio or video) or **record system audio** with one click.
4. Start the transcription. You can cancel it at any time.
5. Export the result to **`.txt`** or **`.pdf`**.

### 🛠️ Building from source

#### Development requirements

- **Node.js ≥ 20** and **pnpm** (or **npm**)
- **Rust** (stable toolchain via [rustup](https://rustup.rs))
- **Linux dependencies:** `webkit2gtk-4.1`, `gtk3`, `libayatana-appindicator`, `librsvg`, `cmake`, `clang`, `shaderc` / Vulkan headers
- **macOS dependencies:** Full **Xcode** installation (`xcode-select` pointing to `/Applications/Xcode.app/Contents/Developer`) and `cmake`

#### Run in development mode

```bash
git clone https://github.com/hasz2k/AI-transkrypt.git
cd AI-transkrypt
pnpm install
pnpm tauri dev
```

#### Production build

```bash
pnpm tauri build
```

The packages are written to `src-tauri/target/release/bundle/`.

### 🧰 Tech stack

| Layer | Technology |
|---|---|
| Desktop shell | [Tauri v2](https://tauri.app) |
| Frontend | React 19 |
| Backend | Rust |
| Speech recognition | [whisper.cpp](https://github.com/ggerganov/whisper.cpp) + Vulkan (Linux) / Metal (macOS) |
| Audio capture | PipeWire `pw-record` (Linux) / `ScreenCaptureKit` (macOS) |

### 🗺️ Roadmap

AI Transkrypt follows an **Open Core** model: the core desktop app stays free and open source under AGPL-3.0.

- [x] **Phase 1 — v0.1.x-beta: Core MVP & stabilization** *(current)*
  - [x] Local transcription of audio/video files (whisper.cpp + Vulkan / Metal)
  - [x] System audio capture via PipeWire (Linux) and ScreenCaptureKit (macOS)
  - [x] macOS Apple Silicon (`.app` / `.dmg`) port
  - [x] TXT and PDF export
  - [x] Task cancellation
  - [x] Opt-in diagnostics
- [ ] **Phase 2 — v0.2.0: Open source expansion**
  - [ ] Automatic UI language detection based on OS locale (`pl` / `en` fallback)
  - [ ] In-app Whisper model manager (`base`, `small`, `medium`, `large-v3-turbo`)
  - [ ] Timestamps and `.srt` / `.vtt` subtitle export
  - [ ] Audio source selector (microphone vs. a specific app/window)
  - [ ] Official Windows builds
- [ ] **Phase 3 — Pro edition**
  - [ ] Speaker diarization
  - [ ] Meeting summaries, action items and translation with local LLMs (e.g. Ollama)
  - [ ] Batch queue processing

### 🐛 Reporting bugs

Please open an issue on **[GitHub Issues](https://github.com/hasz2k/AI-transkrypt/issues)**. Use the built-in **diagnostics** button to copy your environment report (OS, kernel, desktop session, GPU, audio server, model status) and paste it into the issue. The report never includes your recordings or transcripts.

### ☕ Support the project

If AI Transkrypt saves you time and you'd like to support its open-source development (including the upcoming Windows port):

👉 **[Donate via PayPal](https://paypal.me/hasz2k)**

### 📄 License

The open-source edition is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**. See [LICENSE](LICENSE) for details.

---

## 🇵🇱 Polski

### Spis treści

- [Dlaczego AI Transkrypt?](#dlaczego-ai-transkrypt)
- [Funkcje](#-funkcje)
- [Instalacja](#-instalacja)
- [Jak używać](#%EF%B8%8F-jak-używać)
- [Budowanie ze źródeł](#%EF%B8%8F-budowanie-ze-źródeł)
- [Technologie](#-technologie)
- [Plan rozwoju](#%EF%B8%8F-plan-rozwoju)
- [Zgłaszanie błędów](#-zgłaszanie-błędów)
- [Wsparcie projektu](#-wsparcie-projektu)
- [Licencja](#-licencja)

### Dlaczego AI Transkrypt?

Większość narzędzi do transkrypcji wysyła nagrania na cudzy serwer. AI Transkrypt uruchamia **whisper.cpp** bezpośrednio na Twoim komputerze, więc wywiady, spotkania i wykłady nigdzie nie wychodzą. Akceleracja GPU działa natywnie przez **Vulkan** (Linux) i **Metal** (macOS), więc nie potrzebujesz CUDA, Pythona ani PyTorcha.

### ✨ Funkcje

| | |
|---|---|
| 🔒 **100% lokalnie** | Nagrania i wygenerowany tekst nigdy nie opuszczają Twojego komputera. |
| ⚡ **Natywna akceleracja GPU** | Szybka transkrypcja przez **Vulkan** na Linuxie (AMD i Intel; NVIDIA powinna działać przez Vulkan, ale **nie była jeszcze testowana**) oraz **Metal** na macOS (Apple Silicon). |
| 🎧 **Nagrywanie dźwięku systemowego** | Nagrywanie prosto z wyjścia audio (spotkania online, przeglądarka, wykłady) jednym kliknięciem przez PipeWire `pw-record` (Linux) lub natywne `ScreenCaptureKit` (macOS 13.0+, bez wirtualnych sterowników audio). |
| 🎞️ **Wiele formatów** | `.wav` `.mp3` `.flac` `.ogg` `.m4a` `.mp4` `.mkv` `.webm`, dekodowanie i resampling do 16 kHz mono w czystym Ruście. |
| 📄 **Eksport TXT i PDF** | Czysty tekst albo sformatowany PDF z pełną obsługą polskich znaków. |
| 🌍 **Dwujęzyczny interfejs** | Przełączanie interfejsu między polskim i angielskim. Język rozpoznawania: polski, angielski lub automatyczny. |
| 🛑 **Anulowanie zadań** | Trwającą transkrypcję możesz przerwać w dowolnym momencie. |
| 🧪 **Diagnostyka (opt-in)** | Kopiowanie raportu środowiska (bez treści nagrań) do schowka i zgłaszanie błędu na GitHubie jednym kliknięciem. |

### 📦 Instalacja

> Obecnie dostępne dla systemów **Linux** oraz **macOS (Apple Silicon)**. Wersja na Windows jest planowana w v0.2.0.

#### 🐧 Linux

**Wymagania:**
- **PipeWire** z narzędziem `pw-record` (pakiet `pipewire-utils` lub odpowiednik), potrzebne do nagrywania dźwięku systemowego
- **Sterowniki Vulkan**, np. `mesa-vulkan-drivers` dla AMD/Intel

> ⚠️ **Karty NVIDIA nie były jeszcze testowane.** Jeśli uruchamiasz AI Transkrypt na karcie NVIDIA, podziel się wynikiem (działa czy nie) w [GitHub Issues](https://github.com/hasz2k/AI-transkrypt/issues), najlepiej z dołączonym raportem diagnostycznym.

Pobierz najnowszy plik `.AppImage` lub `.deb` z zakładki **[Releases](https://github.com/hasz2k/AI-transkrypt/releases)**.

**AppImage:**

```bash
chmod +x ai-transkrypt_0.1.1_amd64.AppImage
./ai-transkrypt_0.1.1_amd64.AppImage
```

**Debian / Ubuntu:**

```bash
sudo apt install ./ai-transkrypt_0.1.1_amd64.deb
```

#### 🍎 macOS (Apple Silicon)

**Wymagania:**
- **macOS 13.0 (Ventura)** lub nowszy na **Apple Silicon (`arm64` — M1 / M2 / M3 / M4)**

Pobierz najnowszy plik `ai-transkrypt_*_aarch64.dmg` z zakładki **[Releases](https://github.com/hasz2k/AI-transkrypt/releases)**, przeciągnij **ai-transkrypt.app** do `/Applications`, a następnie usuń flagę kwarantanny Gatekeepera w Terminalu:

```bash
xattr -cr /Applications/ai-transkrypt.app
```

> ℹ️ Ponieważ paczka nie jest podpisana płatnym certyfikatem Apple Developer ID, przed pierwszym uruchomieniem należy zdjąć atrybut kwarantanny (`xattr -cr`). Przy pierwszym starcie zezwól na dostęp w *Ustawienia systemowe → Prywatność i ochrona → Nagrywanie ekranu i dźwięku systemowego*.

Przy pierwszym uruchomieniu aplikacja pobiera lokalny model Whisper do katalogu danych użytkownika (podfolder `models/`). Dzieje się to tylko raz i wymaga internetu. Potem transkrypcja działa w pełni offline.

### 🎙️ Jak używać

1. Uruchom aplikację i poczekaj na pobranie modelu (tylko za pierwszym razem).
2. Wybierz język rozpoznawania: **polski**, **angielski** lub **automatyczny**.
3. **Otwórz plik** (audio lub wideo) albo **nagraj dźwięk systemowy** jednym kliknięciem.
4. Uruchom transkrypcję. W każdej chwili możesz ją anulować.
5. Wyeksportuj wynik do **`.txt`** lub **`.pdf`**.

### 🛠️ Budowanie ze źródeł

#### Wymagania deweloperskie

- **Node.js ≥ 20** oraz **pnpm** (lub **npm**)
- **Rust** (stabilny toolchain przez [rustup](https://rustup.rs))
- **Zależności systemowe Linux:** `webkit2gtk-4.1`, `gtk3`, `libayatana-appindicator`, `librsvg`, `cmake`, `clang`, `shaderc` / nagłówki Vulkan
- **Zależności systemowe macOS:** pełna instalacja **Xcode** (`xcode-select` wskazujący na `/Applications/Xcode.app/Contents/Developer`) oraz `cmake`

#### Uruchomienie w trybie deweloperskim

```bash
git clone https://github.com/hasz2k/AI-transkrypt.git
cd AI-transkrypt
pnpm install
pnpm tauri dev
```

#### Budowanie paczki produkcyjnej

```bash
pnpm tauri build
```

Gotowe paczki trafiają do `src-tauri/target/release/bundle/`.

### 🧰 Technologie

| Warstwa | Technologia |
|---|---|
| Aplikacja desktopowa | [Tauri v2](https://tauri.app) |
| Frontend | React 19 |
| Backend | Rust |
| Rozpoznawanie mowy | [whisper.cpp](https://github.com/ggerganov/whisper.cpp) + Vulkan (Linux) / Metal (macOS) |
| Przechwytywanie dźwięku | PipeWire `pw-record` (Linux) / `ScreenCaptureKit` (macOS) |

### 🗺️ Plan rozwoju

Projekt rozwijany jest w modelu **Open Core**: podstawowa wersja desktopowa pozostaje darmowa i otwarta na licencji AGPL-3.0.

- [x] **Faza 1 — v0.1.x-beta: fundamenty i stabilizacja** *(aktualna)*
  - [x] Lokalna transkrypcja plików audio/wideo (whisper.cpp + Vulkan / Metal)
  - [x] Nagrywanie dźwięku systemowego przez PipeWire (Linux) oraz ScreenCaptureKit (macOS)
  - [x] Port na macOS Apple Silicon (`.app` / `.dmg`)
  - [x] Eksport do TXT i PDF
  - [x] Anulowanie zadań w trakcie
  - [x] Raporty diagnostyczne (opt-in)
- [ ] **Faza 2 — v0.2.0: rozszerzenie wersji Open Source**
  - [ ] Automatyczne wykrywanie języka interfejsu na podstawie ustawień systemu (`pl` / `en`)
  - [ ] Wybór modelu Whisper w interfejsie (`base`, `small`, `medium`, `large-v3-turbo`)
  - [ ] Znaczniki czasu i eksport napisów `.srt` / `.vtt`
  - [ ] Wybór źródła dźwięku (mikrofon albo konkretna aplikacja/okno)
  - [ ] Oficjalne wydania na Windows
- [ ] **Faza 3 — wersja Pro**
  - [ ] Rozpoznawanie mówców (diaryzacja)
  - [ ] Podsumowania spotkań, listy zadań i tłumaczenia z lokalnymi modelami LLM (np. Ollama)
  - [ ] Przetwarzanie wsadowe (kolejka wielu plików)

### 🐛 Zgłaszanie błędów

Zgłoś problem w **[GitHub Issues](https://github.com/hasz2k/AI-transkrypt/issues)**. Użyj wbudowanego przycisku **diagnostyki**, żeby skopiować raport środowiska (system, jądro, sesja graficzna, GPU, serwer audio, stan modelu), i wklej go do zgłoszenia. Raport nigdy nie zawiera nagrań ani transkrypcji.

### ☕ Wsparcie projektu

Jeśli aplikacja oszczędza Ci czas i chcesz wesprzeć dalszy rozwój wersji Open Source (m.in. nadchodzący port na Windows):

👉 **[Wesprzyj przez PayPal](https://paypal.me/hasz2k)**

### 📄 Licencja

Kod źródłowy wersji Open Source udostępniany jest na licencji **GNU Affero General Public License v3.0 (AGPL-3.0)**. Szczegóły w pliku [LICENSE](LICENSE).
