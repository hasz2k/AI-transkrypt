type Props = {
  progress: number;
  label?: string;
};

export function ProgressBar({ progress, label }: Props) {
  const clamped = Math.max(0, Math.min(100, progress));
  return (
    <div className="progress">
      <div className="progress__bar">
        <div className="progress__fill" style={{ width: `${clamped}%` }} />
      </div>
      <div className="progress__label">
        <span>{label ?? "Postęp"}</span>
        <span>{clamped.toFixed(0)}%</span>
      </div>
    </div>
  );
}
