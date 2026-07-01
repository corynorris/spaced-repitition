import { createRoute } from "@tanstack/react-router";
import { rootRoute } from "../../router";
import { publicOnlyRoute } from "../routes/protect";
import { LoginPage } from "./pages/LoginPage";
import { RegisterPage } from "./pages/RegisterPage";

export const authRoutes = {
  login: createRoute({
    getParentRoute: () => rootRoute,
    path: "/login",
    component: LoginPage,
    beforeLoad: publicOnlyRoute,
  }),
  register: createRoute({
    getParentRoute: () => rootRoute,
    path: "/register",
    component: RegisterPage,
    beforeLoad: publicOnlyRoute,
  }),
};
