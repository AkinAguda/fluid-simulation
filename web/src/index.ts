import Renderer from "./renderer";

import("fluid")
  .then(({ FluidConfig }) => {
    const fluidConfig = FluidConfig.new(145, 0.5);
    let renderer = new Renderer(fluidConfig, 0.6);
    renderer.start();
  })
  .catch((e) => console.error(e));
