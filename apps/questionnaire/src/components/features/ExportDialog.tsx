import { useState } from "react";
import { save } from "@tauri-apps/plugin-dialog";
import Button from "../ui/Button";
import Input from "../ui/Input";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from "../ui/Dialog";
import { invokeGenerateExportPack } from "../../api/tauri";
import { useUiStore } from "../../state/uiStore";

export default function ExportDialog() {
  const [isOpen, setIsOpen] = useState(false);
  const [loading, setLoading] = useState(false);
  const [exportPath, setExportPath] = useState<string>("");
  const addToast = useUiStore((state) => state.addToast);

  const handleSelectPath = async () => {
    try {
      const filePath = await save({
        filters: [
          {
            name: "Export Pack",
            extensions: ["zip"],
          },
        ],
        defaultPath: "questionnaire-export.zip",
      });

      if (filePath && typeof filePath === "string") {
        setExportPath(filePath);
      }
    } catch (err) {
      console.error("Path selection failed:", err);
    }
  };

  const handleExport = async () => {
    if (!exportPath) return;

    setLoading(true);
    try {
      const result = await invokeGenerateExportPack(exportPath);
      addToast({
        title: "Export Successful",
        description: `Export pack created with ${result.file_count} files`,
        variant: "success",
      });
      setIsOpen(false);
      setExportPath("");
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      addToast({
        title: "Export Failed",
        description: message,
        variant: "destructive",
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button size="lg">Generate Export Pack</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Generate Export Pack</DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <p className="text-sm text-muted-foreground">
            This will create a ZIP file containing all questionnaire data, answer bank entries,
            and evidence files.
          </p>

          <div className="flex gap-4 items-end">
            <Input
              label="Export Location"
              value={exportPath || "No path selected"}
              readOnly
              className="flex-1"
            />
            <Button onClick={handleSelectPath} variant="outline" disabled={loading}>
              Browse...
            </Button>
          </div>

          <div className="flex gap-4 pt-4">
            <Button onClick={handleExport} disabled={!exportPath || loading}>
              {loading ? "Exporting..." : "Generate Export"}
            </Button>
            <Button variant="outline" onClick={() => setIsOpen(false)} disabled={loading}>
              Cancel
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
