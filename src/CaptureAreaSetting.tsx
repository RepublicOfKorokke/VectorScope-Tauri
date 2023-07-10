import { render } from "solid-js/web";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";

import "./styles.css";

async function getWIndowPosition() {
  const factor = await appWindow.scaleFactor();

  const physicalPosition = await appWindow.outerPosition();
  const logicalPosition = physicalPosition.toLogical(factor);

  const physicalSize = await appWindow.outerSize();
  const logicalSize = physicalSize.toLogical(factor);

  const x_1 = logicalPosition.x;
  const y_1 = logicalPosition.y;
  const x_2 = logicalPosition.x + logicalSize.width;
  const y_2 = logicalPosition.y + logicalSize.height;

  invoke("print_log", {
    text: `logical window top left ${x_1} / ${y_1}`,
  });
  invoke("print_log", {
    text: `logical window bottom right ${x_2} / ${y_2}`,
  });
}

export function CaptureAreaSetting() {
  appWindow.setTitle("Capture Area Setting");
  appWindow.setAlwaysOnTop(true);
  window.addEventListener("dblclick", getWIndowPosition);

  return (
    <div>
      <p> Double click window to set capture area</p>
    </div>
  );
}

render(
  () => <CaptureAreaSetting />,
  document.getElementById("root") as HTMLElement
);
