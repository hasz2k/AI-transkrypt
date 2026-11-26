import { useEffect, useRef, useState } from "react";
import "./App.css";
import { BetaBanner } from "./components/BetaBanner";
import { DropZone } from "./components/DropZone";
import { ProgressBar } from "./components/ProgressBar";
import { TranscriptionView } from "./components/TranscriptionView";
import { ExportButtons } from "./components/ExportButtons";
import { ModelDownloader } from "./components/ModelDownloader";
import { TesterTools } from "./components/TesterTools";
import {
  transcribeFile,
  cancelTranscription,
  exportTxt,
  exportPdf,
  checkModel,
  startSystemCapture,
  stopSystemCapture,
} from "./lib/tauri";
import { listen } from "@tauri-apps/api/event";
import { useLanguage } from "./LanguageContext";

type Status = "idle" | "transcribing" | "done" | "error" | "cancelled";
type ProgressPayload = { percent: number };

export default function App() {
  const { t, lang, setLang } = useLanguage();

  const [file, setFile] = useState<File | null>(null);
  const [status, setStatus] = useState<Status>("idle");
  const [progress, setProgress] = useState(0);
  const [text, setText] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [language, setLanguage] = useState<string>("pl");
  const [isRecording, setIsRecording] = useState(false);

  const [modelReady, setModelReady] = useState<boolean | null>(null);

  const currentTaskId = useRef<number>(0);

  const isBusy = status === "transcribing";

  useEffect(() => {
    checkModel()
      .then(setModelReady)
      .catch(() => setModelReady(false));
  }, []);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    listen<ProgressPayload>("transcription-progress", (event) => {
      if (isBusy) setProgress(event.payload.percent);
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      if (unlisten) unlisten();
    };
  }, [isBusy]);

  async function handleStartCapture() {
    try {
      setError(null);
      await startSystemCapture();
      setIsRecording(true);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  async function handleStopCapture() {
    try {
      const wavPath = await stopSystemCapture();
      setIsRecording(false);

      if (wavPath) {
        const fileName = wavPath.split(/[/\\]/).pop() || "recording.wav";
        const recordedFile = new File([], fileName, { type: "audio/wav" });
        Object.defineProperty(recordedFile, "path", { value: wavPath });

        setFile(recordedFile);
        setText("");
        setStatus("idle");
        setProgress(0);
        setError(null);
        currentTaskId.current++;
      }
    } catch (e: any) {
      setIsRecording(false);
      setError(e?.message ?? String(e));
    }
  }

  async function handleTranscribe() {
    if (!file) return;

    const taskId = ++currentTaskId.current;

    setStatus("transcribing");
    setProgress(0);
    setText("");
    setError(null);

    try {
      const path = (file as any).path ?? file.name;
      const result = await transcribeFile(path, language);

      if (currentTaskId.current !== taskId) {
        return;
      }

      setProgress(100);
      setText(result);
      setStatus("done");
    } catch (e: any) {
      if (currentTaskId.current !== taskId) return;
      setStatus("error");
      setError(e?.message ?? String(e));
    }
  }

  async function handleCancel() {
    currentTaskId.current++;
    setStatus("cancelled");
    setProgress(0);
    try {
      await cancelTranscription();
    } catch (e) {
      console.error("Błąd anulowania:", e);
    }
  }

  async function handleExportTxt() {
    try {
      const baseName = file?.name.replace(/\.[^.]+$/, "") || "transcription";
      await exportTxt(text, `${baseName}.txt`);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  async function handleExportPdf() {
    try {
      const baseName = file?.name.replace(/\.[^.]+$/, "") || "transcription";
      await exportPdf(text, `${baseName}.pdf`);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  if (modelReady === null) {
    return <div style={{ padding: 32 }}>{t("modelChecking")}</div>;
  }

  if (!modelReady) {
    return <ModelDownloader onReady={() => setModelReady(true)} />;
  }

  return (
    <div className="app">
      <header className="app__header">
        <div>
          <h1 className="app__title">{t("appTitle")}</h1>
          <div className="app__subtitle">{t("appSubtitle")}</div>
        </div>
        <div className="app__header-actions">
          <TesterTools lang={lang} />
          <select
            className="btn"
            value={lang}
            onChange={(e) => setLang(e.target.value as "pl" | "en")}
          >
            <option value="pl">🇵🇱 PL</option>
            <option value="en">🇬🇧 EN</option>
          </select>
        </div>
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
            currentTaskId.current++;
          }}
          disabled={isBusy || isRecording}
        />

        <div className="actions">
          {!isRecording ? (
            <button
              className="btn"
              disabled={isBusy}
              onClick={handleStartCapture}
            >
              {lang === "pl" ? "🎙️ Nagraj dźwięk" : "🎙️ Record audio"}
            </button>
          ) : (
            <button
              className="btn btn--danger"
              onClick={handleStopCapture}
            >
              {lang === "pl" ? "⏹️ Zatrzymaj nagrywanie" : "⏹️ Stop recording"}
            </button>
          )}

          <select
            className="btn"
            value={language}
            onChange={(e) => setLanguage(e.target.value)}
            disabled={isBusy || isRecording}
          >
            <option value="pl">{t("langPolish")}</option>
            <option value="en">{t("langEnglish")}</option>
            <option value="">{t("langAuto")}</option>
          </select>

          {!isBusy ? (
            <button
              className="btn btn--primary"
              disabled={!file || isRecording}
              onClick={handleTranscribe}
            >
              {t("transcribe")}
            </button>
          ) : (
            <button
              className="btn btn--danger"
              onClick={handleCancel}
            >
              {t("cancel")}
            </button>
          )}

          {file && !isBusy && !isRecording && (
            <button
              className="btn btn--danger"
              onClick={() => {
                setFile(null);
                setText("");
                setStatus("idle");
                setProgress(0);
                setError(null);
                currentTaskId.current++;
              }}
            >
              {t("clear")}
            </button>
          )}
        </div>

        {isBusy && (
          <ProgressBar
            progress={progress}
            label={t("progressLabel")}
          />
        )}

        {status === "cancelled" && (
          <div
            className="error"
            style={{
              color: "var(--text-dim)",
              background: "transparent",
              borderColor: "var(--border)",
            }}
          >
            {t("cancelled")}
          </div>
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