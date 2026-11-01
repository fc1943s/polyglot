import * as playwright from "@playwright/test"
import * as path from "path"

export const test = playwright.test.extend<{
  context: playwright.BrowserContext
  extensionId: string
}>({
  context: async ({}, use) => {
    const pathToExtension = path.join(__dirname, "../dist")
    const context = await playwright.chromium.launchPersistentContext("", {
      recordVideo: {
        dir: path.join(__dirname, "../test-results/videos"),
      },
      headless: false,
      args: [`--headless=new`, `--disable-extensions-except=${pathToExtension}`, `--load-extension=${pathToExtension}`],
    })
    await use(context)
    await context.close()
  },
  extensionId: async ({ context }, use) => {
    let [background] = context.serviceWorkers()
    if (!background) background = await context.waitForEvent("serviceworker")

    const extensionId = background.url().split("/")[2]
    await use(extensionId)
  },
})
export const expect = playwright.test.expect
