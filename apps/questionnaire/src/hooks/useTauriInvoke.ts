import { useState, useCallback } from "react";
import { useUiStore } from "../state/uiStore";

export interface TauriInvokeOptions {
  showErrorToast?: boolean;
  errorPrefix?: string;
}

/**
 * Generic hook for Tauri invoke with error handling and loading state
 */
export function useTauriInvoke<T, Args extends unknown[]>(
  invokeFunction: (...args: Args) => Promise<T>,
  options: TauriInvokeOptions = {}
) {
  const { showErrorToast = true, errorPrefix = "Error" } = options;
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [data, setData] = useState<T | null>(null);
  const addToast = useUiStore((state) => state.addToast);

  const execute = useCallback(
    async (...args: Args) => {
      setLoading(true);
      setError(null);
      try {
        const result = await invokeFunction(...args);
        setData(result);
        return result;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : String(err);
        setError(errorMessage);

        if (showErrorToast) {
          addToast({
            title: errorPrefix,
            description: errorMessage,
            variant: "destructive",
          });
        }
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [invokeFunction, showErrorToast, errorPrefix, addToast]
  );

  const reset = useCallback(() => {
    setLoading(false);
    setError(null);
    setData(null);
  }, []);

  return {
    execute,
    loading,
    error,
    data,
    reset,
  };
}
