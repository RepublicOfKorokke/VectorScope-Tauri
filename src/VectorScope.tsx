import { render } from "solid-js/web";
import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { register, unregisterAll } from "@tauri-apps/api/globalShortcut";

import "./styles.css";

const GLOBAL_SHORTCUT_KEY: string = "CommandOrControl+Shift+R";
const LISTEN_EVENT_NAME: string = "event-vector-scope";

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

export function Capture() {
  const [image, setImage] = createSignal("");

  appWindow.setTitle("Vector Scope");
  appWindow.setContentProtected(true);
  appWindow.setAlwaysOnTop(true);
  appWindow.setSize(new LogicalSize(300, 320));

  registerGlobalShortcutKey();

  listenCaptureScreen();
  listenCloseWindow();

  async function registerGlobalShortcutKey() {
    register(GLOBAL_SHORTCUT_KEY, () => {
      if (isListeningEmit) {
        invoke("stop_emit_vector_scope_image_as_payload");
        isListeningEmit = false;
      }

      invoke("get_vector_scope_image_as_payload").then((payload: any) => {
        let dataURI = payload.message as string;
        if (temporaryImage) objectURL.revokeObjectURL(temporaryImage);
        let imageDataBlob: Blob = convertDataURIToBlob(dataURI);
        temporaryImage = objectURL.createObjectURL(imageDataBlob);
        setImage(temporaryImage);
        dataURI = "";
      });
    });
  }

  async function listenCaptureScreen() {
    await listen(LISTEN_EVENT_NAME, (event: any) => {
      let dataURI = event.payload.message as string; // event.payload is payload
      if (temporaryImage) objectURL.revokeObjectURL(temporaryImage);
      let imageDataBlob: Blob = convertDataURIToBlob(dataURI);
      temporaryImage = objectURL.createObjectURL(imageDataBlob);
      setImage(temporaryImage);
      dataURI = "";
    });
    invoke("start_emit_vector_scope_image_as_payload");
    isListeningEmit = true;
  }

  async function listenCloseWindow() {
    const unlisten = await appWindow.onCloseRequested(async (event) => {
      invoke("stop_emit_vector_scope_image_as_payload");
      unregisterAll();
    });
  }

  return (
    <div>
      <img src={image()}></img>
    </div>
  );
}

render(() => <Capture />, document.getElementById("root") as HTMLElement);
