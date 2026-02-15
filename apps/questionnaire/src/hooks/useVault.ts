import { useState, useCallback } from "react";
import { invokeVaultCreate, invokeVaultOpen, invokeVaultClose } from "../api/tauri";
import { useUiStore } from "../state/uiStore";
import type { VaultDto } from "@packages/types";

export function useVault() {
  const [currentVault, setCurrentVault] = useState<VaultDto | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const addToast = useUiStore((state) => state.addToast);

  const createVault = useCallback(
    async (path: string, name: string) => {
      setLoading(true);
      setError(null);
      try {
        const vault = await invokeVaultCreate(path, name);
        setCurrentVault(vault);
        addToast({
          title: "Vault Created",
          description: `Vault "${name}" created successfully`,
          variant: "success",
        });
        return vault;
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
        addToast({
          title: "Failed to Create Vault",
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

  const openVault = useCallback(
    async (path: string) => {
      setLoading(true);
      setError(null);
      try {
        const vault = await invokeVaultOpen(path);
        setCurrentVault(vault);
        addToast({
          title: "Vault Opened",
          description: `Vault "${vault.name}" opened successfully`,
          variant: "success",
        });
        return vault;
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        setError(message);
        addToast({
          title: "Failed to Open Vault",
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

  const closeVault = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      await invokeVaultClose();
      setCurrentVault(null);
      addToast({
        title: "Vault Closed",
        description: "Vault closed successfully",
        variant: "default",
      });
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      addToast({
        title: "Failed to Close Vault",
        description: message,
        variant: "destructive",
      });
      throw err;
    } finally {
      setLoading(false);
    }
  }, [addToast]);

  return {
    currentVault,
    loading,
    error,
    createVault,
    openVault,
    closeVault,
  };
}
