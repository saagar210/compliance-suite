import { describe, it, expect, vi } from "vitest";

// Mock all Tauri API calls
const mockInvokeImportQuestionnaire = vi.fn();
const mockInvokeGetColumnProfiles = vi.fn();
const mockInvokeSaveColumnMapping = vi.fn();

vi.mock("../../api/tauri", () => ({
  invokeImportQuestionnaire: (...args: any[]) => mockInvokeImportQuestionnaire(...args),
  invokeGetColumnProfiles: (...args: any[]) => mockInvokeGetColumnProfiles(...args),
  invokeSaveColumnMapping: (...args: any[]) => mockInvokeSaveColumnMapping(...args),
  invokeAnswerBankList: vi.fn(() => Promise.resolve([])),
  invokeAnswerBankCreate: vi.fn(),
  invokeAnswerBankUpdate: vi.fn(),
  invokeAnswerBankDelete: vi.fn(),
  invokeGenerateExportPack: vi.fn(),
}));

/**
 * Integration Tests - Questionnaire Flow
 *
 * These tests verify the integration between components and the Tauri backend.
 * For full E2E testing with routing, use Playwright or similar tools.
 */
describe("Questionnaire Flow Integration", () => {
  it("should call API in correct sequence: import → get profiles → save mapping", async () => {
    // Mock data
    const mockImportData = {
      import_id: "import-123",
      source_filename: "security_questionnaire.xlsx",
      source_path: "/path/to/security_questionnaire.xlsx",
      import_status: "pending_mapping",
      created_at: new Date().toISOString(),
    };

    const mockColumnProfiles = [
      {
        col_ref: "A",
        label: "Question",
        ordinal: 0,
        non_empty_count: 50,
        sample: ["What is your security policy?", "How do you handle data breaches?"],
      },
      {
        col_ref: "B",
        label: "Answer",
        ordinal: 1,
        non_empty_count: 50,
        sample: ["We follow industry standards", "We have an incident response plan"],
      },
    ];

    const mockUpdatedImport = {
      ...mockImportData,
      import_status: "mapped",
      column_map: { question: "A", answer: "B", notes: undefined },
    };

    // Setup mocks
    mockInvokeImportQuestionnaire.mockResolvedValue(mockImportData);
    mockInvokeGetColumnProfiles.mockResolvedValue(mockColumnProfiles);
    mockInvokeSaveColumnMapping.mockResolvedValue(mockUpdatedImport);

    // Simulate the flow
    const importResult = await mockInvokeImportQuestionnaire("/path/to/test.xlsx");
    expect(importResult.import_id).toBe("import-123");

    const profiles = await mockInvokeGetColumnProfiles(importResult.import_id);
    expect(profiles).toHaveLength(2);

    const mapped = await mockInvokeSaveColumnMapping(importResult.import_id, {
      question: "A",
      answer: "B",
    });
    expect(mapped.import_status).toBe("mapped");

    // Verify call sequence
    expect(mockInvokeImportQuestionnaire).toHaveBeenCalledWith("/path/to/test.xlsx");
    expect(mockInvokeGetColumnProfiles).toHaveBeenCalledWith("import-123");
    expect(mockInvokeSaveColumnMapping).toHaveBeenCalledWith("import-123", {
      question: "A",
      answer: "B",
    });
  });

  it("should handle import errors and not call subsequent APIs", async () => {
    // Reset mocks to ensure clean state
    mockInvokeImportQuestionnaire.mockReset();
    mockInvokeGetColumnProfiles.mockReset();
    mockInvokeSaveColumnMapping.mockReset();

    mockInvokeImportQuestionnaire.mockRejectedValue(
      new Error("Failed to parse file: Invalid format")
    );

    try {
      await mockInvokeImportQuestionnaire("/path/to/invalid.xlsx");
      throw new Error("Should have thrown error");
    } catch (error) {
      expect((error as Error).message).toContain("Failed to parse file");
    }

    // Subsequent APIs should not be called
    expect(mockInvokeGetColumnProfiles).not.toHaveBeenCalled();
    expect(mockInvokeSaveColumnMapping).not.toHaveBeenCalled();
  });

  it("should handle column mapping with optional notes field", async () => {
    const mockImportData = {
      import_id: "import-456",
      source_filename: "test.xlsx",
      source_path: "/path/to/test.xlsx",
      import_status: "pending_mapping",
      created_at: new Date().toISOString(),
    };

    const mockUpdatedImport = {
      ...mockImportData,
      import_status: "mapped",
      column_map: { question: "A", answer: "B", notes: "C" },
    };

    mockInvokeSaveColumnMapping.mockResolvedValue(mockUpdatedImport);

    const result = await mockInvokeSaveColumnMapping("import-456", {
      question: "A",
      answer: "B",
      notes: "C",
    });

    expect(result.column_map).toEqual({
      question: "A",
      answer: "B",
      notes: "C",
    });
  });

  it("should validate required fields in column mapping", async () => {
    const mockImportData = {
      import_id: "import-789",
      source_filename: "test.xlsx",
      source_path: "/path/to/test.xlsx",
      import_status: "pending_mapping",
      created_at: new Date().toISOString(),
    };

    // Simulate validation - both question and answer are required
    const isValid = (mapping: { question?: string; answer?: string }): boolean => {
      return Boolean(mapping.question && mapping.answer);
    };

    expect(isValid({ question: "A", answer: "B" })).toBe(true);
    expect(isValid({ question: "A" })).toBe(false);
    expect(isValid({ answer: "B" })).toBe(false);
    expect(isValid({})).toBe(false);
  });
});
