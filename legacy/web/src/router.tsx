import { createRootRoute, createRouter, Outlet } from "@tanstack/react-router";
import { authRoutes } from "./features/auth/routes";
import { dashboardRoutes } from "./features/dashboard/routes";

const basepath =
  import.meta.env.BASE_URL === "/"
    ? "/"
    : import.meta.env.BASE_URL.replace(/\/$/, "");

export const rootRoute = createRootRoute({
  component: () => <Outlet />,
});

const routeTree = rootRoute.addChildren([
  authRoutes.login,
  authRoutes.register,
  dashboardRoutes.dashboard,
]);

export const router = createRouter({
  routeTree,
  basepath,
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
