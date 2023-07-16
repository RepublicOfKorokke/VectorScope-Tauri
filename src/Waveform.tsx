import { render } from "solid-js/web";
import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { register, unregisterAll } from "@tauri-apps/api/globalShortcut";

import "./styles.css";
import "./reverse_image.css";

const LISTEN_EVENT_NAME: string = "event-waveform";

let isListeningEmit = false;

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

  appWindow.setTitle("Waveform");
  appWindow.setContentProtected(true);
  appWindow.setAlwaysOnTop(true);
  appWindow.setSize(new LogicalSize(1000, 255));

  registerGlobalShortcutKey();

  startListenWaveformImage();
  listenCloseWindow();

  window.addEventListener("dblclick", startListenWaveformImage);

  async function registerGlobalShortcutKey() {
    register(GLOBAL_SHORTCUT_KEY, () => {
      stopListenWaveformImage();
      invoke("get_vector_scope_image_as_payload").then((payload: any) => {
        let dataURI = payload.base64_waveform as string;
        if (temporaryImage) objectURL.revokeObjectURL(temporaryImage);
        let imageDataBlob: Blob = convertDataURIToBlob(dataURI);
        temporaryImage = objectURL.createObjectURL(imageDataBlob);
        setImage(temporaryImage);
        dataURI = "";
      });
    });
  }

  async function startListenWaveformImage() {
    if (!isListeningEmit) {
      await listen(LISTEN_EVENT_NAME, (event: any) => {
        let dataURI = event.payload.base64_waveform as string; // event.payload is payload
        if (temporaryImage) objectURL.revokeObjectURL(temporaryImage);
        let imageDataBlob: Blob = convertDataURIToBlob(dataURI);
        temporaryImage = objectURL.createObjectURL(imageDataBlob);
        setImage(temporaryImage);
        dataURI = "";
      });
      invoke("set_is_waveform_required", { state: true });
      isListeningEmit = true;
    }
  }

  async function stopListenWaveformImage() {
    if (isListeningEmit) {
      invoke("set_is_waveform_required", { state: false });
      isListeningEmit = false;
    }
  }

  async function listenCloseWindow() {
    await appWindow.onCloseRequested(async () => {
      invoke("set_is_waveform_required", { state: false });
      unregisterAll();
    });
  }

  return (
    <div>
      <img src={image()}></img>
    </div>
  );
}

render(() => <Waveform />, document.getElementById("root") as HTMLElement);
