import { useEffect, useState } from "react";
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from "../ui/Table";
import Button from "../ui/Button";
import Input from "../ui/Input";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from "../ui/Dialog";
import Form, { FormField } from "../ui/Form";
import { useAnswerBank } from "../../hooks/useAnswerBank";
import type { AnswerBankCreateInputDto } from "@packages/types";

export default function AnswerBankTable() {
  const { entries, loading, loadEntries, createEntry, deleteEntry, setOffset, offset, limit } =
    useAnswerBank();
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
  const [formData, setFormData] = useState({
    question_canonical: "",
    answer_short: "",
    answer_long: "",
    notes: "",
    tags: "",
    owner: "user",
  });

  useEffect(() => {
    loadEntries();
  }, [loadEntries]);

  const handleCreate = async () => {
    const input: AnswerBankCreateInputDto = {
      question_canonical: formData.question_canonical,
      answer_short: formData.answer_short,
      answer_long: formData.answer_long,
      notes: formData.notes || undefined,
      evidence_links: [],
      owner: formData.owner,
      last_reviewed_at: undefined,
      tags: formData.tags ? formData.tags.split(",").map((t) => t.trim()) : [],
      source: "manual",
    };

    try {
      await createEntry(input);
      setIsCreateDialogOpen(false);
      setFormData({
        question_canonical: "",
        answer_short: "",
        answer_long: "",
        notes: "",
        tags: "",
        owner: "user",
      });
    } catch (err) {
      console.error("Failed to create entry:", err);
    }
  };

  const handleDelete = async (entryId: string) => {
    if (confirm("Are you sure you want to delete this entry?")) {
      try {
        await deleteEntry(entryId);
      } catch (err) {
        console.error("Failed to delete entry:", err);
      }
    }
  };

  const handleNextPage = () => {
    setOffset(offset + limit);
    loadEntries(limit, offset + limit);
  };

  const handlePreviousPage = () => {
    const newOffset = Math.max(0, offset - limit);
    setOffset(newOffset);
    loadEntries(limit, newOffset);
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-lg font-semibold">Answer Bank</h2>
          <p className="text-sm text-muted-foreground">
            Manage your answer bank entries
          </p>
        </div>
        <Dialog open={isCreateDialogOpen} onOpenChange={setIsCreateDialogOpen}>
          <DialogTrigger asChild>
            <Button>Add Entry</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create Answer Bank Entry</DialogTitle>
            </DialogHeader>
            <Form className="space-y-4">
              <FormField label="Question" required>
                <Input
                  value={formData.question_canonical}
                  onChange={(e) =>
                    setFormData({ ...formData, question_canonical: e.target.value })
                  }
                  placeholder="Enter the question"
                />
              </FormField>
              <FormField label="Short Answer" required>
                <Input
                  value={formData.answer_short}
                  onChange={(e) =>
                    setFormData({ ...formData, answer_short: e.target.value })
                  }
                  placeholder="Brief answer"
                />
              </FormField>
              <FormField label="Long Answer" required>
                <textarea
                  className="w-full min-h-[100px] rounded-md border border-input bg-background px-3 py-2 text-sm"
                  value={formData.answer_long}
                  onChange={(e) =>
                    setFormData({ ...formData, answer_long: e.target.value })
                  }
                  placeholder="Detailed answer"
                />
              </FormField>
              <FormField label="Notes">
                <Input
                  value={formData.notes}
                  onChange={(e) => setFormData({ ...formData, notes: e.target.value })}
                  placeholder="Optional notes"
                />
              </FormField>
              <FormField label="Tags (comma-separated)">
                <Input
                  value={formData.tags}
                  onChange={(e) => setFormData({ ...formData, tags: e.target.value })}
                  placeholder="tag1, tag2, tag3"
                />
              </FormField>
              <Button onClick={handleCreate} disabled={loading}>
                Create Entry
              </Button>
            </Form>
          </DialogContent>
        </Dialog>
      </div>

      <div className="border rounded-md">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Question</TableHead>
              <TableHead>Short Answer</TableHead>
              <TableHead>Tags</TableHead>
              <TableHead>Owner</TableHead>
              <TableHead className="w-[100px]">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {loading && entries.length === 0 ? (
              <TableRow>
                <TableCell colSpan={5} className="text-center py-8 text-muted-foreground">
                  Loading...
                </TableCell>
              </TableRow>
            ) : entries.length === 0 ? (
              <TableRow>
                <TableCell colSpan={5} className="text-center py-8 text-muted-foreground">
                  No entries found. Create your first entry to get started.
                </TableCell>
              </TableRow>
            ) : (
              entries.map((entry) => (
                <TableRow key={entry.entry_id}>
                  <TableCell className="font-medium">{entry.question_canonical}</TableCell>
                  <TableCell>{entry.answer_short}</TableCell>
                  <TableCell>
                    <div className="flex gap-1 flex-wrap">
                      {entry.tags.map((tag) => (
                        <span
                          key={tag}
                          className="text-xs bg-secondary px-2 py-1 rounded"
                        >
                          {tag}
                        </span>
                      ))}
                    </div>
                  </TableCell>
                  <TableCell>{entry.owner}</TableCell>
                  <TableCell>
                    <Button
                      variant="destructive"
                      size="sm"
                      onClick={() => handleDelete(entry.entry_id)}
                    >
                      Delete
                    </Button>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>

      <div className="flex justify-between items-center">
        <p className="text-sm text-muted-foreground">
          Showing {offset + 1} - {Math.min(offset + limit, offset + entries.length)}
        </p>
        <div className="flex gap-2">
          <Button variant="outline" onClick={handlePreviousPage} disabled={offset === 0}>
            Previous
          </Button>
          <Button
            variant="outline"
            onClick={handleNextPage}
            disabled={entries.length < limit}
          >
            Next
          </Button>
        </div>
      </div>
    </div>
  );
}
