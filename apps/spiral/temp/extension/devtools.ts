import { Buffer } from "buffer"

const attach = async (tabId: number) => {
  try {
    const target = (await chrome.debugger.getTargets().catch(() => [])).find((s) => s.tabId == tabId)
    if (target?.attached) {
      await chrome.debugger.detach({ tabId })
    }
  } catch (e) {
    console.log("attach / e:", e)
  }
  await chrome.debugger.attach(
    {
      tabId,
    },
    `1.3`
  )

  await chrome.debugger.sendCommand({ tabId }, `Fetch.enable`, {
    patterns: [
      {
        urlPattern: `http*`,
        resourceType: `Document`,
        requestStage: `Response`,
      },
    ],
  })

  const parser = new DOMParser()

  chrome.debugger.onEvent.addListener(async (source, method, _params) => {
    const params = _params as {
      requestId: string
      request: {
        url: string
        urlFragment?: string
        method: string
        headers: Record<string, string>
      }
      resourceType: string
      responseStatusCode?: number
      responseStatusText?: string
      responseHeaders?: { name: string; value: string }[]
      networkId?: string
      redirectedRequestId?: string
    }
    if (source.tabId != tabId || method != `Fetch.requestPaused`) return
    const { requestId } = params

    const response = (await chrome.debugger.sendCommand({ tabId }, `Fetch.getResponseBody`, { requestId })) as {
      body: string
      base64Encoded: boolean
    }
    let bodyText = response.base64Encoded ? Buffer.from(response.body, `base64`).toString("utf-8") : response.body

    const lowerText = bodyText.toLowerCase()
    console.log("attach / onEvent / lowerText:", lowerText)
    if (lowerText.includes(`content-security-policy`)) {
      let html: Document
      try {
        html = parser.parseFromString(bodyText, `text/html`)
      } catch (e) {
        html = {} as Document
      }
      if (html) {
        html.querySelectorAll(`meta`).forEach((meta) => {
          if (meta.getAttribute("http-equiv")?.toLowerCase() == "content-security-policy") {
            meta.remove()
          }
        })
        bodyText = html.documentElement.outerHTML
      }
    }

    await chrome.debugger.sendCommand({ tabId }, `Fetch.fulfillRequest`, {
      requestId,
      responseCode: params.responseStatusCode ?? 200,
      responseHeaders: params.responseHeaders,
      body: Buffer.from(bodyText, `utf-8`).toString(`base64`),
    })
  })
}

;(async () => {
  console.log("devtools")
  const { tabId } = chrome.devtools.inspectedWindow
  const tab = await chrome.tabs.get(tabId)
  console.log("devtools / tabId:", tabId, "/ tab:", tab)
  if (tab.url?.startsWith(`http`)) {
    await attach(tabId)
  }
})()
