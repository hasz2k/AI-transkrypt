type Props = {
  disabled: boolean;
  onExportTxt: () => void;
  onExportPdf: () => void;
  exportTxtLabel: string;
  exportPdfLabel: string;
};

export function ExportButtons({
  disabled,
  onExportTxt,
  onExportPdf,
  exportTxtLabel,
  exportPdfLabel,
}: Props) {
  return (
    <div className="actions">
    <button className="btn" disabled={disabled} onClick={onExportTxt}>
    {exportTxtLabel}
    </button>
    <button
    className="btn btn--primary"
    disabled={disabled}
    onClick={onExportPdf}
    >
    {exportPdfLabel}
    </button>
    </div>
  );
}
