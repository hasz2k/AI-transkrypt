type Props = {
  text: string;
  placeholder?: string;
};

export function TranscriptionView({ text, placeholder }: Props) {
  if (!text) {
    return (
      <div className="transcription">
        <div className="transcription__placeholder">
          {placeholder ?? "Transkrypcja pojawi się tutaj…"}
        </div>
      </div>
    );
  }
  return (
    <div className="transcription">
      <textarea
        className="transcription__area"
        value={text}
        readOnly
        spellCheck={false}
      />
    </div>
  );
}
