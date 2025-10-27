/// <reference types="chrome"/>

// import * as chrome from "chrome"
// declare const chrome: any

function getBrowserInstance() {
  return chrome || (window as any).browser
}

const isExtension = typeof getBrowserInstance().runtime !== "undefined"

console.log("service_worker.ts / isExtension:", isExtension)

self.onmessage = async (event) => {
  console.log("service_worker.ts / onmessage / event:", event)
}

function stringToId(str: string): number {
  let hash = 42
  for (let i = 0; i < str.length; i++) {
    hash = (hash << 5) + hash + str.charCodeAt(i)
  }
  return Math.round(Math.abs(hash / 6))
}

function generateUnblockRule(filter: string): chrome.declarativeNetRequest.Rule {
  return {
    id: stringToId(filter),
    priority: 1,
    action: {
      type: chrome.declarativeNetRequest.RuleActionType.MODIFY_HEADERS,
      responseHeaders: [
        {
          header: "Content-Security-Policy",
          operation: chrome.declarativeNetRequest.HeaderOperation.SET,
          // value: "script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval'; script-src-elem 'self' 'unsafe-inline' * data:; object-src 'self'; default-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' * data:; font-src 'self' *; connect-src 'self' *; media-src 'self' * blob:;",
          value: "default-src * 'unsafe-inline' 'unsafe-eval' 'wasm-unsafe-eval' 'wasm-eval' data: blob:;",
        },
        { header: "content-security-policy-report-only", operation: chrome.declarativeNetRequest.HeaderOperation.REMOVE },
        { header: "Referrer-Policy", operation: chrome.declarativeNetRequest.HeaderOperation.REMOVE },
        { header: "x-content-security-policy", operation: chrome.declarativeNetRequest.HeaderOperation.REMOVE },
        { header: "X-Frame-Options", operation: chrome.declarativeNetRequest.HeaderOperation.REMOVE },
        { header: "x-webkit-csp", operation: chrome.declarativeNetRequest.HeaderOperation.REMOVE },
      ],
    },
    condition: {
      urlFilter: `*${filter}*`,
      resourceTypes: [
        chrome.declarativeNetRequest.ResourceType.MAIN_FRAME,
        chrome.declarativeNetRequest.ResourceType.SUB_FRAME,
      ],
    },
  }
}

chrome.runtime.onInstalled.addListener(() => {
  console.log("service_worker.ts / chrome.runtime.onInstalled")
})

let reloadCount: { [tabId: number]: number } = {}

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  console.log(
    "service_worker.js / onMessage / request:",
    request,
    "/ sender:",
    sender,
    "/ reloadCount:",
    sender.tab?.id && reloadCount[sender.tab.id]
  )

  if (request.message && request.message.includes("WebAssembly")) {
    let filter = sender.url || sender.tab?.url || sender.origin || ""
    const matches = filter.match(/^https?\:\/\/([^\/?#]+)(?:[\/?#]|$)/i)
    filter = (matches && matches[1]) || ""
    if (!filter) {
      console.log("service_worker.js / onMessage / no filter")
    } else {
      ;(async () => {
        const rule = generateUnblockRule(filter)

        await chrome.declarativeNetRequest.updateDynamicRules({ removeRuleIds: [rule.id], addRules: [rule] })

        if (chrome.runtime.lastError) {
          console.log(
            "service_worker.js / onMessage / updateDynamicRules / rule:",
            rule,
            "/ error:",
            chrome.runtime.lastError
          )
        } else {
          console.log("service_worker.js / onMessage / updateDynamicRules / rule:", rule)
        }

        if (sender.tab?.id) {
          reloadCount[sender.tab.id] = (reloadCount[sender.tab.id] || 0) + 1
          if (reloadCount[sender.tab.id] < 2) {
            await chrome.tabs.reload(sender.tab.id)
          }
        }

        sendResponse({ rule })
      })()
      return true
    }
  }
})
