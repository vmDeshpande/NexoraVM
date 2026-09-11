import { StatusBadge } from "./StatusBadge";
import type { PageMeta } from "../types/navigation";
import type { RuntimeStatus } from "../types/runtime";

interface HeaderProps {
  page: PageMeta;
  runtime: RuntimeStatus;
}

export function Header({ page, runtime }: HeaderProps) {
  return (
    <header className="header">
      <div className="header__copy">
        <p className="header__eyebrow">NexoraVM / {page.section}</p>
        <h1>{page.title}</h1>
        <p className="header__description">{page.description}</p>
      </div>
      <div className="header__status" aria-label="Current runtime status">
        <span className="header__status-label">Runtime</span>
        <StatusBadge runtime={runtime} />
      </div>
    </header>
  );
}
