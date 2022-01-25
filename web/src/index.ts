import Renderer from "./renderer";

import("fluid")
  .then(({ FluidConfig }) => {
    const canvas = document.getElementById("canvas") as HTMLCanvasElement;
    const aspectRatio = canvas.width / canvas.height;
    const fluidConfig = FluidConfig.new(128 * aspectRatio, 128, 0.5);
    let renderer = new Renderer(fluidConfig, 0.6);
    renderer.start();
  })
  .catch((e) => console.error(e));
