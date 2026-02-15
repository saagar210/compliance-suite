import { BrowserRouter, Routes, Route } from "react-router-dom";
import { QueryClientProvider, QueryClient } from "@tanstack/react-query";
import ImportPage from "./routes/Import";
import MapPage from "./routes/Map";
import AnswerBankPage from "./routes/AnswerBank";
import ReviewPage from "./routes/Review";
import ExportPage from "./routes/Export";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<ImportPage />} />
          <Route path="/map" element={<MapPage />} />
          <Route path="/answer-bank" element={<AnswerBankPage />} />
          <Route path="/review" element={<ReviewPage />} />
          <Route path="/export" element={<ExportPage />} />
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  );
}
