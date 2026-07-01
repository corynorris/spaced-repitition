import { isDev } from "@/utils/env";
import { create } from "zustand";
import { devtools, persist } from "zustand/middleware";
import { User } from "./types";

interface AuthState {
  user: User | null;
  setUser: (user: User | null) => void;
}

export const useAuthStore = create<AuthState>()(
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (isDev ? devtools : (fn: any) => fn)(
    persist(
      (set) => ({
        user: null,
        setUser: (user: User | null) => set({ user }),
      }),
      {
        name: "auth-storage",
        partialize: (state: AuthState) => ({ user: state.user }),
      },
    ),
    {
      name: "Auth Store",
    },
  ),
);
