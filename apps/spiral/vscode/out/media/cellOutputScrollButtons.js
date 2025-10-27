"use strict";

// media/cellOutputScrollButtons.ts
function addScrollButtons(container) {
  if (container.querySelector(".scroll-to-top-button")) return;
  console.log(`spiral-vscode.cellOutputScrollButtons / addScrollButtons`);
  const scrollToTopButton = document.createElement("button");
  scrollToTopButton.innerHTML = "\u2B06\uFE0F Top";
  scrollToTopButton.classList.add("scroll-to-top-button");
  scrollToTopButton.style.position = "absolute";
  scrollToTopButton.style.bottom = "10px";
  scrollToTopButton.style.right = "50px";
  scrollToTopButton.onclick = () => {
    container.scrollTop = 0;
  };
  container.appendChild(scrollToTopButton);
  const scrollToBottomButton = document.createElement("button");
  scrollToBottomButton.innerHTML = "\u2B07\uFE0F Bottom";
  scrollToBottomButton.classList.add("scroll-to-bottom-button");
  scrollToBottomButton.style.position = "absolute";
  scrollToBottomButton.style.top = "10px";
  scrollToBottomButton.style.right = "50px";
  scrollToBottomButton.onclick = () => {
    container.scrollTop = container.scrollHeight;
  };
  container.appendChild(scrollToBottomButton);
}
function observeContainer() {
  console.log(`spiral-vscode.cellOutputScrollButtons / observeContainer`);
  const observer = new MutationObserver((mutations) => {
    for (const mutation of mutations) {
      if (mutation.type === "childList") {
        mutation.addedNodes.forEach((node) => {
          if (node.nodeType === Node.ELEMENT_NODE) {
            const el = node;
            if (el.classList.contains("codeOutput-focus-indicator-container")) {
              addScrollButtons(el);
            }
          }
        });
      }
    }
  });
  observer.observe(document.body, {
    childList: true,
    subtree: true
  });
}
console.log(`spiral-vscode.cellOutputScrollButtons / before window.addEventListener("load")`);
window.addEventListener("load", observeContainer);
