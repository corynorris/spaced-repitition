import { createRoute } from "@tanstack/react-router";
import { rootRoute } from "../../router";
import { protectRoute } from "../routes/protect";
import { DashboardPage } from "./pages/DashboardPage";

export const dashboardRoutes = {
  dashboard: createRoute({
    path: "/",
    component: DashboardPage,
    beforeLoad: protectRoute,
    getParentRoute: () => rootRoute,
  }),
};
