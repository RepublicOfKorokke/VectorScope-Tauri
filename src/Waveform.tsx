import { render } from "solid-js/web";
import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { register, unregister } from "@tauri-apps/api/globalShortcut";

import "./styles.css";
import "./waveform.css";

const GLOBAL_SHORTCUT_KEY: string = "CommandOrControl+Shift+V";
const LISTEN_EVENT_NAME: string = "event-waveform";

// Methods to address the memory leaks problems in Safari
let BASE64_MARKER = ";base64,";
let temporaryImage: string;
let objectURL = window.URL || window.webkitURL;

let zoomed: boolean = false;

function convertDataURIToBlob(dataURI: string): Blob {
  // Convert image (in base64) to binary data
  let base64Index = dataURI.indexOf(BASE64_MARKER) + BASE64_MARKER.length;
  let base64 = dataURI.substring(base64Index);

  let raw = atob(base64);
  let rawLength = raw.length;
  let array = new Uint8Array(new ArrayBuffer(rawLength));

  for (let i = 0; i < rawLength; i++) {
    array[i] = raw.charCodeAt(i);
  }

  // Create and return a new blob object using binary data
  return new Blob([array], { type: "image/png" });
}

export function Waveform() {
  const [image, setImage] = createSignal("");
  const [width, setWidth] = createSignal("95vw");
  const [height, setHeight] = createSignal("95vh");

  initializeWindow();
  registerGlobalShortcutKey();
  listenCloseWindow();
  window.addEventListener("dblclick", () => setManualModeOn(false));
  window.addEventListener("click", setImageSize);

  async function initializeWindow() {
    appWindow.setTitle("Waveform");
    appWindow.setContentProtected(true);
    appWindow.setAlwaysOnTop(true);
    appWindow.setSize(new LogicalSize(500, 280));

    await listen(LISTEN_EVENT_NAME, (event: any) => {
      let dataURI = event.payload as string; // event.payload is payload
      if (temporaryImage) objectURL.revokeObjectURL(temporaryImage);
      let imageDataBlob: Blob = convertDataURIToBlob(dataURI);
      temporaryImage = objectURL.createObjectURL(imageDataBlob);
      setImage(temporaryImage);
      dataURI = "";
    });
    setIsWaveformWindowOpen(true);
    setManualModeOn(false);
  }

  async function registerGlobalShortcutKey() {
    register(GLOBAL_SHORTCUT_KEY, () => {
      setManualModeOn(true);
      invoke("one_shot_emit");
    });
  }

  async function listenCloseWindow() {
    await appWindow.onCloseRequested(async () => {
      setIsWaveformWindowOpen(false);
      unregister(GLOBAL_SHORTCUT_KEY);
    });
  }

  async function setIsWaveformWindowOpen(open: boolean) {
    invoke("set_is_waveform_window_open", { state: open });
  }

  async function setManualModeOn(state: boolean) {
    invoke("set_manual_mode", { state: state });
  }

  async function setImageSize() {
    if (zoomed) {
      setWidth("95vw");
      setHeight("95vh");
    } else {
      setWidth("auto");
      setHeight("auto");
    }
    zoomed = !zoomed;
  }

  return (
    <div>
      <img
        src={image()}
        style={{
          width: `${width()}`,
          height: `${height()}`,
          "-webkit-transform": `scaleY(-1)`,
          transform: `scaleY(-1)`,
        }}
      ></img>
    </div>
  );
}

render(() => <Waveform />, document.getElementById("root") as HTMLElement);
