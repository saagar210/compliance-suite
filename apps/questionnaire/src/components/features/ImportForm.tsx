import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import Button from "../ui/Button";
import Input from "../ui/Input";
import Form from "../ui/Form";
import { useImport } from "../../hooks/useImport";

interface ImportFormProps {
  onComplete: () => void;
}

export default function ImportForm({ onComplete }: ImportFormProps) {
  const { importFile, loading, error } = useImport();
  const [selectedFile, setSelectedFile] = useState<string | null>(null);

  const handleSelectFile = async () => {
    try {
      const filePath = await open({
        filters: [
          {
            name: "Questionnaire Files",
            extensions: ["xlsx", "csv"],
          },
        ],
      });

      if (filePath && typeof filePath === "string") {
        setSelectedFile(filePath);
      }
    } catch (err) {
      console.error("File selection failed:", err);
    }
  };

  const handleImport = async () => {
    if (!selectedFile) return;

    try {
      await importFile(selectedFile);
      onComplete();
    } catch (err) {
      // Error already handled by useImport hook
      console.error("Import failed:", err);
    }
  };

  return (
    <Form className="max-w-2xl">
      <div className="space-y-4">
        <div>
          <h2 className="text-lg font-semibold mb-2">Select Questionnaire File</h2>
          <p className="text-sm text-muted-foreground mb-4">
            Choose an Excel (.xlsx) or CSV (.csv) file containing your questionnaire data.
          </p>
        </div>

        <div className="flex gap-4 items-end">
          <Input
            label="Selected File"
            value={selectedFile || "No file selected"}
            readOnly
            className="flex-1"
          />
          <Button onClick={handleSelectFile} variant="outline" disabled={loading}>
            Browse...
          </Button>
        </div>

        {error && (
          <div className="p-4 bg-destructive/10 border border-destructive/20 rounded-md">
            <p className="text-sm text-destructive">{error}</p>
          </div>
        )}

        <div className="flex gap-4">
          <Button
            onClick={handleImport}
            disabled={!selectedFile || loading}
            className="min-w-32"
          >
            {loading ? "Importing..." : "Import Questionnaire"}
          </Button>
        </div>
      </div>
    </Form>
  );
}
