import * as vscode from "vscode"
// import * as path from "path"

// function injectScript(extensionPath: string) {
  // const panel = vscode.window.createWebviewPanel("spiral", "Spiral", vscode.ViewColumn.Beside, {
  //   enableScripts: true,
  //   retainContextWhenHidden: true,
  // })

  // const scriptPathOnDisk = vscode.Uri.file(path.join(extensionPath, "media", "cellOutputScrollButtons.js"))
  // const scriptUri = panel.webview.asWebviewUri(scriptPathOnDisk)

  // panel.webview.html = `<!DOCTYPE html><html><body><script src="${scriptUri}"></script></body></html>`
// }

export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      if (editor && editor.document.languageId === "polyglot-notebook") {
        // injectScript(context.extensionPath)
      } else {
        console.log(
          `spiral-vscode.extension.activate / vscode.workspace.onDidChangeActiveTextEditor / editor: ${JSON.stringify(
            editor
          )}`
        )
      }
    })
  )

  vscode.workspace.onDidOpenTextDocument((document) => {
    if (document.languageId === "polyglot-notebook") {
    } else {
      console.log(
        `spiral-vscode.extension.activate / vscode.workspace.onDidOpenTextDocument / document: ${JSON.stringify(
          document
        )}`
      )
    }
  })
}

export function deactivate() {}
