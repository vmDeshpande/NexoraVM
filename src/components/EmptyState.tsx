import { Button } from "./Button";

interface EmptyStateProps {
  actionDisabled?: boolean;
  actionLabel?: string;
  description: string;
  onAction?: () => void;
  title: string;
}

export function EmptyState({
  actionDisabled = false,
  actionLabel,
  description,
  onAction,
  title,
}: EmptyStateProps) {
  return (
    <div className="empty-state" role="status">
      <div className="empty-state__icon" aria-hidden="true">
        <span />
      </div>
      <div className="empty-state__copy">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
      {actionLabel ? (
        <Button disabled={actionDisabled} onClick={onAction}>
          {actionLabel}
        </Button>
      ) : null}
    </div>
  );
}
