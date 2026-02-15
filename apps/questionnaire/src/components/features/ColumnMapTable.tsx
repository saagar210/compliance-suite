import { useState, useEffect } from "react";
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from "../ui/Table";
import Button from "../ui/Button";
import { useImport } from "../../hooks/useImport";
import type { ColumnMapDto } from "@packages/types";

interface ColumnMapTableProps {
  onComplete: () => void;
}

export default function ColumnMapTable({ onComplete }: ColumnMapTableProps) {
  const { currentImport, columnProfiles, saveColumnMapping, loading } = useImport();
  const [questionCol, setQuestionCol] = useState<string>("");
  const [answerCol, setAnswerCol] = useState<string>("");
  const [notesCol, setNotesCol] = useState<string>("");

  // Auto-detect columns based on labels
  useEffect(() => {
    if (columnProfiles.length > 0) {
      // Simple heuristic: look for columns with "question" and "answer" in their labels
      const qCol = columnProfiles.find((p) =>
        p.label.toLowerCase().includes("question")
      );
      const aCol = columnProfiles.find((p) =>
        p.label.toLowerCase().includes("answer")
      );
      const nCol = columnProfiles.find((p) =>
        p.label.toLowerCase().includes("note")
      );

      if (qCol) setQuestionCol(qCol.col_ref);
      if (aCol) setAnswerCol(aCol.col_ref);
      if (nCol) setNotesCol(nCol.col_ref);
    }
  }, [columnProfiles]);

  const handleSave = async () => {
    if (!currentImport || !questionCol || !answerCol) {
      return;
    }

    const columnMap: ColumnMapDto = {
      question: questionCol,
      answer: answerCol,
      notes: notesCol || undefined,
    };

    try {
      await saveColumnMapping(currentImport.import_id, columnMap);
      onComplete();
    } catch (err) {
      console.error("Failed to save column mapping:", err);
    }
  };

  const isValid = questionCol && answerCol;

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-semibold mb-2">Map Columns</h2>
        <p className="text-sm text-muted-foreground">
          Map the columns from your file to the expected fields. The system has auto-detected
          some mappings based on column names.
        </p>
      </div>

      <div className="border rounded-md">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Column</TableHead>
              <TableHead>Sample Data</TableHead>
              <TableHead>Map To</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {columnProfiles.map((profile) => (
              <TableRow key={profile.col_ref}>
                <TableCell className="font-medium">{profile.label}</TableCell>
                <TableCell className="text-sm text-muted-foreground">
                  {profile.sample.slice(0, 2).join(", ")}
                  {profile.sample.length > 2 && "..."}
                </TableCell>
                <TableCell>
                  <div className="flex gap-2">
                    <label className="flex items-center gap-2">
                      <input
                        type="radio"
                        name="question"
                        checked={questionCol === profile.col_ref}
                        onChange={() => setQuestionCol(profile.col_ref)}
                      />
                      <span className="text-sm">Question</span>
                    </label>
                    <label className="flex items-center gap-2">
                      <input
                        type="radio"
                        name="answer"
                        checked={answerCol === profile.col_ref}
                        onChange={() => setAnswerCol(profile.col_ref)}
                      />
                      <span className="text-sm">Answer</span>
                    </label>
                    <label className="flex items-center gap-2">
                      <input
                        type="radio"
                        name="notes"
                        checked={notesCol === profile.col_ref}
                        onChange={() => setNotesCol(profile.col_ref)}
                      />
                      <span className="text-sm">Notes</span>
                    </label>
                  </div>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      <div className="flex gap-4">
        <Button onClick={handleSave} disabled={!isValid || loading}>
          {loading ? "Saving..." : "Save Mapping & Continue"}
        </Button>
      </div>

      {!isValid && (
        <p className="text-sm text-muted-foreground">
          * Please map at least Question and Answer columns to continue
        </p>
      )}
    </div>
  );
}
