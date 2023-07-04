import { render } from "solid-js/web";
import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

import "./styles.css";

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

  listenCaptureScreen();
  invoke("emit_capture_result");

  async function listenCaptureScreen() {
    await listen("event-capture-screen", (event: any) => {
      invoke("print_log", { text: "receive cature result" });
      let dataURI = event.payload.message as string;
      if (temporaryImage) objectURL.revokeObjectURL(temporaryImage);
      let imageDataBlob: Blob = convertDataURIToBlob(dataURI);
      temporaryImage = objectURL.createObjectURL(imageDataBlob);
      setImage(temporaryImage);
      dataURI = "";
      // repeat capturing
      invoke("emit_capture_result");
    });
  }

  return (
    <div>
      <img src={image()}></img>
    </div>
  );
}

render(() => <Capture />, document.getElementById("root") as HTMLElement);
