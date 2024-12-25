import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useAuthStore } from "../store";
import { User } from "../types";

interface LoginCredentials {
  email: string;
  password: string;
}

interface RegisterCredentials extends LoginCredentials {
  username: string;
}

interface ApiResponse {
  user: User;
}

export function useAuthBase() {
  const { user: storedUser, setUser } = useAuthStore();
  const queryClient = useQueryClient();

  const { data: user, isLoading } = useQuery({
    queryKey: ["user"],
    queryFn: async () => {
      if (!storedUser?.token) return null;

      const response = await fetch("/api/user", {
        headers: {
          Authorization: `Bearer ${storedUser.token}`,
        },
      });

      if (!response.ok) {
        setUser(null);
        throw new Error("Failed to fetch user");
      }

      const data = (await response.json()) as ApiResponse;
      return data.user;
    },
    enabled: !!storedUser?.token,
    retry: (failureCount, error) => {
      if (error instanceof Error && error.message === "Failed to fetch user") {
        return false;
      }
      return failureCount < 2;
    },
    initialData: storedUser,
  });

  const login = useMutation({
    mutationFn: async (credentials: LoginCredentials) => {
      const response = await fetch("/api/users/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ user: credentials }),
      });

      if (!response.ok) {
        throw new Error("Login failed");
      }

      const data = (await response.json()) as ApiResponse;
      return data.user;
    },
    onSuccess: (data) => {
      setUser(data);
      queryClient.setQueryData(["user"], data);
    },
  });

  const register = useMutation({
    mutationFn: async (credentials: RegisterCredentials) => {
      const response = await fetch("/api/users", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ user: credentials }),
      });

      if (!response.ok) {
        throw new Error("Registration failed");
      }

      const data = (await response.json()) as ApiResponse;
      return data.user;
    },
    onSuccess: (data) => {
      setUser(data);
      queryClient.setQueryData(["user"], data);
    },
  });

  const clearUser = () => {
    setUser(null);
    queryClient.setQueryData(["user"], null);
  };

  return {
    user: user ?? null,
    isAuthenticated: !!user,
    loading: isLoading && !!storedUser?.token,
    login: login.mutate,
    register: register.mutate,
    clearUser,
  } as const;
}
