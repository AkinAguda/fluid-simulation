import Renderer from "./renderer";
import { resizeCanvasToDisplaySize } from "./utils";

const DEFAULT = 128;

import("fluid")
  .then(({ FluidConfig }) => {
    const canvas = document.getElementById("canvas") as HTMLCanvasElement;
    resizeCanvasToDisplaySize(canvas);
    const aspectRatio = canvas.width / canvas.height;
    const fluidConfig = FluidConfig.new(DEFAULT, DEFAULT / aspectRatio, 0.5);
    let renderer = new Renderer(fluidConfig, 0.6);
    renderer.start();
  })
  .catch((e) => console.error(e));
