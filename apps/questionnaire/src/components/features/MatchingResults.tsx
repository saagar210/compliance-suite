import { useState } from "react";
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from "../ui/Table";
import Button from "../ui/Button";
import Input from "../ui/Input";
import { useMatching } from "../../hooks/useMatching";

export default function MatchingResults() {
  const { suggestions, loading, error, getMatches } = useMatching();
  const [question, setQuestion] = useState("");

  const handleSearch = async () => {
    if (!question.trim()) return;
    try {
      await getMatches(question, 10);
    } catch (err) {
      console.error("Matching failed:", err);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      handleSearch();
    }
  };

  const getScoreColor = (score: number) => {
    if (score >= 0.7) return "text-green-600 font-semibold";
    if (score >= 0.4) return "text-yellow-600";
    return "text-muted-foreground";
  };

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-semibold mb-2">Find Matching Answers</h2>
        <p className="text-sm text-muted-foreground mb-4">
          Enter a question to find matching answers from your answer bank
        </p>
      </div>

      <div className="flex gap-4">
        <Input
          value={question}
          onChange={(e) => setQuestion(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Type your question here..."
          className="flex-1"
        />
        <Button onClick={handleSearch} disabled={loading || !question.trim()}>
          {loading ? "Searching..." : "Find Matches"}
        </Button>
      </div>

      {error && (
        <div className="p-4 bg-destructive/10 border border-destructive/20 rounded-md">
          <p className="text-sm text-destructive">{error}</p>
        </div>
      )}

      {suggestions.length > 0 && (
        <div className="border rounded-md">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-[100px]">Score</TableHead>
                <TableHead>Matched Question</TableHead>
                <TableHead>Answer Preview</TableHead>
                <TableHead>Confidence</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {suggestions.map((suggestion, idx) => (
                <TableRow key={`${suggestion.answer_bank_entry_id}-${idx}`}>
                  <TableCell>
                    <span className={getScoreColor(suggestion.score)}>
                      {(suggestion.score * 100).toFixed(0)}%
                    </span>
                  </TableCell>
                  <TableCell className="font-medium">{suggestion.normalized_question}</TableCell>
                  <TableCell className="text-sm text-muted-foreground">
                    {suggestion.normalized_answer?.substring(0, 100) || ""}
                    {(suggestion.normalized_answer?.length || 0) > 100 && "..."}
                  </TableCell>
                  <TableCell className="text-sm">{suggestion.confidence_explanation}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      )}

      {!loading && suggestions.length === 0 && question && (
        <div className="text-center py-12 border rounded-md">
          <p className="text-muted-foreground">
            No matches found. Try rephrasing your question or add more entries to the answer bank.
          </p>
        </div>
      )}
    </div>
  );
}
