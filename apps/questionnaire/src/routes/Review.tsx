import { Sidebar } from "../components/layout/Sidebar";
import { Header } from "../components/layout/Header";
import MatchingResults from "../components/features/MatchingResults";
import { Toaster } from "../components/ui/Toast";

export default function ReviewPage() {
  return (
    <div className="flex h-screen bg-background">
      <Sidebar />
      <div className="flex-1 flex flex-col overflow-hidden">
        <Header
          title="Review & Match"
          subtitle="Step 4: Find matching answers for your questions"
        />
        <main className="flex-1 overflow-auto p-8">
          <div className="max-w-6xl mx-auto">
            <MatchingResults />
          </div>
        </main>
      </div>
      <Toaster />
    </div>
  );
}
