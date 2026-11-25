import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useLanguage } from "../LanguageContext";

type Props = {
  onReady: () => void;
};

type ProgressPayload = {
  downloaded: number;
  total: number;
  percent: number;
};

export function ModelDownloader({ onReady }: Props) {
  const { t } = useLanguage();
  const [progress, setProgress] = useState(0);
  const [status, setStatus] = useState<"idle" | "downloading" | "done" | "error">("idle");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    listen<ProgressPayload>("model-download-progress", (event) => {
      setProgress(event.payload.percent);
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  async function handleDownload() {
    setStatus("downloading");
    setProgress(0);
    setError(null);
    try {
      await invoke<string>("download_model");
      setStatus("done");
      setProgress(100);
      setTimeout(() => onReady(), 800);
    } catch (e: any) {
      setStatus("error");
      setError(e?.message ?? String(e));
    }
  }

  return (
    <div
    style={{
      display: "flex",
      flexDirection: "column",
      alignItems: "center",
      justifyContent: "center",
      height: "100vh",
      gap: 16,
      padding: 32,
      textAlign: "center",
    }}
    >
    <h1 style={{ margin: 0 }}>{t("modelTitle")}</h1>
    <p style={{ color: "var(--text-dim)", maxWidth: 500 }}>
    {t("modelDescription")}
    </p>

    {status === "idle" && (
      <button className="btn btn--primary" onClick={handleDownload}>
      {t("modelDownload")}
      </button>
    )}

    {status === "downloading" && (
      <div style={{ width: "100%", maxWidth: 400 }}>
      <div className="progress__bar">
      <div className="progress__fill" style={{ width: `${progress}%` }} />
      </div>
      <div style={{ fontSize: 12, color: "var(--text-dim)", marginTop: 8 }}>
      {t("modelDownloading")} {progress}%
      </div>
      </div>
    )}

    {status === "done" && (
      <div style={{ color: "var(--success)" }}>{t("modelDone")}</div>
    )}

    {error && <div className="error">⚠️ {error}</div>}
    </div>
  );
}
