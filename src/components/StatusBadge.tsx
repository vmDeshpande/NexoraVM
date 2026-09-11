import type { RuntimeState } from "../types/runtime";

type StatusTone = "neutral" | "info" | "success" | "warning" | "error";

const toneByState: Record<RuntimeState, StatusTone> = {
  starting: "info",
  ready: "success",
  "not-configured": "neutral",
  error: "error",
};

interface StatusBadgeProps {
  runtime: {
    detail: string;
    label: string;
    state: RuntimeState;
  };
}

export function StatusBadge({ runtime }: StatusBadgeProps) {
  return (
    <span
      className={`status-badge status-badge--${toneByState[runtime.state]}`}
      role="status"
      title={runtime.detail}
    >
      <span className="status-badge__dot" aria-hidden="true" />
      {runtime.label}
    </span>
  );
}
