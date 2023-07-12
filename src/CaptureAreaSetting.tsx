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

  const [firstLinetext, setFirstLineText] = createSignal("");
  const [secondLineText, setSecondLineText] = createSignal("");
  setFirstLineText("Double click: Set capture area");
  setSecondLineText("Long click: Reset capture area");

  window.addEventListener("dblclick", setCaptureArea);
  async function setCaptureArea() {
    setFirstLineText("Capture Area has set");
    const factor = await appWindow.scaleFactor();

    const physicalPosition = await appWindow.outerPosition();
    const logicalPosition = physicalPosition.toLogical(factor);

    const physicalSize = await appWindow.outerSize();
    const logicalSize = physicalSize.toLogical(factor);

    const x_1 = logicalPosition.x;
    const y_1 = logicalPosition.y;
    const x_2 = logicalPosition.x + logicalSize.width;
    const y_2 = logicalPosition.y + logicalSize.height;
    setSecondLineText(`[${x_1}, ${y_1}] - [${x_2}, ${y_1}]`);

    await invoke("set_capture_area", {
      topLeft: [x_1, y_1],
      bottomRight: [x_2, y_2],
    });
  }

  let longClickTimeout: NodeJS.Timeout;
  window.addEventListener("mousedown", (event) => {
    longClickTimeout = setTimeout(function () {
      invoke("init_capture_area");
      setFirstLineText("Capture Area has removed");
      setSecondLineText("");
    }, 1000);
  });

  window.addEventListener("mouseup", (event) => {
    clearTimeout(longClickTimeout);
  });

  return (
    <div style="display: flex; flex-flow: column; justify-content: center; align-items: center; height: 90vh;">
      <p class="fade-in">{firstLinetext()}</p>
      <p class="fade-in">{secondLineText()}</p>
    </div>
  );
}

render(
  () => <CaptureAreaSetting />,
  document.getElementById("root") as HTMLElement
);
