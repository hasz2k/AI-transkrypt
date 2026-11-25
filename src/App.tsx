import { useEffect, useState } from "react";
import "./App.css";
import { BetaBanner } from "./components/BetaBanner";
import { DropZone } from "./components/DropZone";
import { ProgressBar } from "./components/ProgressBar";
import { TranscriptionView } from "./components/TranscriptionView";
import { ExportButtons } from "./components/ExportButtons";
import { ModelDownloader } from "./components/ModelDownloader";
import { transcribeFile, exportTxt, exportPdf } from "./lib/tauri";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useLanguage } from "./LanguageContext";

type Status = "idle" | "transcribing" | "done" | "error";

type ProgressPayload = { percent: number };

export default function App() {
  const { t, lang, setLang } = useLanguage();
  const [modelReady, setModelReady] = useState<boolean | null>(null);
  const [file, setFile] = useState<File | null>(null);
  const [language, setLanguage] = useState<string>("pl");
  const [status, setStatus] = useState<Status>("idle");
  const [progress, setProgress] = useState(0);
  const [text, setText] = useState("");
  const [error, setError] = useState<string | null>(null);

  const isBusy = status === "transcribing";

  // Sprawdź dostępność modelu przy starcie
  useEffect(() => {
    invoke<boolean>("check_model")
    .then(setModelReady)
    .catch(() => setModelReady(false));
  }, []);

  // Nasłuchuj progress z backendu
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    listen<ProgressPayload>("transcription-progress", (event) => {
      setProgress(event.payload.percent);
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  if (modelReady === null) {
    return <div style={{ padding: 32 }}>{t("modelChecking")}</div>;
  }

  if (!modelReady) {
    return <ModelDownloader onReady={() => setModelReady(true)} />;
  }

  async function handleTranscribe() {
    if (!file) return;
    setStatus("transcribing");
    setProgress(0);
    setText("");
    setError(null);

    try {
      const path = (file as any).path ?? file.name;
      const result = await transcribeFile(path, language);
      setProgress(100);
      setText(result);
      setStatus("done");
    } catch (e: any) {
      setStatus("error");
      setError(e?.message ?? String(e));
    }
  }

  async function handleExportTxt() {
    try {
      const baseName = file?.name.replace(/\.[^.]+$/, "") || "transkrypcja";
      const saved = await exportTxt(text, `${baseName}.txt`);
      console.log("Zapisano:", saved);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  async function handleExportPdf() {
    try {
      const baseName = file?.name.replace(/\.[^.]+$/, "") || "transkrypcja";
      const saved = await exportPdf(text, `${baseName}.pdf`);
      console.log("Zapisano:", saved);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  return (
    <div className="app">
    <header className="app__header">
    <div>
    <h1 className="app__title">{t("appTitle")}</h1>
    <div className="app__subtitle">{t("appSubtitle")}</div>
    </div>
    <select
    className="btn"
    value={lang}
    onChange={(e) => setLang(e.target.value as "pl" | "en")}
    >
    <option value="pl">🇵🇱 PL</option>
    <option value="en">🇬🇧 EN</option>
    </select>
    </header>

    <BetaBanner />

    <main className="app__main">
    <DropZone
    file={file}
    onFile={(f) => {
      setFile(f);
      setText("");
      setStatus("idle");
      setError(null);
      setProgress(0);
    }}
    disabled={isBusy}
    />

    <div className="actions">
    <select
    className="btn"
    value={language}
    onChange={(e) => setLanguage(e.target.value)}
    disabled={isBusy}
    >
    <option value="pl">{t("langPolish")}</option>
    <option value="en">{t("langEnglish")}</option>
    <option value="">{t("langAuto")}</option>
    </select>
    <button
    className="btn btn--primary"
    disabled={!file || isBusy}
    onClick={handleTranscribe}
    >
    {t(isBusy ? "transcribing" : "transcribe")}
    </button>
    {file && !isBusy && (
      <button
      className="btn btn--danger"
      onClick={() => {
        setFile(null);
        setText("");
        setStatus("idle");
        setProgress(0);
        setError(null);
      }}
      >
      {t("clear")}
      </button>
    )}
    </div>

    {(isBusy || status === "done") && (
      <ProgressBar
      progress={progress}
      label={isBusy ? t("progressLabel") : t("progressDone")}
      />
    )}

    {error && <div className="error">⚠️ {error}</div>}

    <TranscriptionView text={text} placeholder={t("transcriptionPlaceholder")} />

    <ExportButtons
    disabled={!text}
    onExportTxt={handleExportTxt}
    onExportPdf={handleExportPdf}
    exportTxtLabel={t("exportTxt")}
    exportPdfLabel={t("exportPdf")}
    />
    </main>
    </div>
  );
}
