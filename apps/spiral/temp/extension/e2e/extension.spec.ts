import { test, expect } from "./fixtures"

test("popup localhost", async ({ browser }) => {
  const context = await browser.newContext({ ignoreHTTPSErrors: true });
  const page = await context.newPage();
  await page.goto("https://localhost:5000/popup.html")
  await expect(page.locator("body")).toHaveText(/Loading\.\.\..*?/)
})

test("popup extension", async ({ page, extensionId }) => {
  await page.goto(`chrome-extension://${extensionId}/popup.html`)
  await expect(page.locator("body")).toHaveText(/Loading\.\.\..*?/)
})

test("libgen", async ({ page }) => {
  await page.goto("https://libgen.is")
  await page.type("#searchform", "aaA")
  await expect(page.locator("#searchform")).toHaveValue("AAA")
})
