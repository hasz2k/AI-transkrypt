import { getVersion } from "@tauri-apps/api/app";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useEffect, useState } from "react";
import { getDiagnosticLogs } from "../lib/tauri";

interface TesterToolsProps {
  lang: "pl" | "en";
}

async function copyTextSafe(text: string): Promise<void> {
  try {
    await navigator.clipboard.writeText(text);
    return;
  } catch {
    // Fallback dla WebKitGTK / Wayland po wywołaniu async
  }

  const textarea = document.createElement("textarea");
  textarea.value = text;
  textarea.style.position = "fixed";
  textarea.style.opacity = "0";
  document.body.appendChild(textarea);
  textarea.focus();
  textarea.select();
  const ok = document.execCommand("copy");
  document.body.removeChild(textarea);

  if (!ok) {
    throw new Error("Nie udało się zapisać do schowka");
  }
}

export function TesterTools({ lang }: TesterToolsProps) {
  const [version, setVersion] = useState("0.1.1");
  const [status, setStatus] = useState<"idle" | "copied" | "error">("idle");

  useEffect(() => {
    getVersion().then(setVersion).catch(() => {});
  }, []);

  async function handleCopyLogs() {
    try {
      const report = await getDiagnosticLogs();
      await copyTextSafe(report);
      setStatus("copied");
      setTimeout(() => setStatus("idle"), 2500);
    } catch (e) {
      console.error("Błąd kopiowania logów:", e);
      setStatus("error");
      setTimeout(() => setStatus("idle"), 3000);
    }
  }

  function handleOpenFeedback() {
    openUrl("https://github.com/hasz2k/AI-transkrypt/issues/new/choose");
  }

  function handleOpenSupport() {
    openUrl("https://paypal.me/hasz2k");
  }

  const label =
  status === "copied"
  ? lang === "pl"
  ? "✓ Skopiowano logi"
  : "✓ Logs copied"
  : status === "error"
  ? lang === "pl"
  ? "⚠️ Błąd logów"
  : "⚠️ Log error"
  : lang === "pl"
  ? "📋 Kopiuj logi"
  : "📋 Copy logs";

  return (
    <div className="tester-bar">
    <span className="version-badge">v{version}-beta</span>
    <button
    type="button"
    className="btn btn--sm"
    onClick={handleCopyLogs}
    title={
      lang === "pl"
      ? "Kopiuje dane diagnostyczne (bez treści nagrań) do schowka"
      : "Copies diagnostic data (no audio/transcript content) to clipboard"
    }
    >
    {label}
    </button>
    <button
    type="button"
    className="btn btn--sm"
    onClick={handleOpenFeedback}
    >
    {lang === "pl" ? "🐞 Zgłoś błąd" : "🐞 Feedback"}
    </button>
    <button
    type="button"
    className="btn btn--sm"
    onClick={handleOpenSupport}
    title="https://paypal.me/hasz2k"
    >
    {lang === "pl" ? "☕ Wesprzyj" : "☕ Support"}
    </button>
    </div>
  );
}
