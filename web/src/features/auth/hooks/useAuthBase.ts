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

interface GraphQLUser {
  userId: string;
  email: string;
  username: string;
  role: "ADMIN" | "USER";
}

interface AuthPayload {
  user: GraphQLUser;
  token: string;
}

interface GraphQLResponse<T> {
  data?: T;
  errors?: Array<{ message: string }>;
}

const GRAPHQL_URL = "/api/graphql";

/**
 * Executes a GraphQL request, returning data or throwing on GraphQL/server errors.
 */
async function graphqlRequest<T>(query: string, variables?: Record<string, unknown>, token?: string): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const response = await fetch(GRAPHQL_URL, {
    method: "POST",
    headers,
    body: JSON.stringify({ query, variables }),
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}: GraphQL request failed`);
  }

  const result = (await response.json()) as GraphQLResponse<T>;

  if (result.errors?.length) {
    throw new Error(result.errors[0].message);
  }

  if (!result.data) {
    throw new Error("No data returned from GraphQL query");
  }

  return result.data;
}

function graphqlUserToAppUser(gqlUser: GraphQLUser, token: string): User {
  return {
    email: gqlUser.email,
    username: gqlUser.username,
    token,
    created_at: undefined,
  };
}

export function useAuthBase() {
  const { user: storedUser, setUser } = useAuthStore();
  const queryClient = useQueryClient();

  // Fetch current user (me query)
  const { data: user, isLoading } = useQuery({
    queryKey: ["user"],
    queryFn: async () => {
      if (!storedUser?.token) return null;

      const data = await graphqlRequest<{
        users: { me: GraphQLUser | null };
      }>(
        `query { users { me { userId email username role } } }`,
        undefined,
        storedUser.token,
      );

      if (!data.users.me) {
        setUser(null);
        return null;
      }

      return graphqlUserToAppUser(data.users.me, storedUser.token);
    },
    enabled: !!storedUser?.token,
    retry: (failureCount) => failureCount < 2,
    initialData: storedUser,
  });

  // Login mutation
  const login = useMutation({
    mutationFn: async (credentials: LoginCredentials) => {
      const data = await graphqlRequest<{
        users: { login: AuthPayload };
      }>(
        `mutation($input: LoginInput!) {
          users { login(input: $input) { user { userId email username role } token } }
        }`,
        { input: credentials },
      );

      return data.users.login;
    },
    onSuccess: (data) => {
      const appUser = graphqlUserToAppUser(data.user, data.token);
      setUser(appUser);
      queryClient.setQueryData(["user"], appUser);
    },
  });

  // Register mutation
  const register = useMutation({
    mutationFn: async (credentials: RegisterCredentials) => {
      const data = await graphqlRequest<{
        users: { register: AuthPayload };
      }>(
        `mutation($input: CreateUserInput!) {
          users { register(input: $input) { user { userId email username role } token } }
        }`,
        { input: credentials },
      );

      return data.users.register;
    },
    onSuccess: (data) => {
      // Auto-login after registration
      const appUser = graphqlUserToAppUser(data.user, data.token);
      setUser(appUser);
      queryClient.setQueryData(["user"], appUser);
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
    login: login.mutateAsync,
    register: register.mutateAsync,
    clearUser,
  } as const;
}
