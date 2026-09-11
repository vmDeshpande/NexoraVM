import { useEffect, useState } from "react";
import { runtimeService } from "../lib/runtimeService";
import type { RuntimeSnapshot } from "../types/runtime";

interface RuntimeSnapshotState {
  error: string | null;
  loading: boolean;
  snapshot: RuntimeSnapshot | null;
}

export function useRuntimeSnapshot() {
  const [state, setState] = useState<RuntimeSnapshotState>({
    error: null,
    loading: true,
    snapshot: null,
  });

  useEffect(() => {
    let isCurrent = true;

    runtimeService
      .getRuntimeSnapshot()
      .then((snapshot) => {
        if (isCurrent) {
          setState({ error: null, loading: false, snapshot });
        }
      })
      .catch(() => {
        if (isCurrent) {
          setState({
            error: "Runtime information is temporarily unavailable.",
            loading: false,
            snapshot: null,
          });
        }
      });

    return () => {
      isCurrent = false;
    };
  }, []);

  return state;
}
