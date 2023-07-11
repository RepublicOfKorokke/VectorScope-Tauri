import { render } from "solid-js/web";
import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";

import "./styles.css";

export function CaptureAreaSetting() {
  appWindow.setTitle("Capture Area Setting");
  appWindow.setContentProtected(true);
  appWindow.setAlwaysOnTop(true);
  appWindow.setSize(new LogicalSize(500, 500));

  const [text, setText] = createSignal("");
  setText("Double click window to set capture area");

  window.addEventListener("dblclick", setCaptureArea);
  async function setCaptureArea() {
    setText("Capture Area has set");
    const factor = await appWindow.scaleFactor();

    const physicalPosition = await appWindow.outerPosition();
    const logicalPosition = physicalPosition.toLogical(factor);

    const physicalSize = await appWindow.outerSize();
    const logicalSize = physicalSize.toLogical(factor);

    const x_1 = logicalPosition.x;
    const y_1 = logicalPosition.y;
    const x_2 = logicalPosition.x + logicalSize.width;
    const y_2 = logicalPosition.y + logicalSize.height;

    await invoke("set_capture_area", {
      topLeft: [x_1, y_1],
      bottomRight: [x_2, y_2],
    });
  }

  let longClickTimeout: NodeJS.Timeout;
  window.addEventListener("mousedown", (event) => {
    longClickTimeout = setTimeout(function () {
      invoke("init_capture_area");
      setText("Capture Area has removed");
    }, 1000);
  });

  window.addEventListener("mouseup", (event) => {
    clearTimeout(longClickTimeout);
  });

  return (
    <div style="display: flex; justify-content: center; align-items: center; height: 90vh;">
      <p>{text()}</p>
    </div>
  );
}

render(
  () => <CaptureAreaSetting />,
  document.getElementById("root") as HTMLElement
);
