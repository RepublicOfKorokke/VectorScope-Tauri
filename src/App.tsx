import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

async function createCaptureResultScreen() {
  invoke("create_capture_window");
}

export function App() {
  const [mouseX1, setMouseX1] = createSignal(0);
  const [mouseY1, setMouseY1] = createSignal(0);
  const [mouseX2, setMouseX2] = createSignal(0);
  const [mouseY2, setMouseY2] = createSignal(0);

  async function getMousePosition1() {
    let mousePosition: [number, number] = await invoke("get_mouse_position");
    setMouseX1(mousePosition[0]);
    setMouseY1(mousePosition[1]);
  }

  async function getMousePosition2() {
    let mousePosition: [number, number] = await invoke("get_mouse_position");
    setMouseX2(mousePosition[0]);
    setMouseY2(mousePosition[1]);
  }

  return (
    <>
      <div class="container">
        <button onclick={(e) => getMousePosition1()}>Top left</button>
        <button onclick={(e) => getMousePosition2()}>Bottom Right</button>
        <button onclick={(e) => createCaptureResultScreen()}>
          Take screenshot
        </button>
        <p>
          mouse position 1: {mouseX1()} / {mouseY1()}
          <br></br>
          mouse position 2: {mouseX2()} / {mouseY2()}
        </p>
      </div>
    </>
  );
}
