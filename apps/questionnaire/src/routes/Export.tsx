import { Sidebar } from "../components/layout/Sidebar";
import { Header } from "../components/layout/Header";
import ExportDialog from "../components/features/ExportDialog";
import { Toaster } from "../components/ui/Toast";

export default function ExportPage() {
  return (
    <div className="flex h-screen bg-background">
      <Sidebar />
      <div className="flex-1 flex flex-col overflow-hidden">
        <Header
          title="Export"
          subtitle="Step 5: Generate your final export pack"
        />
        <main className="flex-1 overflow-auto p-8">
          <div className="max-w-4xl mx-auto">
            <div className="space-y-6">
              <div>
                <h2 className="text-lg font-semibold mb-2">Ready to Export</h2>
                <p className="text-sm text-muted-foreground mb-6">
                  Your questionnaire has been processed and is ready for export. The export
                  pack will include all imported data, mapped columns, answer bank entries,
                  and any linked evidence files.
                </p>
              </div>

              <div className="border rounded-lg p-6 bg-secondary/20">
                <h3 className="font-medium mb-4">Export Contents</h3>
                <ul className="space-y-2 text-sm text-muted-foreground">
                  <li className="flex items-center gap-2">
                    <span className="text-primary">✓</span>
                    Questionnaire data with mapped columns
                  </li>
                  <li className="flex items-center gap-2">
                    <span className="text-primary">✓</span>
                    Answer bank entries
                  </li>
                  <li className="flex items-center gap-2">
                    <span className="text-primary">✓</span>
                    Evidence files and links
                  </li>
                  <li className="flex items-center gap-2">
                    <span className="text-primary">✓</span>
                    Audit trail and metadata
                  </li>
                </ul>
              </div>

              <div className="pt-4">
                <ExportDialog />
              </div>
            </div>
          </div>
        </main>
      </div>
      <Toaster />
    </div>
  );
}
