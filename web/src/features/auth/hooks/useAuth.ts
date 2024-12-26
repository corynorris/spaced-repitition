import { useNavigate, useRouter } from "@tanstack/react-router";
import { LoginCredentials, RegisterCredentials } from "../types";
import { useAuthBase } from "./useAuthBase";

export function useAuth() {
  const auth = useAuthBase();
  const router = useRouter();
  const navigate = useNavigate();

  const login = async (credentials: LoginCredentials) => {
    const user = await auth.login(credentials);
    navigate({ to: "/" });
    return user;
  };

  const register = async (credentials: RegisterCredentials) => {
    const user = await auth.register(credentials);
    navigate({ to: "/" });
    return user;
  };

  const logout = async () => {
    auth.clearUser();
    await router.invalidate();
    navigate({ to: "/login" });
  };

  return {
    ...auth,
    login,
    register,
    logout,
  } as const;
}
