import { useState, useCallback } from "react";
import { invokeGetMatchingSuggestions } from "../api/tauri";
import { useUiStore } from "../state/uiStore";
import type { MatchSuggestionDto } from "@packages/types";

export function useMatching() {
  const [suggestions, setSuggestions] = useState<MatchSuggestionDto[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const addToast = useUiStore((state) => state.addToast);

  const getMatches = useCallback(
    async (question: string, topN?: number) => {
      if (!question.trim()) {
        setSuggestions([]);
        return [];
      }

      setLoading(true);
      setError(null);
      try {
        const results = await invokeGetMatchingSuggestions(question, topN);
        setSuggestions(results);
        return results;
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
        addToast({
          title: "Matching Failed",
          description: message,
          variant: "destructive",
        });
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [addToast]
  );

  const clearSuggestions = useCallback(() => {
    setSuggestions([]);
    setError(null);
  }, []);

  return {
    suggestions,
    loading,
    error,
    getMatches,
    clearSuggestions,
  };
}
