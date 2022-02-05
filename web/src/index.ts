import Renderer from "./renderer";
import { resizeCanvasToDisplaySize } from "./utils";

import("fluid")
  .then(({ FluidConfig }) => {
    const canvas = document.getElementById("canvas") as HTMLCanvasElement;
    let renderer = new Renderer(canvas);
    renderer.start();
  })
  .catch((e) => console.error(e));
