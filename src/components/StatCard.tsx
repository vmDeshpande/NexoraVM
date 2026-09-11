type StatTone = "neutral" | "info" | "success" | "warning" | "error";

interface StatCardProps {
  detail: string;
  label: string;
  tone?: StatTone;
  value: string;
}

export function StatCard({
  detail,
  label,
  tone = "neutral",
  value,
}: StatCardProps) {
  return (
    <article className={`stat-card stat-card--${tone}`}>
      <div className="stat-card__topline">
        <span className="stat-card__indicator" aria-hidden="true" />
        <span className="stat-card__label">{label}</span>
      </div>
      <strong className="stat-card__value">{value}</strong>
      <p className="stat-card__detail">{detail}</p>
    </article>
  );
}
