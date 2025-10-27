import { PlaywrightTestConfig, devices } from "@playwright/test"
import * as path from "path"

const PORT = process.env.PORT || 5000

const baseURL = `https://localhost:${PORT}`

export default {
  timeout: 60 * 1000,
  testDir: path.join(__dirname, "e2e"),
  retries: process.env.CI ? 1 : 0,
  outputDir: "test-results/",
  fullyParallel: true,
  reporter: "html",

  webServer: [
    {
      command: `cd ${__dirname}/dist && pwsh -c "$job = Start-Job { & ~/.bun/bin/bunx --bun ssl-serve --ssl }; Start-Sleep 60; Stop-Job $job | Out-Null; Receive-Job $job -Wait -AutoRemoveJob"`,
      url: baseURL,
      timeout: 60 * 1000,
      ignoreHTTPSErrors: true,
      reuseExistingServer: false, // !process.env.CI,
    },
  ],

  use: {
    baseURL,
    trace: { mode: "on-first-retry" },
    video: { mode: "on-first-retry" },
  },

  projects: [
    {
      name: "Desktop Chrome",
      use: {
        ...devices["Desktop Chrome"],
      },
    },
  ],
} as PlaywrightTestConfig
