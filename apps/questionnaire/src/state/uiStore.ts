import { create } from "zustand";

interface Toast {
  id: string;
  title: string;
  description?: string;
  variant: "default" | "destructive" | "success";
}

interface Modal {
  id: string;
  title: string;
  content: React.ReactNode;
  onClose: () => void;
}

interface UiState {
  // Toast notifications
  toasts: Toast[];

  // Modals
  modals: Modal[];

  // Sidebar
  sidebarOpen: boolean;

  // Global loading state
  globalLoading: boolean;

  // Actions
  addToast: (toast: Omit<Toast, "id">) => void;
  removeToast: (id: string) => void;
  openModal: (modal: Omit<Modal, "id">) => void;
  closeModal: (id: string) => void;
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
  setGlobalLoading: (loading: boolean) => void;
}

export const useUiStore = create<UiState>((set) => ({
  toasts: [],
  modals: [],
  sidebarOpen: true,
  globalLoading: false,

  addToast: (toast) =>
    set((state) => ({
      toasts: [...state.toasts, { ...toast, id: Math.random().toString(36) }],
    })),

  removeToast: (id) =>
    set((state) => ({
      toasts: state.toasts.filter((t) => t.id !== id),
    })),

  openModal: (modal) =>
    set((state) => ({
      modals: [...state.modals, { ...modal, id: Math.random().toString(36) }],
    })),

  closeModal: (id) =>
    set((state) => ({
      modals: state.modals.filter((m) => m.id !== id),
    })),

  toggleSidebar: () =>
    set((state) => ({
      sidebarOpen: !state.sidebarOpen,
    })),

  setSidebarOpen: (sidebarOpen) => set({ sidebarOpen }),

  setGlobalLoading: (globalLoading) => set({ globalLoading }),
}));
