import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import ColumnMapTable from "../components/features/ColumnMapTable";

// Mock the hooks
const mockSaveColumnMapping = vi.fn();
const mockUseImport = vi.fn(() => ({
  currentImport: {
    import_id: "import-123",
    source_filename: "test.xlsx",
    source_path: "/path/to/test.xlsx",
    import_status: "pending_mapping",
    created_at: new Date().toISOString(),
  },
  columnProfiles: [
    {
      col_ref: "A",
      label: "Question Text",
      sample: ["What is your name?", "What is your role?", "What is your company?"],
    },
    {
      col_ref: "B",
      label: "Answer Details",
      sample: ["John Doe", "Software Engineer", "Acme Corp"],
    },
    {
      col_ref: "C",
      label: "Additional Notes",
      sample: ["Note 1", "Note 2", "Note 3"],
    },
  ],
  saveColumnMapping: mockSaveColumnMapping,
  loading: false,
}));

vi.mock("../hooks/useImport", () => ({
  useImport: () => mockUseImport(),
}));

describe("ColumnMapTable", () => {
  const mockOnComplete = vi.fn();

  beforeEach(() => {
    mockOnComplete.mockReset();
    mockSaveColumnMapping.mockReset();
  });

  it("should render column mapping table", () => {
    render(<ColumnMapTable onComplete={mockOnComplete} />);

    expect(screen.getByText(/Map Columns/i)).toBeInTheDocument();
    expect(screen.getByText(/Question Text/i)).toBeInTheDocument();
    expect(screen.getByText(/Answer Details/i)).toBeInTheDocument();
    expect(screen.getByText(/Additional Notes/i)).toBeInTheDocument();
  });

  it("should display sample data for each column", () => {
    render(<ColumnMapTable onComplete={mockOnComplete} />);

    expect(screen.getByText(/What is your name\?, What is your role\?.../i)).toBeInTheDocument();
    expect(screen.getByText(/John Doe, Software Engineer.../i)).toBeInTheDocument();
    expect(screen.getByText(/Note 1, Note 2.../i)).toBeInTheDocument();
  });

  it("should auto-detect columns based on labels", () => {
    render(<ColumnMapTable onComplete={mockOnComplete} />);

    // Check that "Question" radio is selected for column A
    const questionRadios = screen.getAllByRole("radio", { name: /Question/i });
    const answerRadios = screen.getAllByRole("radio", { name: /Answer/i });
    const notesRadios = screen.getAllByRole("radio", { name: /Notes/i });

    // Question should be auto-selected for column A
    expect(questionRadios[0]).toBeChecked();
    // Answer should be auto-selected for column B
    expect(answerRadios[1]).toBeChecked();
    // Notes should be auto-selected for column C
    expect(notesRadios[2]).toBeChecked();
  });

  it("should allow user to select column mappings", async () => {
    const user = userEvent.setup();
    render(<ColumnMapTable onComplete={mockOnComplete} />);

    const questionRadios = screen.getAllByRole("radio", { name: /Question/i });

    // Click on question radio for column B
    await user.click(questionRadios[1]);

    await waitFor(() => {
      expect(questionRadios[1]).toBeChecked();
    });
  });

  it("should disable save button when required mappings are missing", () => {
    mockUseImport.mockReturnValue({
      currentImport: {
        import_id: "import-123",
        source_filename: "test.xlsx",
        source_path: "/path/to/test.xlsx",
        import_status: "pending_mapping",
        created_at: new Date().toISOString(),
      },
      columnProfiles: [
        {
          col_ref: "A",
          label: "Column A",
          sample: ["Sample 1", "Sample 2"],
        },
      ],
      saveColumnMapping: mockSaveColumnMapping,
      loading: false,
    });

    render(<ColumnMapTable onComplete={mockOnComplete} />);

    const saveButton = screen.getByText(/Save Mapping & Continue/i);
    expect(saveButton).toBeDisabled();
  });

  it("should show validation message when required mappings are missing", () => {
    mockUseImport.mockReturnValue({
      currentImport: {
        import_id: "import-123",
        source_filename: "test.xlsx",
        source_path: "/path/to/test.xlsx",
        import_status: "pending_mapping",
        created_at: new Date().toISOString(),
      },
      columnProfiles: [
        {
          col_ref: "A",
          label: "Column A",
          sample: ["Sample 1", "Sample 2"],
        },
      ],
      saveColumnMapping: mockSaveColumnMapping,
      loading: false,
    });

    render(<ColumnMapTable onComplete={mockOnComplete} />);

    expect(
      screen.getByText(/Please map at least Question and Answer columns to continue/i)
    ).toBeInTheDocument();
  });

  it("should call saveColumnMapping and onComplete when save button is clicked", async () => {
    const user = userEvent.setup();
    mockSaveColumnMapping.mockResolvedValue({
      import_id: "import-123",
      column_map: { question: "A", answer: "B", notes: "C" },
    });

    render(<ColumnMapTable onComplete={mockOnComplete} />);

    const saveButton = screen.getByText(/Save Mapping & Continue/i);
    await user.click(saveButton);

    await waitFor(() => {
      expect(mockSaveColumnMapping).toHaveBeenCalledWith("import-123", {
        question: "A",
        answer: "B",
        notes: "C",
      });
      expect(mockOnComplete).toHaveBeenCalled();
    });
  });

  it("should show loading state during save", () => {
    mockUseImport.mockReturnValue({
      currentImport: {
        import_id: "import-123",
        source_filename: "test.xlsx",
        source_path: "/path/to/test.xlsx",
        import_status: "pending_mapping",
        created_at: new Date().toISOString(),
      },
      columnProfiles: [
        {
          col_ref: "A",
          label: "Question Text",
          sample: ["What is your name?"],
        },
        {
          col_ref: "B",
          label: "Answer Details",
          sample: ["John Doe"],
        },
      ],
      saveColumnMapping: mockSaveColumnMapping,
      loading: true,
    });

    render(<ColumnMapTable onComplete={mockOnComplete} />);

    expect(screen.getByText(/Saving.../i)).toBeInTheDocument();
    expect(screen.getByText(/Saving.../i)).toBeDisabled();
  });

  it("should allow notes column to be optional", async () => {
    const user = userEvent.setup();
    mockSaveColumnMapping.mockResolvedValue({
      import_id: "import-123",
      column_map: { question: "A", answer: "B" },
    });

    mockUseImport.mockReturnValue({
      currentImport: {
        import_id: "import-123",
        source_filename: "test.xlsx",
        source_path: "/path/to/test.xlsx",
        import_status: "pending_mapping",
        created_at: new Date().toISOString(),
      },
      columnProfiles: [
        {
          col_ref: "A",
          label: "Question",
          sample: ["Q1"],
        },
        {
          col_ref: "B",
          label: "Answer",
          sample: ["A1"],
        },
      ],
      saveColumnMapping: mockSaveColumnMapping,
      loading: false,
    });

    render(<ColumnMapTable onComplete={mockOnComplete} />);

    const saveButton = screen.getByText(/Save Mapping & Continue/i);
    await user.click(saveButton);

    await waitFor(() => {
      expect(mockSaveColumnMapping).toHaveBeenCalledWith("import-123", {
        question: "A",
        answer: "B",
        notes: undefined,
      });
    });
  });
});
