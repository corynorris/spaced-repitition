import { test, expect } from "@playwright/test";

test.describe("public pages", () => {
	test("home page loads", async ({ page }) => {
		const response = await page.goto("/");
		expect(response?.status()).toBe(200);
		await expect(page.locator("body")).not.toBeEmpty();
	});

	test("login page loads", async ({ page }) => {
		await page.goto("/login");
		await expect(page.locator("form")).toBeVisible();
	});

	test("register page loads", async ({ page }) => {
		await page.goto("/register");
		await expect(page.locator("form")).toBeVisible();
	});
});

test.describe("app redirects", () => {
	test("unauthenticated /app redirects to login", async ({ page }) => {
		const response = await page.goto("/app");
		// Should redirect to login or show error
		expect(response?.url()).toContain("/login");
	});
});
