import * as wasm from "./pkg/spiral_temp_extension_bg.wasm"
import * as extension from "./pkg/spiral_temp_extension"

/// <reference types="chrome"/>

// import * as chrome from "chrome"
declare const chrome: any

function getBrowserInstance() {
  return chrome || (window as any).browser
}

const isExtension = typeof getBrowserInstance().runtime !== "undefined"

console.log("content_script.ts / isExtension:", isExtension)

const relativeWasmUrl: string = (wasm as any).default

const sendMessage = (data: any) =>
  new Promise((resolve) => getBrowserInstance().runtime.sendMessage(data, (response: unknown) => resolve(response)))

window.onload = async () => {
  console.log("content_script.ts / window.onload")

  const wasmUrl = isExtension ? getBrowserInstance().runtime.getURL(relativeWasmUrl) : relativeWasmUrl

  const wasmRequest = fetch(wasmUrl)
  try {
    const exports = await extension.default(wasmRequest)
    console.log("content_script.ts / exports:", exports)

    extension.app(window)
  } catch (error: any) {
    console.error("content_script.ts / error:", error)
    const rule = await sendMessage({ message: error.message })
    console.log("content_script.ts / rule:", rule)
  }
}

if (isExtension) {
  getBrowserInstance().runtime.onMessage.addListener((request: any, sender: any, _sendResponse: any) => {
    console.log("content_script.ts / onMessage / request:", request, "/ sender:", sender)
  })
}
