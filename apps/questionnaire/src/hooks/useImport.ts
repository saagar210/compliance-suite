import { useCallback } from "react";
import { useImportStore } from "../state/importStore";
import { useUiStore } from "../state/uiStore";
import {
  invokeImportQuestionnaire,
  invokeGetColumnProfiles,
  invokeSaveColumnMapping,
} from "../api/tauri";
import type { ColumnMapDto } from "@packages/types";

export function useImport() {
  const store = useImportStore();
  const addToast = useUiStore((state) => state.addToast);

  const importFile = useCallback(
    async (filePath: string) => {
      store.setLoading(true);
      store.setError(null);
      try {
        const importData = await invokeImportQuestionnaire(filePath);
        store.setCurrentImport(importData);

        // Fetch column profiles
        const profiles = await invokeGetColumnProfiles(importData.import_id);
        store.setColumnProfiles(profiles);

        addToast({
          title: "Import Successful",
          description: `Imported questionnaire from ${importData.source_filename}`,
          variant: "success",
        });

        return importData;
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        store.setError(message);
        addToast({
          title: "Import Failed",
          description: message,
          variant: "destructive",
        });
        throw err;
      } finally {
        store.setLoading(false);
      }
    },
    [store, addToast]
  );

  const saveColumnMapping = useCallback(
    async (importId: string, columnMap: ColumnMapDto) => {
      store.setLoading(true);
      store.setError(null);
      try {
        const updatedImport = await invokeSaveColumnMapping(importId, columnMap);
        store.setCurrentImport(updatedImport);
        store.setColumnMap(columnMap);

        addToast({
          title: "Column Mapping Saved",
          description: "Column mapping has been saved successfully",
          variant: "success",
        });

        return updatedImport;
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        store.setError(message);
        addToast({
          title: "Failed to Save Mapping",
          description: message,
          variant: "destructive",
        });
        throw err;
      } finally {
        store.setLoading(false);
      }
    },
    [store, addToast]
  );

  return {
    importFile,
    saveColumnMapping,
    currentImport: store.currentImport,
    columnProfiles: store.columnProfiles,
    columnMap: store.columnMap,
    loading: store.loading,
    error: store.error,
    step: store.step,
    setStep: store.setStep,
  };
}
