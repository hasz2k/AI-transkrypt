import { useCallback, useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { useLanguage } from "../LanguageContext";

const ACCEPTED = [".wav", ".mp3", ".flac", ".ogg", ".m4a", ".mp4", ".mkv", ".webm"];

type Props = {
  file: File | null;
  onFile: (file: File) => void;
  disabled?: boolean;
};

export function DropZone({ file, onFile, disabled }: Props) {
  const { t } = useLanguage();
  const [active, setActive] = useState(false);

  // Nasłuchuj natywnych zdarzeń przeciągania Tauri
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    getCurrentWebview()
    .onDragDropEvent((event) => {
      if (disabled) return;

      if (event.payload.type === "over") {
        // Przeciąganie w obszarze okna
        setActive(true);
      } else if (event.payload.type === "drop") {
        // Upuszczenie pliku
        setActive(false);
        const paths = event.payload.paths;
        if (paths.length > 0) {
          const fullPath = paths[0];
          // Wyodrębnij nazwę pliku ze ścieżki
          const name = fullPath.split(/[\\/]/).pop() ?? fullPath;
          // Utwórz tymczasowy obiekt File do przechowywania ścieżki
          const pseudo = new File([], name, { type: "" });
          (pseudo as any).path = fullPath;
          onFile(pseudo);
        }
      } else {
        // leave lub cancelled
        setActive(false);
      }
    })
    .then((fn) => {
      unlisten = fn;
    });

    return () => {
      if (unlisten) unlisten();
    };
  }, [disabled, onFile]);

  // Wybór pliku przez dialog (już działa)
  const handlePick = useCallback(async () => {
    if (disabled) return;
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Audio / Wideo",
          extensions: ACCEPTED.map((e) => e.slice(1)),
        },
      ],
    });
    if (typeof selected === "string") {
      const name = selected.split(/[\\/]/).pop() ?? selected;
      const pseudo = new File([], name, { type: "" });
      (pseudo as any).path = selected;
      onFile(pseudo);
    }
  }, [disabled, onFile]);

  return (
    <div
    className={
      "dropzone" +
      (active ? " dropzone--active" : "") +
      (file ? " dropzone--has-file" : "")
    }
    onClick={file ? undefined : handlePick}
    >
    {file ? (
      <div className="dropzone__file">
      <div className="dropzone__file-info">
      <div className="dropzone__file-name">📄 {file.name}</div>
      <div className="dropzone__file-meta">{t("dropzoneFileSelected")}</div>
      </div>
      </div>
    ) : (
      <>
      <div className="dropzone__icon">🎙️</div>
      <div className="dropzone__text">
      <strong>{t("dropzoneTitle")}</strong>
      <br />
      {t("dropzoneOr")}
      <br />
      <span style={{ fontSize: 11 }}>{ACCEPTED.join(", ")}</span>
      </div>
      </>
    )}
    </div>
  );
}
