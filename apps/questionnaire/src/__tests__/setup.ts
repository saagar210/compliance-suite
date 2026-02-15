import { expect, afterEach, vi } from "vitest";
import { cleanup } from "@testing-library/react";
import "@testing-library/jest-dom";

// Cleanup after each test
afterEach(() => {
  cleanup();
});

// Mock Tauri API
const mockTauriInvoke = vi.fn();
const mockTauriDialog = {
  open: vi.fn(),
  save: vi.fn(),
};

// Mock @tauri-apps/api
vi.mock("@tauri-apps/api/core", () => ({
  invoke: mockTauriInvoke,
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: mockTauriDialog.open,
  save: mockTauriDialog.save,
}));

// Reset mocks before each test
beforeEach(() => {
  mockTauriInvoke.mockReset();
  mockTauriDialog.open.mockReset();
  mockTauriDialog.save.mockReset();
});

// Export mocks for tests
export { mockTauriInvoke, mockTauriDialog };
