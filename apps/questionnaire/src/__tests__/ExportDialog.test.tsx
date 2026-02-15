import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import ExportDialog from "../components/features/ExportDialog";
import { mockTauriInvoke, mockTauriDialog } from "./setup";

// Mock the API
const mockInvokeGenerateExportPack = vi.fn();
vi.mock("../api/tauri", () => ({
  invokeGenerateExportPack: (...args: any[]) => mockInvokeGenerateExportPack(...args),
}));

// Mock the UI store
const mockAddToast = vi.fn();
vi.mock("../state/uiStore", () => ({
  useUiStore: (selector: any) => {
    const store = {
      addToast: mockAddToast,
    };
    return selector ? selector(store) : store;
  },
}));

describe("ExportDialog", () => {
  beforeEach(() => {
    mockTauriDialog.save.mockReset();
    mockInvokeGenerateExportPack.mockReset();
    mockAddToast.mockReset();
  });

  it("should render export trigger button", () => {
    render(<ExportDialog />);

    expect(screen.getByText(/Generate Export Pack/i)).toBeInTheDocument();
  });

  it("should open dialog when trigger button is clicked", async () => {
    const user = userEvent.setup();
    render(<ExportDialog />);

    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    await waitFor(() => {
      expect(screen.getAllByText(/Generate Export Pack/i).length).toBeGreaterThan(1);
      expect(
        screen.getByText(/This will create a ZIP file containing all questionnaire data/i)
      ).toBeInTheDocument();
    });
  });

  it("should show browse button and path input in dialog", async () => {
    const user = userEvent.setup();
    render(<ExportDialog />);

    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    await waitFor(() => {
      expect(screen.getByText(/Browse.../i)).toBeInTheDocument();
      expect(screen.getByDisplayValue(/No path selected/i)).toBeInTheDocument();
    });
  });

  it("should select export path when browse button is clicked", async () => {
    const user = userEvent.setup();
    mockTauriDialog.save.mockResolvedValue("/path/to/export/questionnaire-export.zip");

    render(<ExportDialog />);

    // Open dialog
    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    await waitFor(() => {
      expect(screen.getByText(/Browse.../i)).toBeInTheDocument();
    });

    // Click browse
    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      expect(mockTauriDialog.save).toHaveBeenCalledWith({
        filters: [
          {
            name: "Export Pack",
            extensions: ["zip"],
          },
        ],
        defaultPath: "questionnaire-export.zip",
      });
    });

    await waitFor(() => {
      expect(
        screen.getByDisplayValue("/path/to/export/questionnaire-export.zip")
      ).toBeInTheDocument();
    });
  });

  it("should disable generate button when no path is selected", async () => {
    const user = userEvent.setup();
    render(<ExportDialog />);

    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    await waitFor(() => {
      const generateButton = screen.getByText(/^Generate Export$/i);
      expect(generateButton).toBeDisabled();
    });
  });

  it("should enable generate button when path is selected", async () => {
    const user = userEvent.setup();
    mockTauriDialog.save.mockResolvedValue("/path/to/export/questionnaire-export.zip");

    render(<ExportDialog />);

    // Open dialog
    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    await waitFor(() => {
      expect(screen.getByText(/Browse.../i)).toBeInTheDocument();
    });

    // Select path
    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      const generateButton = screen.getByText(/^Generate Export$/i);
      expect(generateButton).not.toBeDisabled();
    });
  });

  it("should call export API and show success toast when generate is clicked", async () => {
    const user = userEvent.setup();
    mockTauriDialog.save.mockResolvedValue("/path/to/export/questionnaire-export.zip");
    mockInvokeGenerateExportPack.mockResolvedValue({
      output_path: "/path/to/export/questionnaire-export.zip",
      file_count: 25,
    });

    render(<ExportDialog />);

    // Open dialog
    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    // Select path
    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      expect(
        screen.getByDisplayValue("/path/to/export/questionnaire-export.zip")
      ).toBeInTheDocument();
    });

    // Generate export
    const generateButton = screen.getByText(/^Generate Export$/i);
    await user.click(generateButton);

    await waitFor(() => {
      expect(mockInvokeGenerateExportPack).toHaveBeenCalledWith(
        "/path/to/export/questionnaire-export.zip"
      );
      expect(mockAddToast).toHaveBeenCalledWith({
        title: "Export Successful",
        description: "Export pack created with 25 files",
        variant: "success",
      });
    });
  });

  it("should show loading state during export", async () => {
    const user = userEvent.setup();
    mockTauriDialog.save.mockResolvedValue("/path/to/export/questionnaire-export.zip");
    mockInvokeGenerateExportPack.mockImplementation(
      () => new Promise((resolve) => setTimeout(resolve, 1000))
    );

    render(<ExportDialog />);

    // Open dialog
    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    // Select path
    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      expect(
        screen.getByDisplayValue("/path/to/export/questionnaire-export.zip")
      ).toBeInTheDocument();
    });

    // Click generate
    const generateButton = screen.getByText(/^Generate Export$/i);
    await user.click(generateButton);

    // Check loading state
    await waitFor(() => {
      expect(screen.getByText(/Exporting.../i)).toBeInTheDocument();
    });
  });

  it("should show error toast when export fails", async () => {
    const user = userEvent.setup();
    mockTauriDialog.save.mockResolvedValue("/path/to/export/questionnaire-export.zip");
    mockInvokeGenerateExportPack.mockRejectedValue(new Error("Export failed: Disk full"));

    render(<ExportDialog />);

    // Open dialog
    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    // Select path
    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      expect(
        screen.getByDisplayValue("/path/to/export/questionnaire-export.zip")
      ).toBeInTheDocument();
    });

    // Generate export
    const generateButton = screen.getByText(/^Generate Export$/i);
    await user.click(generateButton);

    await waitFor(() => {
      expect(mockAddToast).toHaveBeenCalledWith({
        title: "Export Failed",
        description: "Export failed: Disk full",
        variant: "destructive",
      });
    });
  });

  it("should close dialog and reset state after successful export", async () => {
    const user = userEvent.setup();
    mockTauriDialog.save.mockResolvedValue("/path/to/export/questionnaire-export.zip");
    mockInvokeGenerateExportPack.mockResolvedValue({
      output_path: "/path/to/export/questionnaire-export.zip",
      file_count: 25,
    });

    render(<ExportDialog />);

    // Open dialog
    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    // Select path
    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      expect(
        screen.getByDisplayValue("/path/to/export/questionnaire-export.zip")
      ).toBeInTheDocument();
    });

    // Generate export
    const generateButton = screen.getByText(/^Generate Export$/i);
    await user.click(generateButton);

    await waitFor(() => {
      expect(mockInvokeGenerateExportPack).toHaveBeenCalled();
    });

    // Dialog should close (content should not be visible)
    await waitFor(
      () => {
        expect(
          screen.queryByText(/This will create a ZIP file containing all questionnaire data/i)
        ).not.toBeInTheDocument();
      },
      { timeout: 3000 }
    );
  });

  it("should handle path selection cancellation", async () => {
    const user = userEvent.setup();
    mockTauriDialog.save.mockResolvedValue(null);

    render(<ExportDialog />);

    // Open dialog
    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    await waitFor(() => {
      expect(screen.getByText(/Browse.../i)).toBeInTheDocument();
    });

    // Click browse
    const browseButton = screen.getByText(/Browse.../i);
    await user.click(browseButton);

    await waitFor(() => {
      expect(mockTauriDialog.save).toHaveBeenCalled();
    });

    // Path should still be "No path selected"
    expect(screen.getByDisplayValue(/No path selected/i)).toBeInTheDocument();
  });

  it("should allow cancellation during export", async () => {
    const user = userEvent.setup();
    render(<ExportDialog />);

    // Open dialog
    const triggerButton = screen.getByText(/Generate Export Pack/i);
    await user.click(triggerButton);

    await waitFor(() => {
      expect(screen.getByText(/Cancel/i)).toBeInTheDocument();
    });

    // Click cancel
    const cancelButton = screen.getByText(/Cancel/i);
    await user.click(cancelButton);

    // Dialog should close
    await waitFor(() => {
      expect(
        screen.queryByText(/This will create a ZIP file containing all questionnaire data/i)
      ).not.toBeInTheDocument();
    });
  });
});
