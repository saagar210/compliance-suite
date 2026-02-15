import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import ImportForm from "../components/features/ImportForm";
import { mockTauriInvoke, mockTauriDialog } from "./setup";

// Mock the hooks
const mockImportFile = vi.fn();
const mockUseImport = vi.fn(() => ({
  importFile: mockImportFile,
  loading: false,
  error: null,
}));

vi.mock("../hooks/useImport", () => ({
  useImport: () => mockUseImport(),
}));

describe("ImportForm", () => {
  const mockOnComplete = vi.fn();

  beforeEach(() => {
    mockOnComplete.mockReset();
    mockImportFile.mockReset();
    mockTauriDialog.open.mockReset();
  });

  it("should render import form with browse button", () => {
    render(<ImportForm onComplete={mockOnComplete} />);

    expect(screen.getByText(/Select Questionnaire File/i)).toBeInTheDocument();
    expect(screen.getByText(/Browse.../i)).toBeInTheDocument();
    expect(screen.getByText(/Import Questionnaire/i)).toBeInTheDocument();
  });

  it("should show 'No file selected' initially", () => {
    render(<ImportForm onComplete={mockOnComplete} />);

    expect(screen.getByDisplayValue(/No file selected/i)).toBeInTheDocument();
  });

  it("should disable import button when no file is selected", () => {
    render(<ImportForm onComplete={mockOnComplete} />);

    const importButton = screen.getByText(/Import Questionnaire/i);
    expect(importButton).toBeDisabled();
  });

  it("should select file when browse button is clicked", async () => {
    const user = userEvent.setup();
    mockTauriDialog.open.mockResolvedValue("/path/to/questionnaire.xlsx");

    render(<ImportForm onComplete={mockOnComplete} />);

    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      expect(mockTauriDialog.open).toHaveBeenCalledWith({
        filters: [
          {
            name: "Questionnaire Files",
            extensions: ["xlsx", "csv"],
          },
        ],
      });
    });

    await waitFor(() => {
      expect(screen.getByDisplayValue("/path/to/questionnaire.xlsx")).toBeInTheDocument();
    });
  });

  it("should enable import button when file is selected", async () => {
    const user = userEvent.setup();
    mockTauriDialog.open.mockResolvedValue("/path/to/questionnaire.xlsx");

    render(<ImportForm onComplete={mockOnComplete} />);

    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      const importButton = screen.getByText(/Import Questionnaire/i);
      expect(importButton).not.toBeDisabled();
    });
  });

  it("should call importFile and onComplete when import button is clicked", async () => {
    const user = userEvent.setup();
    mockTauriDialog.open.mockResolvedValue("/path/to/questionnaire.xlsx");
    mockImportFile.mockResolvedValue({ import_id: "123", source_filename: "test.xlsx" });

    render(<ImportForm onComplete={mockOnComplete} />);

    // Select file
    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      expect(screen.getByDisplayValue("/path/to/questionnaire.xlsx")).toBeInTheDocument();
    });

    // Import file
    const importButton = screen.getByText(/Import Questionnaire/i);
    await user.click(importButton);

    await waitFor(() => {
      expect(mockImportFile).toHaveBeenCalledWith("/path/to/questionnaire.xlsx");
      expect(mockOnComplete).toHaveBeenCalled();
    });
  });

  it("should show loading state during import", () => {
    mockUseImport.mockReturnValue({
      importFile: mockImportFile,
      loading: true,
      error: null,
    });

    render(<ImportForm onComplete={mockOnComplete} />);

    expect(screen.getByText(/Importing.../i)).toBeInTheDocument();
    expect(screen.getByText(/Browse.../i)).toBeDisabled();
  });

  it("should display error message when import fails", () => {
    const errorMessage = "Failed to import questionnaire";
    mockUseImport.mockReturnValue({
      importFile: mockImportFile,
      loading: false,
      error: errorMessage,
    });

    render(<ImportForm onComplete={mockOnComplete} />);

    expect(screen.getByText(errorMessage)).toBeInTheDocument();
  });

  it("should handle file selection cancellation", async () => {
    const user = userEvent.setup();
    mockTauriDialog.open.mockResolvedValue(null);

    render(<ImportForm onComplete={mockOnComplete} />);

    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      expect(mockTauriDialog.open).toHaveBeenCalled();
    });

    // File selection should still show "No file selected"
    expect(screen.getByDisplayValue(/No file selected/i)).toBeInTheDocument();
  });
});
