import { redirect } from "@tanstack/react-router";
import { useAuthStore } from "../auth/store";

export function protectRoute() {
  const user = useAuthStore.getState().user;
  console.log("user", user);
  if (!user?.token) {
    throw redirect({ to: "/login" });
  }
}

export function publicOnlyRoute() {
  const user = useAuthStore.getState().user;
  if (user?.token) {
    throw redirect({ to: "/" });
  }
}
