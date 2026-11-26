# AI Transkrypt (`v0.1.1-beta`)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](LICENSE)
[![PayPal](https://img.shields.io/badge/PayPal-Wesprzyj%20projekt-00457C?logo=paypal&logoColor=white)](https://paypal.me/hasz2k)

**Lokalna, prywatna aplikacja desktopowa do transkrypcji audio, wideo oraz dźwięku systemowego — bez chmury i bez wysyłania danych na zewnątrz.**

Zbudowana w oparciu o **Tauri v2**, **React 19**, **Rust** oraz **whisper.cpp** z akceleracją sprzętową **Vulkan** i natywnym przechwytywaniem dźwięku w **PipeWire**.

> 🇬🇧 **[English documentation below](#-english-version)**

---

## ✨ Funkcje (`v0.1.1-beta`)

- **100% lokalne przetwarzanie:** nagrania i wygenerowany tekst nigdy nie opuszczają Twojego komputera.
- **Akceleracja GPU (Vulkan):** szybka transkrypcja na kartach AMD, Intel oraz NVIDIA bez konieczności instalowania ciężkich środowisk CUDA/PyTorch.
- **Przechwytywanie dźwięku systemowego (Linux / PipeWire):** nagrywanie bezpośrednio z wyjścia audio (np. spotkania online, przeglądarka, wykłady) jednym kliknięciem poprzez `pw-record`.
- **Szeroka obsługa formatów:** `.wav`, `.mp3`, `.flac`, `.ogg`, `.m4a`, `.mp4`, `.mkv`, `.webm` (dekodowanie i resampling do 16 kHz mono w czystym Ruście).
- **Eksport dokumentów:** zapis transkrypcji do czystego tekstu (`.txt`) oraz sformatowanego dokumentu **PDF** z pełną obsługą polskich znaków diakrytycznych.
- **Dwujęzyczny interfejs (PL / EN):** natychmiastowe przełączanie języka aplikacji oraz wybór języka rozpoznawania mowy (Polski, Angielski, Auto).
- **Narzędzia dla testerów (opt-in):** kopia raportu diagnostycznego środowiska do schowka (bez treści nagrań) oraz bezpośredni skrót do zgłaszania błędów.

---

## 🚀 Instalacja i uruchamianie (Linux)

### Gotowe paczki (Releases)
Pobierz najnowszą paczkę `.AppImage` lub `.deb` z zakładki **[Releases](https://github.com/hasz2k/AI-transkrypt/releases)**:

```bash
chmod +x ai-transkrypt_0.1.1_amd64.AppImage
./ai-transkrypt_0.1.1_amd64.AppImage

Przy pierwszym uruchomieniu aplikacja automatycznie pobierze lokalny model Whisper do katalogu danych użytkownika.
Wymagania systemowe (Linux)

    Serwer dźwięku PipeWire (pipewire-utils z narzędziem pw-record do nagrywania dźwięku systemowego)

    Sterowniki Vulkan (np. mesa-vulkan-drivers dla AMD/Intel)

🛠️ Budowanie ze źródeł
Wymagania deweloperskie

    Node.js >= 20 oraz pnpm

    Rust (stable toolchain via rustup)

    Zależności systemowe Tauri v2 (webkit2gtk-4.1, gtk3, libayatana-appindicator, librsvg, cmake, clang, shaderc / nagłówki Vulkan)

Bash

git clone [https://github.com/hasz2k/AI-transkrypt.git](https://github.com/hasz2k/AI-transkrypt.git)
cd AI-transkrypt
pnpm install
pnpm tauri dev

Budowanie paczki produkcyjnej:
Bash

pnpm tauri build

🗺️ Plan rozwoju (Roadmap)

Projekt rozwijany jest w modelu Open Core — podstawowa wersja desktopowa dla systemu Linux pozostaje darmowa i otwarta na licencji AGPL-3.0.

    Faza 1 (v0.1.x-beta) — Fundamenty i stabilizacja [Aktualna]:

        Lokalna transkrypcja plików audio/wideo (whisper.cpp + Vulkan).

        Nagrywanie dźwięku systemowego w PipeWire (pw-record).

        Eksport do .txt i .pdf, anulowanie zadań w locie, raporty diagnostyczne opt-in.

    Faza 2 (v0.2.0) — Rozszerzenie funkcji Open Source:

        Wybór rozmiaru i wariantu modelu Whisper (base, small, medium, large-v3-turbo) bezpośrednio w interfejsie.

        Znaczniki czasu (timestamps) oraz eksport napisów w formatach .srt i .vtt.

        Wybór konkretnego źródła audio (mikrofon vs. wybrana aplikacja/okno w PipeWire).

        Oficjalne wydania dla macOS (Apple Silicon / Intel) oraz Windows.

    Faza 3 — Wersja Pro (funkcje zaawansowane):

        Rozpoznawanie mówców (diaryzacja — podział wypowiedzi na osoby).

        Automatyczne podsumowania spotkań, listy zadań (action items) i tłumaczenia z lokalnymi modelami LLM (np. Ollama).

        Przetwarzanie wsadowe (kolejkowanie wielu plików jednocześnie).

☕ Wsparcie projektu

Jeśli aplikacja oszczędza Twój czas i chcesz wesprzeć dalszy rozwój wersji Open Source (m.in. porty na macOS i Windows):

👉 Wesprzyj przez PayPal (paypal.me/hasz2k)
📄 Licencja

Kod źródłowy wydania Open Source udostępniany jest na licencji GNU Affero General Public License v3.0 (AGPL-3.0) — szczegóły w pliku LICENSE.
🇬🇧 English Version

AI Transkrypt (v0.1.1-beta) is a local-first desktop application for audio, video, and system audio transcription — running 100% offline with zero cloud dependencies.

Built with Tauri v2, React 19, Rust, and whisper.cpp with Vulkan GPU acceleration and native PipeWire audio capture.
✨ Key Features (v0.1.1-beta)

    100% Local & Private: Audio files and transcripts never leave your machine.

    Vulkan GPU Acceleration: Fast hardware-accelerated inference on AMD, Intel, and NVIDIA GPUs without heavy CUDA/Python dependencies.

    System Audio Capture (Linux / PipeWire): Record desktop audio directly via pw-record with a single click.

    Multi-format Support: .wav, .mp3, .flac, .ogg, .m4a, .mp4, .mkv, .webm decoded and resampled to 16 kHz mono natively in Rust.

    TXT & PDF Export: Export clean text or formatted PDF documents with full Unicode/diacritic support.

    Bilingual UI (PL / EN): Switch between Polish and English interface and transcription languages (pl, en, auto).

    Opt-in Tester Diagnostics: One-click copy of anonymized hardware/environment info (OS, kernel, Wayland/X11, GPU, PipeWire server, model status) and direct GitHub Issue reporting.

🗺️ Roadmap

    Phase 1 (v0.1.x-beta) — Core MVP & Stabilization [Current]: Local transcription (whisper.cpp + Vulkan), PipeWire system capture, TXT/PDF export, task cancellation, opt-in diagnostics.

    Phase 2 (v0.2.0) — Open Source Expansion: In-app Whisper model manager (base, small, medium, large-v3-turbo), timestamps & .srt/.vtt subtitle export, per-app/mic audio source selector, official macOS and Windows builds.

    Phase 3 — Pro Edition: Speaker diarization, local LLM meeting summaries & action items (Ollama integration), and batch queue processing.

☕ Support the Project

If you find AI Transkrypt useful and want to support its ongoing open-source development:

👉 Donate via PayPal (paypal.me/hasz2k)
📄 License

Licensed under AGPL-3.0. See LICENSE for details.
