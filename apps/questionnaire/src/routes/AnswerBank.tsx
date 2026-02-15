import { Sidebar } from "../components/layout/Sidebar";
import { Header } from "../components/layout/Header";
import AnswerBankTable from "../components/features/AnswerBankTable";
import { Toaster } from "../components/ui/Toast";

export default function AnswerBankPage() {
  return (
    <div className="flex h-screen bg-background">
      <Sidebar />
      <div className="flex-1 flex flex-col overflow-hidden">
        <Header title="Answer Bank" subtitle="Step 3: Manage your answer bank entries" />
        <main className="flex-1 overflow-auto p-8">
          <div className="max-w-6xl mx-auto">
            <AnswerBankTable />
          </div>
        </main>
      </div>
      <Toaster />
    </div>
  );
}
