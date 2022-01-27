import Renderer from "./renderer";
import { resizeCanvasToDisplaySize } from "./utils";

import("fluid")
  .then(({ FluidConfig }) => {
    const canvas = document.getElementById("canvas") as HTMLCanvasElement;
    resizeCanvasToDisplaySize(canvas);
    const aspectRatio = canvas.width / canvas.height;
    const fluidConfig = FluidConfig.new(128 * aspectRatio, 128, 0.5);
    let renderer = new Renderer(fluidConfig, 0.6);
    renderer.start();
  })
  .catch((e) => console.error(e));
