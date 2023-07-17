import { render } from "solid-js/web";
import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { register, unregisterAll } from "@tauri-apps/api/globalShortcut";

import "./styles.css";
import "./waveform.css";

const GLOBAL_SHORTCUT_KEY: string = "CommandOrControl+Shift+V";
const LISTEN_EVENT_NAME: string = "event-waveform";

// Methods to address the memory leaks problems in Safari
let BASE64_MARKER = ";base64,";
let temporaryImage: string;
let objectURL = window.URL || window.webkitURL;

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

  initializeWindow();
  registerGlobalShortcutKey();
  listenCloseWindow();
  window.addEventListener("dblclick", () => setManualModeOn(false));

  async function initializeWindow() {
    appWindow.setTitle("Waveform");
    appWindow.setContentProtected(true);
    appWindow.setAlwaysOnTop(true);
    appWindow.setSize(new LogicalSize(1000, 255));

    await listen(LISTEN_EVENT_NAME, (event: any) => {
      let dataURI = event.payload as string; // event.payload is payload
      if (temporaryImage) objectURL.revokeObjectURL(temporaryImage);
      let imageDataBlob: Blob = convertDataURIToBlob(dataURI);
      temporaryImage = objectURL.createObjectURL(imageDataBlob);
      setImage(temporaryImage);
      dataURI = "";
    });
    setWaveformRequired(true);
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
      setWaveformRequired(false);
    });
  }

  async function setWaveformRequired(state: boolean) {
    invoke("set_is_waveform_window_open", { state: state });
  }

  async function setManualModeOn(state: boolean) {
    invoke("set_manual_mode", { state: state });
  }

  return (
    <div>
      <img src={image()}></img>
    </div>
  );
}

render(() => <Waveform />, document.getElementById("root") as HTMLElement);
