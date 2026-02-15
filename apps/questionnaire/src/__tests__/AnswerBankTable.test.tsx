import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import AnswerBankTable from "../components/features/AnswerBankTable";

// Mock the hooks
const mockLoadEntries = vi.fn();
const mockCreateEntry = vi.fn();
const mockDeleteEntry = vi.fn();
const mockSetOffset = vi.fn();

const mockUseAnswerBank = vi.fn(() => ({
  entries: [
    {
      entry_id: "entry-1",
      question_canonical: "What is your security policy?",
      answer_short: "We follow industry standards",
      answer_long: "Our security policy follows industry best practices...",
      notes: "Review annually",
      tags: ["security", "policy"],
      owner: "admin",
      source: "manual",
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    },
    {
      entry_id: "entry-2",
      question_canonical: "How do you handle data breaches?",
      answer_short: "We have an incident response plan",
      answer_long: "Our incident response plan includes...",
      notes: "",
      tags: ["security", "incident"],
      owner: "user",
      source: "manual",
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    },
  ],
  loading: false,
  loadEntries: mockLoadEntries,
  createEntry: mockCreateEntry,
  deleteEntry: mockDeleteEntry,
  setOffset: mockSetOffset,
  offset: 0,
  limit: 20,
}));

vi.mock("../hooks/useAnswerBank", () => ({
  useAnswerBank: () => mockUseAnswerBank(),
}));

// Mock window.confirm
const originalConfirm = window.confirm;

describe("AnswerBankTable", () => {
  beforeEach(() => {
    mockLoadEntries.mockReset();
    mockCreateEntry.mockReset();
    mockDeleteEntry.mockReset();
    mockSetOffset.mockReset();
    window.confirm = vi.fn(() => true);
  });

  afterEach(() => {
    window.confirm = originalConfirm;
  });

  it("should render answer bank table with entries", () => {
    render(<AnswerBankTable />);

    expect(screen.getByText(/Answer Bank/i)).toBeInTheDocument();
    expect(screen.getByText(/What is your security policy\?/i)).toBeInTheDocument();
    expect(screen.getByText(/How do you handle data breaches\?/i)).toBeInTheDocument();
    expect(screen.getByText(/We follow industry standards/i)).toBeInTheDocument();
    expect(screen.getByText(/We have an incident response plan/i)).toBeInTheDocument();
  });

  it("should display tags for each entry", () => {
    render(<AnswerBankTable />);

    const securityTags = screen.getAllByText("security");
    expect(securityTags.length).toBeGreaterThan(0);
    expect(screen.getByText("policy")).toBeInTheDocument();
    expect(screen.getByText("incident")).toBeInTheDocument();
  });

  it("should call loadEntries on mount", () => {
    render(<AnswerBankTable />);

    expect(mockLoadEntries).toHaveBeenCalled();
  });

  it("should show add entry button", () => {
    render(<AnswerBankTable />);

    expect(screen.getByText(/Add Entry/i)).toBeInTheDocument();
  });

  it("should open create dialog when add entry button is clicked", async () => {
    const user = userEvent.setup();
    render(<AnswerBankTable />);

    const addButton = screen.getByText(/Add Entry/i);
    await user.click(addButton);

    await waitFor(() => {
      expect(screen.getByText(/Create Answer Bank Entry/i)).toBeInTheDocument();
    });
  });

  it("should render create form with all fields", async () => {
    const user = userEvent.setup();
    render(<AnswerBankTable />);

    const addButton = screen.getByText(/Add Entry/i);
    await user.click(addButton);

    await waitFor(() => {
      expect(screen.getByPlaceholderText(/Enter the question/i)).toBeInTheDocument();
      expect(screen.getByPlaceholderText(/Brief answer/i)).toBeInTheDocument();
      expect(screen.getByPlaceholderText(/Detailed answer/i)).toBeInTheDocument();
      expect(screen.getByPlaceholderText(/Optional notes/i)).toBeInTheDocument();
      expect(screen.getByPlaceholderText(/tag1, tag2, tag3/i)).toBeInTheDocument();
    });
  });

  it("should create new entry when form is submitted", async () => {
    const user = userEvent.setup();
    mockCreateEntry.mockResolvedValue({
      entry_id: "entry-3",
      question_canonical: "New question?",
      answer_short: "New answer",
      answer_long: "New detailed answer",
      notes: "New notes",
      tags: ["new", "test"],
      owner: "user",
      source: "manual",
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    });

    render(<AnswerBankTable />);

    const addButton = screen.getByText(/Add Entry/i);
    await user.click(addButton);

    await waitFor(() => {
      expect(screen.getByPlaceholderText(/Enter the question/i)).toBeInTheDocument();
    });

    // Fill in form
    await user.type(screen.getByPlaceholderText(/Enter the question/i), "New question?");
    await user.type(screen.getByPlaceholderText(/Brief answer/i), "New answer");
    await user.type(screen.getByPlaceholderText(/Detailed answer/i), "New detailed answer");
    await user.type(screen.getByPlaceholderText(/Optional notes/i), "New notes");
    await user.type(screen.getByPlaceholderText(/tag1, tag2, tag3/i), "new, test");

    // Submit form
    const createButton = screen.getByText(/Create Entry/i);
    await user.click(createButton);

    await waitFor(() => {
      expect(mockCreateEntry).toHaveBeenCalledWith({
        question_canonical: "New question?",
        answer_short: "New answer",
        answer_long: "New detailed answer",
        notes: "New notes",
        evidence_links: [],
        owner: "user",
        last_reviewed_at: undefined,
        tags: ["new", "test"],
        source: "manual",
      });
    });
  });

  it("should delete entry when delete button is clicked and confirmed", async () => {
    const user = userEvent.setup();
    mockDeleteEntry.mockResolvedValue(undefined);

    render(<AnswerBankTable />);

    const deleteButtons = screen.getAllByText(/Delete/i);
    await user.click(deleteButtons[0]);

    await waitFor(() => {
      expect(window.confirm).toHaveBeenCalledWith("Are you sure you want to delete this entry?");
      expect(mockDeleteEntry).toHaveBeenCalledWith("entry-1");
    });
  });

  it("should not delete entry when deletion is cancelled", async () => {
    const user = userEvent.setup();
    window.confirm = vi.fn(() => false);

    render(<AnswerBankTable />);

    const deleteButtons = screen.getAllByText(/Delete/i);
    await user.click(deleteButtons[0]);

    await waitFor(() => {
      expect(window.confirm).toHaveBeenCalled();
    });

    expect(mockDeleteEntry).not.toHaveBeenCalled();
  });

  it("should show empty state when no entries exist", () => {
    mockUseAnswerBank.mockReturnValue({
      entries: [],
      loading: false,
      loadEntries: mockLoadEntries,
      createEntry: mockCreateEntry,
      deleteEntry: mockDeleteEntry,
      setOffset: mockSetOffset,
      offset: 0,
      limit: 20,
    });

    render(<AnswerBankTable />);

    expect(
      screen.getByText(/No entries found. Create your first entry to get started./i)
    ).toBeInTheDocument();
  });

  it("should show loading state", () => {
    mockUseAnswerBank.mockReturnValue({
      entries: [],
      loading: true,
      loadEntries: mockLoadEntries,
      createEntry: mockCreateEntry,
      deleteEntry: mockDeleteEntry,
      setOffset: mockSetOffset,
      offset: 0,
      limit: 20,
    });

    render(<AnswerBankTable />);

    expect(screen.getByText(/Loading.../i)).toBeInTheDocument();
  });

  it("should handle pagination - next page", async () => {
    const user = userEvent.setup();
    render(<AnswerBankTable />);

    const nextButton = screen.getByText(/Next/i);
    await user.click(nextButton);

    await waitFor(() => {
      expect(mockSetOffset).toHaveBeenCalledWith(20);
      expect(mockLoadEntries).toHaveBeenCalledWith(20, 20);
    });
  });

  it("should handle pagination - previous page", async () => {
    const user = userEvent.setup();
    mockUseAnswerBank.mockReturnValue({
      entries: [],
      loading: false,
      loadEntries: mockLoadEntries,
      createEntry: mockCreateEntry,
      deleteEntry: mockDeleteEntry,
      setOffset: mockSetOffset,
      offset: 20,
      limit: 20,
    });

    render(<AnswerBankTable />);

    const previousButton = screen.getByText(/Previous/i);
    await user.click(previousButton);

    await waitFor(() => {
      expect(mockSetOffset).toHaveBeenCalledWith(0);
      expect(mockLoadEntries).toHaveBeenCalledWith(20, 0);
    });
  });

  it("should disable previous button on first page", () => {
    render(<AnswerBankTable />);

    const previousButton = screen.getByText(/Previous/i);
    expect(previousButton).toBeDisabled();
  });

  it("should disable next button when fewer entries than limit", () => {
    mockUseAnswerBank.mockReturnValue({
      entries: [
        {
          entry_id: "entry-1",
          question_canonical: "Question",
          answer_short: "Answer",
          answer_long: "Long answer",
          notes: "",
          tags: [],
          owner: "user",
          source: "manual",
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        },
      ],
      loading: false,
      loadEntries: mockLoadEntries,
      createEntry: mockCreateEntry,
      deleteEntry: mockDeleteEntry,
      setOffset: mockSetOffset,
      offset: 0,
      limit: 20,
    });

    render(<AnswerBankTable />);

    const nextButton = screen.getByText(/Next/i);
    expect(nextButton).toBeDisabled();
  });
});
