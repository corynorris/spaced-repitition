import { useNavigate, useRouter } from "@tanstack/react-router";
import { useAuthBase } from "./useAuthBase";

export function useAuth() {
  const auth = useAuthBase();
  const router = useRouter();
  const navigate = useNavigate();

  const login: typeof auth.login = async (...args) => {
    await auth.login(...args);
    setTimeout(() => {
      navigate({ to: "/" });
    }, 1000);
  };

  const register: typeof auth.register = async (...args) => {
    await auth.register(...args);
    navigate({ to: "/" });
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
