const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

let greetInputEl;
let parentContainer;
let kana_array; 
let converType;
let alwaytop;
let autocopy;

async function to_kana_js() {
  // Remove all child elements from the parent container
  while (parentContainer.firstChild) {
    parentContainer.removeChild(parentContainer.firstChild);
  }
  try {
    kana_array = await invoke("to_kana_fn", { name: greetInputEl.value, convertType: converType.value });
    // auto copy first response
    if (autocopy.checked) {
      copyTextToClipboard(kana_array[0]);
    }
    if (kana_array[0] != '') {
      kana_array.forEach((item, idx) => {
        const divElement = document.createElement('div');
        divElement.textContent = item;
        divElement.addEventListener("click", () => {
          copyTextToClipboard(item);
        })
        parentContainer.appendChild(divElement);
      })
    }
  } catch (error) {
    console.error("An error occurred:", error);
  }
}

function copyTextToClipboard(text) {
  // Function to copy text to clipboard using the Clipboard API
  if ('clipboard' in navigator) {
    navigator.clipboard.writeText(text);
  } else {
    console.warn('Clipboard API not supported in this browser.');
  }
}

async function setWindowAlwaysOnTop(state) {
  // set window top
  try {
    await appWindow.setAlwaysOnTop(state);
    console.log("Window is now set to always be on top.");
  } catch (error) {
    console.error("Error setting window always on top:", error);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  parentContainer = document.getElementById('parent-container');
  converType = document.getElementById('conversion-type');
  alwaytop = document.getElementById('alwaytop');
  autocopy = document.getElementById('autocopy');
  // at first
  setWindowAlwaysOnTop(alwaytop.checked);
  
  // update when change conversion-type
  converType.addEventListener("change", (e) => {
    e.preventDefault();
    to_kana_js();
  })
  // alway on top
  alwaytop.addEventListener("click", () => {
    setWindowAlwaysOnTop(alwaytop.checked);
  })
  
  document.querySelector("#greet-form").addEventListener("keyup", (e) => {
    e.preventDefault();
    to_kana_js();
  });
});
