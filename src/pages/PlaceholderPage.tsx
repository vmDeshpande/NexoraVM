import { EmptyState } from "../components/EmptyState";
import type { NavigationView } from "../types/navigation";

interface PlaceholderPageProps {
  description: string;
  onNavigate: (view: NavigationView) => void;
  returnView: NavigationView;
  title: string;
}

export function PlaceholderPage({
  description,
  onNavigate,
  returnView,
  title,
}: PlaceholderPageProps) {
  return (
    <div className="placeholder-page">
      <EmptyState
        actionLabel="Back to Dashboard"
        description={description}
        onAction={() => onNavigate(returnView)}
        title={title}
      />
    </div>
  );
}
